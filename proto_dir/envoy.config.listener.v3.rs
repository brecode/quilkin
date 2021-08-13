#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiListener {
    #[prost(message, optional, tag="1")]
    pub api_listener: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="filter::ConfigType", tags="4, 5")]
    pub config_type: ::core::option::Option<filter::ConfigType>,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag="4")]
        TypedConfig(::prost_types::Any),
        #[prost(message, tag="5")]
        ConfigDiscovery(super::super::super::core::v3::ExtensionConfigSource),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterChainMatch {
    #[prost(message, optional, tag="8")]
    pub destination_port: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="3")]
    pub prefix_ranges: ::prost::alloc::vec::Vec<super::super::core::v3::CidrRange>,
    #[prost(string, tag="4")]
    pub address_suffix: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub suffix_len: ::core::option::Option<u32>,
    #[prost(enumeration="filter_chain_match::ConnectionSourceType", tag="12")]
    pub source_type: i32,
    #[prost(message, repeated, tag="6")]
    pub source_prefix_ranges: ::prost::alloc::vec::Vec<super::super::core::v3::CidrRange>,
    #[prost(uint32, repeated, packed="false", tag="7")]
    pub source_ports: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, repeated, tag="11")]
    pub server_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="9")]
    pub transport_protocol: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="10")]
    pub application_protocols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `FilterChainMatch`.
pub mod filter_chain_match {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConnectionSourceType {
        Any = 0,
        SameIpOrLoopback = 1,
        External = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterChain {
    #[prost(message, optional, tag="1")]
    pub filter_chain_match: ::core::option::Option<FilterChainMatch>,
    #[prost(message, repeated, tag="3")]
    pub filters: ::prost::alloc::vec::Vec<Filter>,
    #[deprecated]
    #[prost(message, optional, tag="4")]
    pub use_proxy_proto: ::core::option::Option<bool>,
    #[prost(message, optional, tag="5")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
    #[prost(message, optional, tag="6")]
    pub transport_socket: ::core::option::Option<super::super::core::v3::TransportSocket>,
    #[prost(message, optional, tag="9")]
    pub transport_socket_connect_timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub on_demand_configuration: ::core::option::Option<filter_chain::OnDemandConfiguration>,
}
/// Nested message and enum types in `FilterChain`.
pub mod filter_chain {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OnDemandConfiguration {
        #[prost(message, optional, tag="1")]
        pub rebuild_timeout: ::core::option::Option<::prost_types::Duration>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenerFilterChainMatchPredicate {
    #[prost(oneof="listener_filter_chain_match_predicate::Rule", tags="1, 2, 3, 4, 5")]
    pub rule: ::core::option::Option<listener_filter_chain_match_predicate::Rule>,
}
/// Nested message and enum types in `ListenerFilterChainMatchPredicate`.
pub mod listener_filter_chain_match_predicate {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchSet {
        #[prost(message, repeated, tag="1")]
        pub rules: ::prost::alloc::vec::Vec<super::ListenerFilterChainMatchPredicate>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        #[prost(message, tag="1")]
        OrMatch(MatchSet),
        #[prost(message, tag="2")]
        AndMatch(MatchSet),
        #[prost(message, tag="3")]
        NotMatch(::prost::alloc::boxed::Box<super::ListenerFilterChainMatchPredicate>),
        #[prost(bool, tag="4")]
        AnyMatch(bool),
        #[prost(message, tag="5")]
        DestinationPortRange(super::super::super::super::r#type::v3::Int32Range),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenerFilter {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub filter_disabled: ::core::option::Option<ListenerFilterChainMatchPredicate>,
    #[prost(oneof="listener_filter::ConfigType", tags="3")]
    pub config_type: ::core::option::Option<listener_filter::ConfigType>,
}
/// Nested message and enum types in `ListenerFilter`.
pub mod listener_filter {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag="3")]
        TypedConfig(::prost_types::Any),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpListenerConfig {
    #[prost(string, tag="1")]
    pub udp_listener_name: ::prost::alloc::string::String,
    #[prost(oneof="udp_listener_config::ConfigType", tags="3")]
    pub config_type: ::core::option::Option<udp_listener_config::ConfigType>,
}
/// Nested message and enum types in `UdpListenerConfig`.
pub mod udp_listener_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag="3")]
        TypedConfig(::prost_types::Any),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveRawUdpListenerConfig {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenerCollection {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<super::super::super::super::xds::core::v3::CollectionEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Listener {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub address: ::core::option::Option<super::super::core::v3::Address>,
    #[prost(message, repeated, tag="3")]
    pub filter_chains: ::prost::alloc::vec::Vec<FilterChain>,
    #[prost(message, optional, tag="4")]
    pub use_original_dst: ::core::option::Option<bool>,
    #[prost(message, optional, tag="25")]
    pub default_filter_chain: ::core::option::Option<FilterChain>,
    #[prost(message, optional, tag="5")]
    pub per_connection_buffer_limit_bytes: ::core::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
    #[deprecated]
    #[prost(message, optional, tag="7")]
    pub deprecated_v1: ::core::option::Option<listener::DeprecatedV1>,
    #[prost(enumeration="listener::DrainType", tag="8")]
    pub drain_type: i32,
    #[prost(message, repeated, tag="9")]
    pub listener_filters: ::prost::alloc::vec::Vec<ListenerFilter>,
    #[prost(message, optional, tag="15")]
    pub listener_filters_timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(bool, tag="17")]
    pub continue_on_listener_filters_timeout: bool,
    #[prost(message, optional, tag="10")]
    pub transparent: ::core::option::Option<bool>,
    #[prost(message, optional, tag="11")]
    pub freebind: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="13")]
    pub socket_options: ::prost::alloc::vec::Vec<super::super::core::v3::SocketOption>,
    #[prost(message, optional, tag="12")]
    pub tcp_fast_open_queue_length: ::core::option::Option<u32>,
    #[prost(enumeration="super::super::core::v3::TrafficDirection", tag="16")]
    pub traffic_direction: i32,
    #[prost(message, optional, tag="18")]
    pub udp_listener_config: ::core::option::Option<UdpListenerConfig>,
    #[prost(message, optional, tag="19")]
    pub api_listener: ::core::option::Option<ApiListener>,
    #[prost(message, optional, tag="20")]
    pub connection_balance_config: ::core::option::Option<listener::ConnectionBalanceConfig>,
    #[prost(bool, tag="21")]
    pub reuse_port: bool,
    #[prost(message, repeated, tag="22")]
    pub access_log: ::prost::alloc::vec::Vec<super::super::accesslog::v3::AccessLog>,
    #[prost(message, optional, tag="23")]
    pub udp_writer_config: ::core::option::Option<super::super::core::v3::TypedExtensionConfig>,
    #[prost(message, optional, tag="24")]
    pub tcp_backlog_size: ::core::option::Option<u32>,
    #[prost(message, optional, tag="26")]
    pub bind_to_port: ::core::option::Option<bool>,
    #[prost(oneof="listener::ListenerSpecifier", tags="27")]
    pub listener_specifier: ::core::option::Option<listener::ListenerSpecifier>,
}
/// Nested message and enum types in `Listener`.
pub mod listener {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeprecatedV1 {
        #[prost(message, optional, tag="1")]
        pub bind_to_port: ::core::option::Option<bool>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnectionBalanceConfig {
        #[prost(oneof="connection_balance_config::BalanceType", tags="1")]
        pub balance_type: ::core::option::Option<connection_balance_config::BalanceType>,
    }
    /// Nested message and enum types in `ConnectionBalanceConfig`.
    pub mod connection_balance_config {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExactBalance {
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum BalanceType {
            #[prost(message, tag="1")]
            ExactBalance(ExactBalance),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InternalListenerConfig {
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DrainType {
        Default = 0,
        ModifyOnly = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ListenerSpecifier {
        #[prost(message, tag="27")]
        InternalListener(InternalListenerConfig),
    }
}
