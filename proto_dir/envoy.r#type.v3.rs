#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Percent {
    #[prost(double, tag="1")]
    pub value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FractionalPercent {
    #[prost(uint32, tag="1")]
    pub numerator: u32,
    #[prost(enumeration="fractional_percent::DenominatorType", tag="2")]
    pub denominator: i32,
}
/// Nested message and enum types in `FractionalPercent`.
pub mod fractional_percent {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DenominatorType {
        Hundred = 0,
        TenThousand = 1,
        Million = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemanticVersion {
    #[prost(uint32, tag="1")]
    pub major_number: u32,
    #[prost(uint32, tag="2")]
    pub minor_number: u32,
    #[prost(uint32, tag="3")]
    pub patch: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Range {
    #[prost(int64, tag="1")]
    pub start: i64,
    #[prost(int64, tag="2")]
    pub end: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Range {
    #[prost(int32, tag="1")]
    pub start: i32,
    #[prost(int32, tag="2")]
    pub end: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleRange {
    #[prost(double, tag="1")]
    pub start: f64,
    #[prost(double, tag="2")]
    pub end: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CodecClientType {
    Http1 = 0,
    Http2 = 1,
    Http3 = 2,
}
