#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShapesIteration {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<ShapesEpoch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShapesEpoch {
    #[prost(message, repeated, tag = "1")]
    pub shapes: ::prost::alloc::vec::Vec<Shape>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shape {
    #[prost(enumeration = "shape::Category", tag = "1")]
    pub category: i32,
    #[prost(enumeration = "shape::Kind", tag = "2")]
    pub kind: i32,
    #[prost(uint64, tag = "3")]
    pub object: u64,
    #[prost(uint64, tag = "4")]
    pub begin: u64,
    #[prost(sint64, repeated, tag = "5")]
    pub offsets: ::prost::alloc::vec::Vec<i64>,
}
/// Nested message and enum types in `Shape`.
pub mod shape {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Category {
        Other = 0,
        SimpleVec = 1,
        Array = 2,
        Module = 3,
        DataType = 4,
    }
    impl Category {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Category::Other => "Other",
                Category::SimpleVec => "SimpleVec",
                Category::Array => "Array",
                Category::Module => "Module",
                Category::DataType => "DataType",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Other" => Some(Self::Other),
                "SimpleVec" => Some(Self::SimpleVec),
                "Array" => Some(Self::Array),
                "Module" => Some(Self::Module),
                "DataType" => Some(Self::DataType),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Kind {
        ValArray = 0,
        ObjArray = 1,
        Scalar = 2,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::ValArray => "ValArray",
                Kind::ObjArray => "ObjArray",
                Kind::Scalar => "Scalar",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ValArray" => Some(Self::ValArray),
                "ObjArray" => Some(Self::ObjArray),
                "Scalar" => Some(Self::Scalar),
                _ => None,
            }
        }
    }
}
