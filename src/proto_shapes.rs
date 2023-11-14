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
    #[prost(enumeration = "shape::Kind", tag = "1")]
    pub kind: i32,
    #[prost(uint64, tag = "2")]
    pub object: u64,
    #[prost(sint64, repeated, tag = "3")]
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
