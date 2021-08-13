#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldRules {
    #[prost(message, optional, tag="17")]
    pub message: ::core::option::Option<MessageRules>,
    #[prost(oneof="field_rules::Type", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22")]
    pub r#type: ::core::option::Option<field_rules::Type>,
}
/// Nested message and enum types in `FieldRules`.
pub mod field_rules {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        Float(super::FloatRules),
        #[prost(message, tag="2")]
        Double(super::DoubleRules),
        #[prost(message, tag="3")]
        Int32(super::Int32Rules),
        #[prost(message, tag="4")]
        Int64(super::Int64Rules),
        #[prost(message, tag="5")]
        Uint32(super::UInt32Rules),
        #[prost(message, tag="6")]
        Uint64(super::UInt64Rules),
        #[prost(message, tag="7")]
        Sint32(super::SInt32Rules),
        #[prost(message, tag="8")]
        Sint64(super::SInt64Rules),
        #[prost(message, tag="9")]
        Fixed32(super::Fixed32Rules),
        #[prost(message, tag="10")]
        Fixed64(super::Fixed64Rules),
        #[prost(message, tag="11")]
        Sfixed32(super::SFixed32Rules),
        #[prost(message, tag="12")]
        Sfixed64(super::SFixed64Rules),
        #[prost(message, tag="13")]
        Bool(super::BoolRules),
        #[prost(message, tag="14")]
        String(super::StringRules),
        #[prost(message, tag="15")]
        Bytes(super::BytesRules),
        #[prost(message, tag="16")]
        Enum(super::EnumRules),
        #[prost(message, tag="18")]
        Repeated(::prost::alloc::boxed::Box<super::RepeatedRules>),
        #[prost(message, tag="19")]
        Map(::prost::alloc::boxed::Box<super::MapRules>),
        #[prost(message, tag="20")]
        Any(super::AnyRules),
        #[prost(message, tag="21")]
        Duration(super::DurationRules),
        #[prost(message, tag="22")]
        Timestamp(super::TimestampRules),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatRules {
    #[prost(float, optional, tag="1")]
    pub r#const: ::core::option::Option<f32>,
    #[prost(float, optional, tag="2")]
    pub lt: ::core::option::Option<f32>,
    #[prost(float, optional, tag="3")]
    pub lte: ::core::option::Option<f32>,
    #[prost(float, optional, tag="4")]
    pub gt: ::core::option::Option<f32>,
    #[prost(float, optional, tag="5")]
    pub gte: ::core::option::Option<f32>,
    #[prost(float, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<f32>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleRules {
    #[prost(double, optional, tag="1")]
    pub r#const: ::core::option::Option<f64>,
    #[prost(double, optional, tag="2")]
    pub lt: ::core::option::Option<f64>,
    #[prost(double, optional, tag="3")]
    pub lte: ::core::option::Option<f64>,
    #[prost(double, optional, tag="4")]
    pub gt: ::core::option::Option<f64>,
    #[prost(double, optional, tag="5")]
    pub gte: ::core::option::Option<f64>,
    #[prost(double, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<f64>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Rules {
    #[prost(int32, optional, tag="1")]
    pub r#const: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub lt: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub lte: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub gt: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub gte: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Rules {
    #[prost(int64, optional, tag="1")]
    pub r#const: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub lt: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub lte: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub gt: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="5")]
    pub gte: ::core::option::Option<i64>,
    #[prost(int64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32Rules {
    #[prost(uint32, optional, tag="1")]
    pub r#const: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub lt: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub lte: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub gt: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub gte: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt64Rules {
    #[prost(uint64, optional, tag="1")]
    pub r#const: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub lt: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub lte: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub gt: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub gte: ::core::option::Option<u64>,
    #[prost(uint64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SInt32Rules {
    #[prost(sint32, optional, tag="1")]
    pub r#const: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag="2")]
    pub lt: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag="3")]
    pub lte: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag="4")]
    pub gt: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag="5")]
    pub gte: ::core::option::Option<i32>,
    #[prost(sint32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i32>,
    #[prost(sint32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SInt64Rules {
    #[prost(sint64, optional, tag="1")]
    pub r#const: ::core::option::Option<i64>,
    #[prost(sint64, optional, tag="2")]
    pub lt: ::core::option::Option<i64>,
    #[prost(sint64, optional, tag="3")]
    pub lte: ::core::option::Option<i64>,
    #[prost(sint64, optional, tag="4")]
    pub gt: ::core::option::Option<i64>,
    #[prost(sint64, optional, tag="5")]
    pub gte: ::core::option::Option<i64>,
    #[prost(sint64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i64>,
    #[prost(sint64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fixed32Rules {
    #[prost(fixed32, optional, tag="1")]
    pub r#const: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="2")]
    pub lt: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="3")]
    pub lte: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="4")]
    pub gt: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="5")]
    pub gte: ::core::option::Option<u32>,
    #[prost(fixed32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<u32>,
    #[prost(fixed32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fixed64Rules {
    #[prost(fixed64, optional, tag="1")]
    pub r#const: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag="2")]
    pub lt: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag="3")]
    pub lte: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag="4")]
    pub gt: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag="5")]
    pub gte: ::core::option::Option<u64>,
    #[prost(fixed64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<u64>,
    #[prost(fixed64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SFixed32Rules {
    #[prost(sfixed32, optional, tag="1")]
    pub r#const: ::core::option::Option<i32>,
    #[prost(sfixed32, optional, tag="2")]
    pub lt: ::core::option::Option<i32>,
    #[prost(sfixed32, optional, tag="3")]
    pub lte: ::core::option::Option<i32>,
    #[prost(sfixed32, optional, tag="4")]
    pub gt: ::core::option::Option<i32>,
    #[prost(sfixed32, optional, tag="5")]
    pub gte: ::core::option::Option<i32>,
    #[prost(sfixed32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i32>,
    #[prost(sfixed32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SFixed64Rules {
    #[prost(sfixed64, optional, tag="1")]
    pub r#const: ::core::option::Option<i64>,
    #[prost(sfixed64, optional, tag="2")]
    pub lt: ::core::option::Option<i64>,
    #[prost(sfixed64, optional, tag="3")]
    pub lte: ::core::option::Option<i64>,
    #[prost(sfixed64, optional, tag="4")]
    pub gt: ::core::option::Option<i64>,
    #[prost(sfixed64, optional, tag="5")]
    pub gte: ::core::option::Option<i64>,
    #[prost(sfixed64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i64>,
    #[prost(sfixed64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, optional, tag="8")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolRules {
    #[prost(bool, optional, tag="1")]
    pub r#const: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringRules {
    #[prost(string, optional, tag="1")]
    pub r#const: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="19")]
    pub len: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub min_len: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub max_len: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="20")]
    pub len_bytes: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub min_bytes: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub max_bytes: ::core::option::Option<u64>,
    #[prost(string, optional, tag="6")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub suffix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub contains: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub not_contains: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="10")]
    pub r#in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="11")]
    pub not_in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="25", default="true")]
    pub strict: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="26")]
    pub ignore_empty: ::core::option::Option<bool>,
    #[prost(oneof="string_rules::WellKnown", tags="12, 13, 14, 15, 16, 17, 18, 21, 22, 24")]
    pub well_known: ::core::option::Option<string_rules::WellKnown>,
}
/// Nested message and enum types in `StringRules`.
pub mod string_rules {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WellKnown {
        #[prost(bool, tag="12")]
        Email(bool),
        #[prost(bool, tag="13")]
        Hostname(bool),
        #[prost(bool, tag="14")]
        Ip(bool),
        #[prost(bool, tag="15")]
        Ipv4(bool),
        #[prost(bool, tag="16")]
        Ipv6(bool),
        #[prost(bool, tag="17")]
        Uri(bool),
        #[prost(bool, tag="18")]
        UriRef(bool),
        #[prost(bool, tag="21")]
        Address(bool),
        #[prost(bool, tag="22")]
        Uuid(bool),
        #[prost(enumeration="super::KnownRegex", tag="24")]
        WellKnownRegex(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesRules {
    #[prost(bytes="vec", optional, tag="1")]
    pub r#const: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="13")]
    pub len: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub min_len: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub max_len: ::core::option::Option<u64>,
    #[prost(string, optional, tag="4")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="5")]
    pub prefix: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="6")]
    pub suffix: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub contains: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="8")]
    pub r#in: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="9")]
    pub not_in: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag="14")]
    pub ignore_empty: ::core::option::Option<bool>,
    #[prost(oneof="bytes_rules::WellKnown", tags="10, 11, 12")]
    pub well_known: ::core::option::Option<bytes_rules::WellKnown>,
}
/// Nested message and enum types in `BytesRules`.
pub mod bytes_rules {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WellKnown {
        #[prost(bool, tag="10")]
        Ip(bool),
        #[prost(bool, tag="11")]
        Ipv4(bool),
        #[prost(bool, tag="12")]
        Ipv6(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumRules {
    #[prost(int32, optional, tag="1")]
    pub r#const: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="2")]
    pub defined_only: ::core::option::Option<bool>,
    #[prost(int32, repeated, packed="false", tag="3")]
    pub r#in: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed="false", tag="4")]
    pub not_in: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRules {
    #[prost(bool, optional, tag="1")]
    pub skip: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub required: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepeatedRules {
    #[prost(uint64, optional, tag="1")]
    pub min_items: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub max_items: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="3")]
    pub unique: ::core::option::Option<bool>,
    #[prost(message, optional, boxed, tag="4")]
    pub items: ::core::option::Option<::prost::alloc::boxed::Box<FieldRules>>,
    #[prost(bool, optional, tag="5")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapRules {
    #[prost(uint64, optional, tag="1")]
    pub min_pairs: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub max_pairs: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="3")]
    pub no_sparse: ::core::option::Option<bool>,
    #[prost(message, optional, boxed, tag="4")]
    pub keys: ::core::option::Option<::prost::alloc::boxed::Box<FieldRules>>,
    #[prost(message, optional, boxed, tag="5")]
    pub values: ::core::option::Option<::prost::alloc::boxed::Box<FieldRules>>,
    #[prost(bool, optional, tag="6")]
    pub ignore_empty: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyRules {
    #[prost(bool, optional, tag="1")]
    pub required: ::core::option::Option<bool>,
    #[prost(string, repeated, tag="2")]
    pub r#in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub not_in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DurationRules {
    #[prost(bool, optional, tag="1")]
    pub required: ::core::option::Option<bool>,
    #[prost(message, optional, tag="2")]
    pub r#const: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub lt: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="4")]
    pub lte: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="5")]
    pub gt: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="6")]
    pub gte: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, repeated, tag="7")]
    pub r#in: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    #[prost(message, repeated, tag="8")]
    pub not_in: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampRules {
    #[prost(bool, optional, tag="1")]
    pub required: ::core::option::Option<bool>,
    #[prost(message, optional, tag="2")]
    pub r#const: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub lt: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub lte: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub gt: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub gte: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, optional, tag="7")]
    pub lt_now: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="8")]
    pub gt_now: ::core::option::Option<bool>,
    #[prost(message, optional, tag="9")]
    pub within: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KnownRegex {
    Unknown = 0,
    HttpHeaderName = 1,
    HttpHeaderValue = 2,
}
