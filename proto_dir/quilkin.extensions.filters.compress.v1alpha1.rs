#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compress {
    #[prost(message, optional, tag="1")]
    pub mode: ::core::option::Option<compress::ModeValue>,
    #[prost(message, optional, tag="2")]
    pub on_read: ::core::option::Option<compress::ActionValue>,
    #[prost(message, optional, tag="3")]
    pub on_write: ::core::option::Option<compress::ActionValue>,
}
/// Nested message and enum types in `Compress`.
pub mod compress {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModeValue {
        #[prost(enumeration="Mode", tag="1")]
        pub value: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionValue {
        #[prost(enumeration="Action", tag="1")]
        pub value: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        Snappy = 0,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        DoNothing = 0,
        Compress = 1,
        Decompress = 2,
    }
}
