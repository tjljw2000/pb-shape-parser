use std::{env, path::PathBuf};
use glob::glob;
use prost::Message;
use proto_shapes::shape::Kind;
use serde::{Serialize, Deserialize};
use core::fmt;
use std::{fs::File, io::{BufReader, Read, self, Write}, collections::HashMap};
use rayon::prelude::*;

mod proto_shapes;
use crate::proto_shapes::*;

impl Shape {
    pub fn to_string(&self) -> String {
        let str_offsets : String = self.offsets.iter().map( |&offset| offset.to_string() + "," ).collect();
        format!("Myshape {{ kind: {}, object: {}, offsets: {} }}", 
                &self.kind, 
                &self.object, 
                &str_offsets
        )
    }
}

#[derive(Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum PatternKind {
    NoRef,
    ObjArray,
    ShapePattern(Vec<i64>)
}

impl fmt::Display for PatternKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRef => write!(f, "NoRef"),
            Self::ObjArray => write!(f, "ObjArray"),
            Self::ShapePattern(v) => write!(f, "{:?}", v)
        }
    }
}

pub struct ParseReport {
    pub total: i32,
    pub pattern_count: HashMap<PatternKind, (i64, f64)>,
    pub visible: [i32; 11],
    pub invisible: [i32; 11],
}

impl ParseReport {
    pub fn new(shape_iter: ShapesIteration) -> ParseReport {
        let mut counter: HashMap<PatternKind, (i64, f64)> = HashMap::new();
        let mut total = 0;
        let mut visible = [0; 11];
        let mut invisible = [0; 11];
        for epoch in shape_iter.epochs.iter() {
            for shape in epoch.shapes.iter() {
                // println!("{}", shape.to_string());
                match shape.kind() {
                    Kind::ValArray => {
                        counter.entry(PatternKind::NoRef)
                               .and_modify(|(count, _)| *count += 1)
                               .or_insert((1, 0.0));
                        // visible. .map(|&mut count| *count += 1);
                        for v in visible.iter_mut() { *v += 1};
                    },
                    Kind::ObjArray => {
                        counter.entry(PatternKind::ObjArray)
                               .and_modify(|(count, _)| *count += 1)
                               .or_insert((1, 0.0));
                        // visible.map(|&mut count| *count += 1);
                        for v in visible.iter_mut() { *v += 1};
                    },
                    _ => {
                        if shape.offsets.len() == 0 {
                            counter.entry(PatternKind::NoRef)
                                   .and_modify(|(count, _)| *count += 1)
                                   .or_insert((1, 0.0));
                            // visible.map(|&mut count| *count += 1);
                            for v in visible.iter_mut() { *v += 1};
                        } else {
                            counter.entry(PatternKind::ShapePattern(shape.offsets.clone()))
                                   .and_modify(|(count, _)| *count += 1)
                                   .or_insert((1, 0.0));
                            for i in 0..visible.len() {
                                let val: u64 = 1 << (6 + i);
                                // println!("obj:{}, val: {}, offset:{}, obj/val:{}, obj+off/val:{}", shape.object, val, *shape.offsets.last().unwrap(), shape.object / val, (shape.object + u64::try_from(*shape.offsets.last().unwrap()).unwrap() / val ));
                                if (shape.object / val) == ( (shape.object + u64::try_from(*shape.offsets.last().unwrap()).unwrap()) / val  ) { 
                                    visible[i] += 1;
                                } else {
                                    invisible[i] += 1;
                                }
                            }
                        }
                    }
                }
                total += 1;
            }
        }
        
        // normalize
        for (_pattern, (count, percent)) in counter.iter_mut() {
            *percent = *count as f64 / total as f64;
        }

        ParseReport { total, pattern_count : counter, visible, invisible }
    }

    pub fn to_pickle_obj(&self) -> HashMap<String, (f64, i64)> {
        let mut strmap: HashMap<String, (f64, i64)> = HashMap::new();
        for (k, (c, p)) in self.pattern_count.iter() {
            strmap.insert(format!("{}", k), (*p, *c));
        }
        strmap
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let path_match = args[1].to_owned() + "*/shapes.binpb.zst";

    let mut entries: Vec<PathBuf> = vec![];
    for entry in glob(path_match.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => entries.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    // parse_one((*(entries.get(0).unwrap())).clone());
    let _iter = entries.into_par_iter().for_each(|path| parse_one(path) );

    println!("\nAll done!\n\n");
}

fn parse_one(path : PathBuf) {
    // println!("{:?}", path.display());
    let bm_folder = path.parent().unwrap().to_str().unwrap().split('/').last().unwrap();
    let mut bm_split = bm_folder.split('.');
    let bm_name = bm_split.next().unwrap();
    let bm_iter_n = bm_split.last().unwrap();
    let task_name = format!("{}-{}", bm_name, bm_iter_n);
    println!("{}: folder_name: {}", task_name, bm_folder);

    let binpb_file = File::open(path.as_os_str()).unwrap();
    let binpb_bufreader = BufReader::new(binpb_file);

    println!("{}: Reading zstd file", task_name);
    let mut pb_bytes: Vec<u8> = Vec::new();
    let mut zstd_file = zstd::Decoder::new(binpb_bufreader).unwrap();
    let result = zstd_file.read_to_end( &mut pb_bytes).unwrap();
    println!("{}: Read {} KB from zstd file", task_name, result / 1024);

    println!("{}: Parsing protobuf", task_name);
    let pb_iter = ShapesIteration::decode(&*pb_bytes).unwrap();

    println!("{}: Doing satistic", task_name);
    let report = ParseReport::new(pb_iter);

    println!("{}: Serialising report", task_name);
    let report_tuple = (task_name.clone(), report.to_pickle_obj(), report.visible, report.invisible);
    let report_buf = serde_pickle::to_vec(&report_tuple, serde_pickle::SerOptions::new()).unwrap();

    println!("{}: Writing report", task_name);
    io::stdout().flush().unwrap();
    let mut report_file = File::create(format!("{}.pkl", task_name)).unwrap();
    report_file.write_all(&report_buf).unwrap();

    println!("{}: Done", task_name);
}