#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitBreakers {
    #[prost(message, repeated, tag="1")]
    pub thresholds: ::prost::alloc::vec::Vec<circuit_breakers::Thresholds>,
}
/// Nested message and enum types in `CircuitBreakers`.
pub mod circuit_breakers {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Thresholds {
        #[prost(enumeration="super::super::super::core::v3::RoutingPriority", tag="1")]
        pub priority: i32,
        #[prost(message, optional, tag="2")]
        pub max_connections: ::core::option::Option<u32>,
        #[prost(message, optional, tag="3")]
        pub max_pending_requests: ::core::option::Option<u32>,
        #[prost(message, optional, tag="4")]
        pub max_requests: ::core::option::Option<u32>,
        #[prost(message, optional, tag="5")]
        pub max_retries: ::core::option::Option<u32>,
        #[prost(message, optional, tag="8")]
        pub retry_budget: ::core::option::Option<thresholds::RetryBudget>,
        #[prost(bool, tag="6")]
        pub track_remaining: bool,
        #[prost(message, optional, tag="7")]
        pub max_connection_pools: ::core::option::Option<u32>,
    }
    /// Nested message and enum types in `Thresholds`.
    pub mod thresholds {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RetryBudget {
            #[prost(message, optional, tag="1")]
            pub budget_percent: ::core::option::Option<super::super::super::super::super::r#type::v3::Percent>,
            #[prost(message, optional, tag="2")]
            pub min_retry_concurrency: ::core::option::Option<u32>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub typed_config: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutlierDetection {
    #[prost(message, optional, tag="1")]
    pub consecutive_5xx: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub base_ejection_time: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="4")]
    pub max_ejection_percent: ::core::option::Option<u32>,
    #[prost(message, optional, tag="5")]
    pub enforcing_consecutive_5xx: ::core::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub enforcing_success_rate: ::core::option::Option<u32>,
    #[prost(message, optional, tag="7")]
    pub success_rate_minimum_hosts: ::core::option::Option<u32>,
    #[prost(message, optional, tag="8")]
    pub success_rate_request_volume: ::core::option::Option<u32>,
    #[prost(message, optional, tag="9")]
    pub success_rate_stdev_factor: ::core::option::Option<u32>,
    #[prost(message, optional, tag="10")]
    pub consecutive_gateway_failure: ::core::option::Option<u32>,
    #[prost(message, optional, tag="11")]
    pub enforcing_consecutive_gateway_failure: ::core::option::Option<u32>,
    #[prost(bool, tag="12")]
    pub split_external_local_origin_errors: bool,
    #[prost(message, optional, tag="13")]
    pub consecutive_local_origin_failure: ::core::option::Option<u32>,
    #[prost(message, optional, tag="14")]
    pub enforcing_consecutive_local_origin_failure: ::core::option::Option<u32>,
    #[prost(message, optional, tag="15")]
    pub enforcing_local_origin_success_rate: ::core::option::Option<u32>,
    #[prost(message, optional, tag="16")]
    pub failure_percentage_threshold: ::core::option::Option<u32>,
    #[prost(message, optional, tag="17")]
    pub enforcing_failure_percentage: ::core::option::Option<u32>,
    #[prost(message, optional, tag="18")]
    pub enforcing_failure_percentage_local_origin: ::core::option::Option<u32>,
    #[prost(message, optional, tag="19")]
    pub failure_percentage_minimum_hosts: ::core::option::Option<u32>,
    #[prost(message, optional, tag="20")]
    pub failure_percentage_request_volume: ::core::option::Option<u32>,
    #[prost(message, optional, tag="21")]
    pub max_ejection_time: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterCollection {
    #[prost(message, optional, tag="1")]
    pub entries: ::core::option::Option<super::super::super::super::xds::core::v3::CollectionEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    #[prost(message, repeated, tag="43")]
    pub transport_socket_matches: ::prost::alloc::vec::Vec<cluster::TransportSocketMatch>,
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="28")]
    pub alt_stat_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub eds_cluster_config: ::core::option::Option<cluster::EdsClusterConfig>,
    #[prost(message, optional, tag="4")]
    pub connect_timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="5")]
    pub per_connection_buffer_limit_bytes: ::core::option::Option<u32>,
    #[prost(enumeration="cluster::LbPolicy", tag="6")]
    pub lb_policy: i32,
    #[prost(message, optional, tag="33")]
    pub load_assignment: ::core::option::Option<super::super::endpoint::v3::ClusterLoadAssignment>,
    #[prost(message, repeated, tag="8")]
    pub health_checks: ::prost::alloc::vec::Vec<super::super::core::v3::HealthCheck>,
    #[prost(message, optional, tag="9")]
    pub max_requests_per_connection: ::core::option::Option<u32>,
    #[prost(message, optional, tag="10")]
    pub circuit_breakers: ::core::option::Option<CircuitBreakers>,
    #[deprecated]
    #[prost(message, optional, tag="46")]
    pub upstream_http_protocol_options: ::core::option::Option<super::super::core::v3::UpstreamHttpProtocolOptions>,
    #[deprecated]
    #[prost(message, optional, tag="29")]
    pub common_http_protocol_options: ::core::option::Option<super::super::core::v3::HttpProtocolOptions>,
    #[deprecated]
    #[prost(message, optional, tag="13")]
    pub http_protocol_options: ::core::option::Option<super::super::core::v3::Http1ProtocolOptions>,
    #[deprecated]
    #[prost(message, optional, tag="14")]
    pub http2_protocol_options: ::core::option::Option<super::super::core::v3::Http2ProtocolOptions>,
    #[prost(map="string, message", tag="36")]
    pub typed_extension_protocol_options: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
    #[prost(message, optional, tag="16")]
    pub dns_refresh_rate: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="44")]
    pub dns_failure_refresh_rate: ::core::option::Option<cluster::RefreshRate>,
    #[prost(bool, tag="39")]
    pub respect_dns_ttl: bool,
    #[prost(enumeration="cluster::DnsLookupFamily", tag="17")]
    pub dns_lookup_family: i32,
    #[prost(message, repeated, tag="18")]
    pub dns_resolvers: ::prost::alloc::vec::Vec<super::super::core::v3::Address>,
    #[prost(bool, tag="45")]
    pub use_tcp_for_dns_lookups: bool,
    #[prost(message, optional, tag="19")]
    pub outlier_detection: ::core::option::Option<OutlierDetection>,
    #[prost(message, optional, tag="20")]
    pub cleanup_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="21")]
    pub upstream_bind_config: ::core::option::Option<super::super::core::v3::BindConfig>,
    #[prost(message, optional, tag="22")]
    pub lb_subset_config: ::core::option::Option<cluster::LbSubsetConfig>,
    #[prost(message, optional, tag="27")]
    pub common_lb_config: ::core::option::Option<cluster::CommonLbConfig>,
    #[prost(message, optional, tag="24")]
    pub transport_socket: ::core::option::Option<super::super::core::v3::TransportSocket>,
    #[prost(message, optional, tag="25")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
    #[deprecated]
    #[prost(enumeration="cluster::ClusterProtocolSelection", tag="26")]
    pub protocol_selection: i32,
    #[prost(message, optional, tag="30")]
    pub upstream_connection_options: ::core::option::Option<UpstreamConnectionOptions>,
    #[prost(bool, tag="31")]
    pub close_connections_on_host_health_failure: bool,
    #[prost(bool, tag="32")]
    pub ignore_health_on_host_removal: bool,
    #[prost(message, repeated, tag="40")]
    pub filters: ::prost::alloc::vec::Vec<Filter>,
    #[prost(message, optional, tag="41")]
    pub load_balancing_policy: ::core::option::Option<LoadBalancingPolicy>,
    #[prost(message, optional, tag="42")]
    pub lrs_server: ::core::option::Option<super::super::core::v3::ConfigSource>,
    #[deprecated]
    #[prost(bool, tag="47")]
    pub track_timeout_budgets: bool,
    #[prost(message, optional, tag="48")]
    pub upstream_config: ::core::option::Option<super::super::core::v3::TypedExtensionConfig>,
    #[prost(message, optional, tag="49")]
    pub track_cluster_stats: ::core::option::Option<TrackClusterStats>,
    #[prost(message, optional, tag="50")]
    pub preconnect_policy: ::core::option::Option<cluster::PreconnectPolicy>,
    #[prost(bool, tag="51")]
    pub connection_pool_per_downstream_connection: bool,
    #[prost(oneof="cluster::ClusterDiscoveryType", tags="2, 38")]
    pub cluster_discovery_type: ::core::option::Option<cluster::ClusterDiscoveryType>,
    #[prost(oneof="cluster::LbConfig", tags="23, 52, 34, 37")]
    pub lb_config: ::core::option::Option<cluster::LbConfig>,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransportSocketMatch {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub r#match: ::core::option::Option<::prost_types::Struct>,
        #[prost(message, optional, tag="3")]
        pub transport_socket: ::core::option::Option<super::super::super::core::v3::TransportSocket>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomClusterType {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub typed_config: ::core::option::Option<::prost_types::Any>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EdsClusterConfig {
        #[prost(message, optional, tag="1")]
        pub eds_config: ::core::option::Option<super::super::super::core::v3::ConfigSource>,
        #[prost(string, tag="2")]
        pub service_name: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LbSubsetConfig {
        #[prost(enumeration="lb_subset_config::LbSubsetFallbackPolicy", tag="1")]
        pub fallback_policy: i32,
        #[prost(message, optional, tag="2")]
        pub default_subset: ::core::option::Option<::prost_types::Struct>,
        #[prost(message, repeated, tag="3")]
        pub subset_selectors: ::prost::alloc::vec::Vec<lb_subset_config::LbSubsetSelector>,
        #[prost(bool, tag="4")]
        pub locality_weight_aware: bool,
        #[prost(bool, tag="5")]
        pub scale_locality_weight: bool,
        #[prost(bool, tag="6")]
        pub panic_mode_any: bool,
        #[prost(bool, tag="7")]
        pub list_as_any: bool,
    }
    /// Nested message and enum types in `LbSubsetConfig`.
    pub mod lb_subset_config {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LbSubsetSelector {
            #[prost(string, repeated, tag="1")]
            pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(bool, tag="4")]
            pub single_host_per_subset: bool,
            #[prost(enumeration="lb_subset_selector::LbSubsetSelectorFallbackPolicy", tag="2")]
            pub fallback_policy: i32,
            #[prost(string, repeated, tag="3")]
            pub fallback_keys_subset: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// Nested message and enum types in `LbSubsetSelector`.
        pub mod lb_subset_selector {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum LbSubsetSelectorFallbackPolicy {
                NotDefined = 0,
                NoFallback = 1,
                AnyEndpoint = 2,
                DefaultSubset = 3,
                KeysSubset = 4,
            }
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum LbSubsetFallbackPolicy {
            NoFallback = 0,
            AnyEndpoint = 1,
            DefaultSubset = 2,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeastRequestLbConfig {
        #[prost(message, optional, tag="1")]
        pub choice_count: ::core::option::Option<u32>,
        #[prost(message, optional, tag="2")]
        pub active_request_bias: ::core::option::Option<super::super::super::core::v3::RuntimeDouble>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RingHashLbConfig {
        #[prost(message, optional, tag="1")]
        pub minimum_ring_size: ::core::option::Option<u64>,
        #[prost(enumeration="ring_hash_lb_config::HashFunction", tag="3")]
        pub hash_function: i32,
        #[prost(message, optional, tag="4")]
        pub maximum_ring_size: ::core::option::Option<u64>,
    }
    /// Nested message and enum types in `RingHashLbConfig`.
    pub mod ring_hash_lb_config {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum HashFunction {
            XxHash = 0,
            MurmurHash2 = 1,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaglevLbConfig {
        #[prost(message, optional, tag="1")]
        pub table_size: ::core::option::Option<u64>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OriginalDstLbConfig {
        #[prost(bool, tag="1")]
        pub use_http_header: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommonLbConfig {
        #[prost(message, optional, tag="1")]
        pub healthy_panic_threshold: ::core::option::Option<super::super::super::super::r#type::v3::Percent>,
        #[prost(message, optional, tag="4")]
        pub update_merge_window: ::core::option::Option<::prost_types::Duration>,
        #[prost(bool, tag="5")]
        pub ignore_new_hosts_until_first_hc: bool,
        #[prost(bool, tag="6")]
        pub close_connections_on_host_set_change: bool,
        #[prost(message, optional, tag="7")]
        pub consistent_hashing_lb_config: ::core::option::Option<common_lb_config::ConsistentHashingLbConfig>,
        #[prost(oneof="common_lb_config::LocalityConfigSpecifier", tags="2, 3")]
        pub locality_config_specifier: ::core::option::Option<common_lb_config::LocalityConfigSpecifier>,
    }
    /// Nested message and enum types in `CommonLbConfig`.
    pub mod common_lb_config {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ZoneAwareLbConfig {
            #[prost(message, optional, tag="1")]
            pub routing_enabled: ::core::option::Option<super::super::super::super::super::r#type::v3::Percent>,
            #[prost(message, optional, tag="2")]
            pub min_cluster_size: ::core::option::Option<u64>,
            #[prost(bool, tag="3")]
            pub fail_traffic_on_panic: bool,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LocalityWeightedLbConfig {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConsistentHashingLbConfig {
            #[prost(bool, tag="1")]
            pub use_hostname_for_hashing: bool,
            #[prost(message, optional, tag="2")]
            pub hash_balance_factor: ::core::option::Option<u32>,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum LocalityConfigSpecifier {
            #[prost(message, tag="2")]
            ZoneAwareLbConfig(ZoneAwareLbConfig),
            #[prost(message, tag="3")]
            LocalityWeightedLbConfig(LocalityWeightedLbConfig),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RefreshRate {
        #[prost(message, optional, tag="1")]
        pub base_interval: ::core::option::Option<::prost_types::Duration>,
        #[prost(message, optional, tag="2")]
        pub max_interval: ::core::option::Option<::prost_types::Duration>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PreconnectPolicy {
        #[prost(message, optional, tag="1")]
        pub per_upstream_preconnect_ratio: ::core::option::Option<f64>,
        #[prost(message, optional, tag="2")]
        pub predictive_preconnect_ratio: ::core::option::Option<f64>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiscoveryType {
        Static = 0,
        StrictDns = 1,
        LogicalDns = 2,
        Eds = 3,
        OriginalDst = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LbPolicy {
        RoundRobin = 0,
        LeastRequest = 1,
        RingHash = 2,
        Random = 3,
        Maglev = 5,
        ClusterProvided = 6,
        LoadBalancingPolicyConfig = 7,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DnsLookupFamily {
        Auto = 0,
        V4Only = 1,
        V6Only = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClusterProtocolSelection {
        UseConfiguredProtocol = 0,
        UseDownstreamProtocol = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterDiscoveryType {
        #[prost(enumeration="DiscoveryType", tag="2")]
        Type(i32),
        #[prost(message, tag="38")]
        ClusterType(CustomClusterType),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LbConfig {
        #[prost(message, tag="23")]
        RingHashLbConfig(RingHashLbConfig),
        #[prost(message, tag="52")]
        MaglevLbConfig(MaglevLbConfig),
        #[prost(message, tag="34")]
        OriginalDstLbConfig(OriginalDstLbConfig),
        #[prost(message, tag="37")]
        LeastRequestLbConfig(LeastRequestLbConfig),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancingPolicy {
    #[prost(message, repeated, tag="1")]
    pub policies: ::prost::alloc::vec::Vec<load_balancing_policy::Policy>,
}
/// Nested message and enum types in `LoadBalancingPolicy`.
pub mod load_balancing_policy {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Policy {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="3")]
        pub typed_config: ::core::option::Option<::prost_types::Any>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamBindConfig {
    #[prost(message, optional, tag="1")]
    pub source_address: ::core::option::Option<super::super::core::v3::Address>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamConnectionOptions {
    #[prost(message, optional, tag="1")]
    pub tcp_keepalive: ::core::option::Option<super::super::core::v3::TcpKeepalive>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackClusterStats {
    #[prost(bool, tag="1")]
    pub timeout_budgets: bool,
    #[prost(bool, tag="2")]
    pub request_response_sizes: bool,
}
