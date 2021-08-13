#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    #[prost(message, optional, tag="1")]
    pub address: ::core::option::Option<super::super::core::v3::Address>,
    #[prost(message, optional, tag="2")]
    pub health_check_config: ::core::option::Option<endpoint::HealthCheckConfig>,
    #[prost(string, tag="3")]
    pub hostname: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Endpoint`.
pub mod endpoint {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HealthCheckConfig {
        #[prost(uint32, tag="1")]
        pub port_value: u32,
        #[prost(string, tag="2")]
        pub hostname: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbEndpoint {
    #[prost(enumeration="super::super::core::v3::HealthStatus", tag="2")]
    pub health_status: i32,
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
    #[prost(message, optional, tag="4")]
    pub load_balancing_weight: ::core::option::Option<u32>,
    #[prost(oneof="lb_endpoint::HostIdentifier", tags="1, 5")]
    pub host_identifier: ::core::option::Option<lb_endpoint::HostIdentifier>,
}
/// Nested message and enum types in `LbEndpoint`.
pub mod lb_endpoint {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HostIdentifier {
        #[prost(message, tag="1")]
        Endpoint(super::Endpoint),
        #[prost(string, tag="5")]
        EndpointName(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalityLbEndpoints {
    #[prost(message, optional, tag="1")]
    pub locality: ::core::option::Option<super::super::core::v3::Locality>,
    #[prost(message, repeated, tag="2")]
    pub lb_endpoints: ::prost::alloc::vec::Vec<LbEndpoint>,
    #[prost(message, optional, tag="3")]
    pub load_balancing_weight: ::core::option::Option<u32>,
    #[prost(uint32, tag="5")]
    pub priority: u32,
    #[prost(message, optional, tag="6")]
    pub proximity: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterLoadAssignment {
    #[prost(string, tag="1")]
    pub cluster_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub endpoints: ::prost::alloc::vec::Vec<LocalityLbEndpoints>,
    #[prost(map="string, message", tag="5")]
    pub named_endpoints: ::std::collections::HashMap<::prost::alloc::string::String, Endpoint>,
    #[prost(message, optional, tag="4")]
    pub policy: ::core::option::Option<cluster_load_assignment::Policy>,
}
/// Nested message and enum types in `ClusterLoadAssignment`.
pub mod cluster_load_assignment {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Policy {
        #[prost(message, repeated, tag="2")]
        pub drop_overloads: ::prost::alloc::vec::Vec<policy::DropOverload>,
        #[prost(message, optional, tag="3")]
        pub overprovisioning_factor: ::core::option::Option<u32>,
        #[prost(message, optional, tag="4")]
        pub endpoint_stale_after: ::core::option::Option<::prost_types::Duration>,
    }
    /// Nested message and enum types in `Policy`.
    pub mod policy {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DropOverload {
            #[prost(string, tag="1")]
            pub category: ::prost::alloc::string::String,
            #[prost(message, optional, tag="2")]
            pub drop_percentage: ::core::option::Option<super::super::super::super::super::r#type::v3::FractionalPercent>,
        }
    }
}
