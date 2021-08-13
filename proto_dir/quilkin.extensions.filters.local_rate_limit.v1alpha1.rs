#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalRateLimit {
    #[prost(uint64, tag="1")]
    pub max_packets: u64,
    #[prost(message, optional, tag="2")]
    pub period: ::core::option::Option<::prost_types::Duration>,
}
