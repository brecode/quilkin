#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConcatenateBytes {
    #[prost(message, optional, tag="1")]
    pub on_write: ::core::option::Option<concatenate_bytes::StrategyValue>,
    #[prost(message, optional, tag="2")]
    pub on_read: ::core::option::Option<concatenate_bytes::StrategyValue>,
    #[prost(bytes="vec", tag="3")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `ConcatenateBytes`.
pub mod concatenate_bytes {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StrategyValue {
        #[prost(enumeration="Strategy", tag="1")]
        pub value: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Strategy {
        DoNothing = 0,
        Append = 1,
        Prepend = 2,
    }
}
