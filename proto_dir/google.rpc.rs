#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(int32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub details: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
