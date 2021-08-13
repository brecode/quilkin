#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureBytes {
    #[prost(message, optional, tag="1")]
    pub strategy: ::core::option::Option<capture_bytes::StrategyValue>,
    #[prost(uint32, tag="2")]
    pub size: u32,
    #[prost(message, optional, tag="3")]
    pub metadata_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub remove: ::core::option::Option<bool>,
}
/// Nested message and enum types in `CaptureBytes`.
pub mod capture_bytes {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StrategyValue {
        #[prost(enumeration="Strategy", tag="1")]
        pub value: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Strategy {
        Prefix = 0,
        Suffix = 1,
    }
}
