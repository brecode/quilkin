#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualHost {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    #[prost(enumeration="virtual_host::TlsRequirementType", tag="4")]
    pub require_tls: i32,
    #[prost(message, repeated, tag="5")]
    pub virtual_clusters: ::prost::alloc::vec::Vec<VirtualCluster>,
    #[prost(message, repeated, tag="6")]
    pub rate_limits: ::prost::alloc::vec::Vec<RateLimit>,
    #[prost(message, repeated, tag="7")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<super::super::core::v3::HeaderValueOption>,
    #[prost(string, repeated, tag="13")]
    pub request_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="10")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<super::super::core::v3::HeaderValueOption>,
    #[prost(string, repeated, tag="11")]
    pub response_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub cors: ::core::option::Option<CorsPolicy>,
    #[prost(map="string, message", tag="15")]
    pub typed_per_filter_config: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
    #[prost(bool, tag="14")]
    pub include_request_attempt_count: bool,
    #[prost(bool, tag="19")]
    pub include_attempt_count_in_response: bool,
    #[prost(message, optional, tag="16")]
    pub retry_policy: ::core::option::Option<RetryPolicy>,
    #[prost(message, optional, tag="20")]
    pub retry_policy_typed_config: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag="17")]
    pub hedge_policy: ::core::option::Option<HedgePolicy>,
    #[prost(message, optional, tag="18")]
    pub per_request_buffer_limit_bytes: ::core::option::Option<u32>,
}
/// Nested message and enum types in `VirtualHost`.
pub mod virtual_host {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TlsRequirementType {
        None = 0,
        ExternalOnly = 1,
        All = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterAction {
    #[prost(message, optional, tag="1")]
    pub action: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    #[prost(string, tag="14")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="1")]
    pub r#match: ::core::option::Option<RouteMatch>,
    #[prost(message, optional, tag="4")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
    #[prost(message, optional, tag="5")]
    pub decorator: ::core::option::Option<Decorator>,
    #[prost(map="string, message", tag="13")]
    pub typed_per_filter_config: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
    #[prost(message, repeated, tag="9")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<super::super::core::v3::HeaderValueOption>,
    #[prost(string, repeated, tag="12")]
    pub request_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="10")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<super::super::core::v3::HeaderValueOption>,
    #[prost(string, repeated, tag="11")]
    pub response_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="15")]
    pub tracing: ::core::option::Option<Tracing>,
    #[prost(message, optional, tag="16")]
    pub per_request_buffer_limit_bytes: ::core::option::Option<u32>,
    #[prost(oneof="route::Action", tags="2, 3, 7, 17")]
    pub action: ::core::option::Option<route::Action>,
}
/// Nested message and enum types in `Route`.
pub mod route {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag="2")]
        Route(super::RouteAction),
        #[prost(message, tag="3")]
        Redirect(super::RedirectAction),
        #[prost(message, tag="7")]
        DirectResponse(super::DirectResponseAction),
        #[prost(message, tag="17")]
        FilterAction(super::FilterAction),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedCluster {
    #[prost(message, repeated, tag="1")]
    pub clusters: ::prost::alloc::vec::Vec<weighted_cluster::ClusterWeight>,
    #[prost(message, optional, tag="3")]
    pub total_weight: ::core::option::Option<u32>,
    #[prost(string, tag="2")]
    pub runtime_key_prefix: ::prost::alloc::string::String,
}
/// Nested message and enum types in `WeightedCluster`.
pub mod weighted_cluster {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterWeight {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub weight: ::core::option::Option<u32>,
        #[prost(message, optional, tag="3")]
        pub metadata_match: ::core::option::Option<super::super::super::core::v3::Metadata>,
        #[prost(message, repeated, tag="4")]
        pub request_headers_to_add: ::prost::alloc::vec::Vec<super::super::super::core::v3::HeaderValueOption>,
        #[prost(string, repeated, tag="9")]
        pub request_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag="5")]
        pub response_headers_to_add: ::prost::alloc::vec::Vec<super::super::super::core::v3::HeaderValueOption>,
        #[prost(string, repeated, tag="6")]
        pub response_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(map="string, message", tag="10")]
        pub typed_per_filter_config: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatch {
    #[prost(message, optional, tag="4")]
    pub case_sensitive: ::core::option::Option<bool>,
    #[prost(message, optional, tag="9")]
    pub runtime_fraction: ::core::option::Option<super::super::core::v3::RuntimeFractionalPercent>,
    #[prost(message, repeated, tag="6")]
    pub headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    #[prost(message, repeated, tag="7")]
    pub query_parameters: ::prost::alloc::vec::Vec<QueryParameterMatcher>,
    #[prost(message, optional, tag="8")]
    pub grpc: ::core::option::Option<route_match::GrpcRouteMatchOptions>,
    #[prost(message, optional, tag="11")]
    pub tls_context: ::core::option::Option<route_match::TlsContextMatchOptions>,
    #[prost(oneof="route_match::PathSpecifier", tags="1, 2, 10, 12")]
    pub path_specifier: ::core::option::Option<route_match::PathSpecifier>,
}
/// Nested message and enum types in `RouteMatch`.
pub mod route_match {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GrpcRouteMatchOptions {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TlsContextMatchOptions {
        #[prost(message, optional, tag="1")]
        pub presented: ::core::option::Option<bool>,
        #[prost(message, optional, tag="2")]
        pub validated: ::core::option::Option<bool>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnectMatcher {
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PathSpecifier {
        #[prost(string, tag="1")]
        Prefix(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        Path(::prost::alloc::string::String),
        #[prost(message, tag="10")]
        SafeRegex(super::super::super::super::r#type::matcher::v3::RegexMatcher),
        #[prost(message, tag="12")]
        ConnectMatcher(ConnectMatcher),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CorsPolicy {
    #[prost(message, repeated, tag="11")]
    pub allow_origin_string_match: ::prost::alloc::vec::Vec<super::super::super::r#type::matcher::v3::StringMatcher>,
    #[prost(string, tag="2")]
    pub allow_methods: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub allow_headers: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub expose_headers: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub max_age: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub allow_credentials: ::core::option::Option<bool>,
    #[prost(message, optional, tag="10")]
    pub shadow_enabled: ::core::option::Option<super::super::core::v3::RuntimeFractionalPercent>,
    #[prost(oneof="cors_policy::EnabledSpecifier", tags="9")]
    pub enabled_specifier: ::core::option::Option<cors_policy::EnabledSpecifier>,
}
/// Nested message and enum types in `CorsPolicy`.
pub mod cors_policy {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnabledSpecifier {
        #[prost(message, tag="9")]
        FilterEnabled(super::super::super::core::v3::RuntimeFractionalPercent),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteAction {
    #[prost(enumeration="route_action::ClusterNotFoundResponseCode", tag="20")]
    pub cluster_not_found_response_code: i32,
    #[prost(message, optional, tag="4")]
    pub metadata_match: ::core::option::Option<super::super::core::v3::Metadata>,
    #[prost(string, tag="5")]
    pub prefix_rewrite: ::prost::alloc::string::String,
    #[prost(message, optional, tag="32")]
    pub regex_rewrite: ::core::option::Option<super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute>,
    #[prost(message, optional, tag="8")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="24")]
    pub idle_timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="9")]
    pub retry_policy: ::core::option::Option<RetryPolicy>,
    #[prost(message, optional, tag="33")]
    pub retry_policy_typed_config: ::core::option::Option<::prost_types::Any>,
    #[prost(message, repeated, tag="30")]
    pub request_mirror_policies: ::prost::alloc::vec::Vec<route_action::RequestMirrorPolicy>,
    #[prost(enumeration="super::super::core::v3::RoutingPriority", tag="11")]
    pub priority: i32,
    #[prost(message, repeated, tag="13")]
    pub rate_limits: ::prost::alloc::vec::Vec<RateLimit>,
    #[deprecated]
    #[prost(message, optional, tag="14")]
    pub include_vh_rate_limits: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="15")]
    pub hash_policy: ::prost::alloc::vec::Vec<route_action::HashPolicy>,
    #[prost(message, optional, tag="17")]
    pub cors: ::core::option::Option<CorsPolicy>,
    #[deprecated]
    #[prost(message, optional, tag="23")]
    pub max_grpc_timeout: ::core::option::Option<::prost_types::Duration>,
    #[deprecated]
    #[prost(message, optional, tag="28")]
    pub grpc_timeout_offset: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, repeated, tag="25")]
    pub upgrade_configs: ::prost::alloc::vec::Vec<route_action::UpgradeConfig>,
    #[prost(message, optional, tag="34")]
    pub internal_redirect_policy: ::core::option::Option<InternalRedirectPolicy>,
    #[deprecated]
    #[prost(enumeration="route_action::InternalRedirectAction", tag="26")]
    pub internal_redirect_action: i32,
    #[deprecated]
    #[prost(message, optional, tag="31")]
    pub max_internal_redirects: ::core::option::Option<u32>,
    #[prost(message, optional, tag="27")]
    pub hedge_policy: ::core::option::Option<HedgePolicy>,
    #[prost(message, optional, tag="36")]
    pub max_stream_duration: ::core::option::Option<route_action::MaxStreamDuration>,
    #[prost(oneof="route_action::ClusterSpecifier", tags="1, 2, 3")]
    pub cluster_specifier: ::core::option::Option<route_action::ClusterSpecifier>,
    #[prost(oneof="route_action::HostRewriteSpecifier", tags="6, 7, 29, 35")]
    pub host_rewrite_specifier: ::core::option::Option<route_action::HostRewriteSpecifier>,
}
/// Nested message and enum types in `RouteAction`.
pub mod route_action {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestMirrorPolicy {
        #[prost(string, tag="1")]
        pub cluster: ::prost::alloc::string::String,
        #[prost(message, optional, tag="3")]
        pub runtime_fraction: ::core::option::Option<super::super::super::core::v3::RuntimeFractionalPercent>,
        #[prost(message, optional, tag="4")]
        pub trace_sampled: ::core::option::Option<bool>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HashPolicy {
        #[prost(bool, tag="4")]
        pub terminal: bool,
        #[prost(oneof="hash_policy::PolicySpecifier", tags="1, 2, 3, 5, 6")]
        pub policy_specifier: ::core::option::Option<hash_policy::PolicySpecifier>,
    }
    /// Nested message and enum types in `HashPolicy`.
    pub mod hash_policy {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Header {
            #[prost(string, tag="1")]
            pub header_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag="2")]
            pub regex_rewrite: ::core::option::Option<super::super::super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Cookie {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
            #[prost(message, optional, tag="2")]
            pub ttl: ::core::option::Option<::prost_types::Duration>,
            #[prost(string, tag="3")]
            pub path: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConnectionProperties {
            #[prost(bool, tag="1")]
            pub source_ip: bool,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QueryParameter {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FilterState {
            #[prost(string, tag="1")]
            pub key: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PolicySpecifier {
            #[prost(message, tag="1")]
            Header(Header),
            #[prost(message, tag="2")]
            Cookie(Cookie),
            #[prost(message, tag="3")]
            ConnectionProperties(ConnectionProperties),
            #[prost(message, tag="5")]
            QueryParameter(QueryParameter),
            #[prost(message, tag="6")]
            FilterState(FilterState),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpgradeConfig {
        #[prost(string, tag="1")]
        pub upgrade_type: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub enabled: ::core::option::Option<bool>,
        #[prost(message, optional, tag="3")]
        pub connect_config: ::core::option::Option<upgrade_config::ConnectConfig>,
    }
    /// Nested message and enum types in `UpgradeConfig`.
    pub mod upgrade_config {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConnectConfig {
            #[prost(message, optional, tag="1")]
            pub proxy_protocol_config: ::core::option::Option<super::super::super::super::core::v3::ProxyProtocolConfig>,
            #[prost(bool, tag="2")]
            pub allow_post: bool,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaxStreamDuration {
        #[prost(message, optional, tag="1")]
        pub max_stream_duration: ::core::option::Option<::prost_types::Duration>,
        #[prost(message, optional, tag="2")]
        pub grpc_timeout_header_max: ::core::option::Option<::prost_types::Duration>,
        #[prost(message, optional, tag="3")]
        pub grpc_timeout_header_offset: ::core::option::Option<::prost_types::Duration>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClusterNotFoundResponseCode {
        ServiceUnavailable = 0,
        NotFound = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InternalRedirectAction {
        PassThroughInternalRedirect = 0,
        HandleInternalRedirect = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterSpecifier {
        #[prost(string, tag="1")]
        Cluster(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        ClusterHeader(::prost::alloc::string::String),
        #[prost(message, tag="3")]
        WeightedClusters(super::WeightedCluster),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HostRewriteSpecifier {
        #[prost(string, tag="6")]
        HostRewriteLiteral(::prost::alloc::string::String),
        #[prost(message, tag="7")]
        AutoHostRewrite(bool),
        #[prost(string, tag="29")]
        HostRewriteHeader(::prost::alloc::string::String),
        #[prost(message, tag="35")]
        HostRewritePathRegex(super::super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryPolicy {
    #[prost(string, tag="1")]
    pub retry_on: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub num_retries: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub per_try_timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="4")]
    pub retry_priority: ::core::option::Option<retry_policy::RetryPriority>,
    #[prost(message, repeated, tag="5")]
    pub retry_host_predicate: ::prost::alloc::vec::Vec<retry_policy::RetryHostPredicate>,
    #[prost(int64, tag="6")]
    pub host_selection_retry_max_attempts: i64,
    #[prost(uint32, repeated, tag="7")]
    pub retriable_status_codes: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag="8")]
    pub retry_back_off: ::core::option::Option<retry_policy::RetryBackOff>,
    #[prost(message, optional, tag="11")]
    pub rate_limited_retry_back_off: ::core::option::Option<retry_policy::RateLimitedRetryBackOff>,
    #[prost(message, repeated, tag="9")]
    pub retriable_headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    #[prost(message, repeated, tag="10")]
    pub retriable_request_headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
}
/// Nested message and enum types in `RetryPolicy`.
pub mod retry_policy {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryPriority {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof="retry_priority::ConfigType", tags="3")]
        pub config_type: ::core::option::Option<retry_priority::ConfigType>,
    }
    /// Nested message and enum types in `RetryPriority`.
    pub mod retry_priority {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag="3")]
            TypedConfig(::prost_types::Any),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryHostPredicate {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof="retry_host_predicate::ConfigType", tags="3")]
        pub config_type: ::core::option::Option<retry_host_predicate::ConfigType>,
    }
    /// Nested message and enum types in `RetryHostPredicate`.
    pub mod retry_host_predicate {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag="3")]
            TypedConfig(::prost_types::Any),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryBackOff {
        #[prost(message, optional, tag="1")]
        pub base_interval: ::core::option::Option<::prost_types::Duration>,
        #[prost(message, optional, tag="2")]
        pub max_interval: ::core::option::Option<::prost_types::Duration>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResetHeader {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(enumeration="ResetHeaderFormat", tag="2")]
        pub format: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RateLimitedRetryBackOff {
        #[prost(message, repeated, tag="1")]
        pub reset_headers: ::prost::alloc::vec::Vec<ResetHeader>,
        #[prost(message, optional, tag="2")]
        pub max_interval: ::core::option::Option<::prost_types::Duration>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResetHeaderFormat {
        Seconds = 0,
        UnixTimestamp = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HedgePolicy {
    #[prost(message, optional, tag="1")]
    pub initial_requests: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub additional_request_chance: ::core::option::Option<super::super::super::r#type::v3::FractionalPercent>,
    #[prost(bool, tag="3")]
    pub hedge_on_per_try_timeout: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedirectAction {
    #[prost(string, tag="1")]
    pub host_redirect: ::prost::alloc::string::String,
    #[prost(uint32, tag="8")]
    pub port_redirect: u32,
    #[prost(enumeration="redirect_action::RedirectResponseCode", tag="3")]
    pub response_code: i32,
    #[prost(bool, tag="6")]
    pub strip_query: bool,
    #[prost(oneof="redirect_action::SchemeRewriteSpecifier", tags="4, 7")]
    pub scheme_rewrite_specifier: ::core::option::Option<redirect_action::SchemeRewriteSpecifier>,
    #[prost(oneof="redirect_action::PathRewriteSpecifier", tags="2, 5, 9")]
    pub path_rewrite_specifier: ::core::option::Option<redirect_action::PathRewriteSpecifier>,
}
/// Nested message and enum types in `RedirectAction`.
pub mod redirect_action {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RedirectResponseCode {
        MovedPermanently = 0,
        Found = 1,
        SeeOther = 2,
        TemporaryRedirect = 3,
        PermanentRedirect = 4,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SchemeRewriteSpecifier {
        #[prost(bool, tag="4")]
        HttpsRedirect(bool),
        #[prost(string, tag="7")]
        SchemeRedirect(::prost::alloc::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PathRewriteSpecifier {
        #[prost(string, tag="2")]
        PathRedirect(::prost::alloc::string::String),
        #[prost(string, tag="5")]
        PrefixRewrite(::prost::alloc::string::String),
        #[prost(message, tag="9")]
        RegexRewrite(super::super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectResponseAction {
    #[prost(uint32, tag="1")]
    pub status: u32,
    #[prost(message, optional, tag="2")]
    pub body: ::core::option::Option<super::super::core::v3::DataSource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decorator {
    #[prost(string, tag="1")]
    pub operation: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub propagate: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tracing {
    #[prost(message, optional, tag="1")]
    pub client_sampling: ::core::option::Option<super::super::super::r#type::v3::FractionalPercent>,
    #[prost(message, optional, tag="2")]
    pub random_sampling: ::core::option::Option<super::super::super::r#type::v3::FractionalPercent>,
    #[prost(message, optional, tag="3")]
    pub overall_sampling: ::core::option::Option<super::super::super::r#type::v3::FractionalPercent>,
    #[prost(message, repeated, tag="4")]
    pub custom_tags: ::prost::alloc::vec::Vec<super::super::super::r#type::tracing::v3::CustomTag>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualCluster {
    #[prost(message, repeated, tag="4")]
    pub headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
    #[prost(message, optional, tag="1")]
    pub stage: ::core::option::Option<u32>,
    #[prost(string, tag="2")]
    pub disable_key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub actions: ::prost::alloc::vec::Vec<rate_limit::Action>,
    #[prost(message, optional, tag="4")]
    pub limit: ::core::option::Option<rate_limit::Override>,
}
/// Nested message and enum types in `RateLimit`.
pub mod rate_limit {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        #[prost(oneof="action::ActionSpecifier", tags="1, 2, 3, 4, 5, 6, 7, 8, 9")]
        pub action_specifier: ::core::option::Option<action::ActionSpecifier>,
    }
    /// Nested message and enum types in `Action`.
    pub mod action {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SourceCluster {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DestinationCluster {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RequestHeaders {
            #[prost(string, tag="1")]
            pub header_name: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub descriptor_key: ::prost::alloc::string::String,
            #[prost(bool, tag="3")]
            pub skip_if_absent: bool,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RemoteAddress {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GenericKey {
            #[prost(string, tag="1")]
            pub descriptor_value: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub descriptor_key: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HeaderValueMatch {
            #[prost(string, tag="1")]
            pub descriptor_value: ::prost::alloc::string::String,
            #[prost(message, optional, tag="2")]
            pub expect_match: ::core::option::Option<bool>,
            #[prost(message, repeated, tag="3")]
            pub headers: ::prost::alloc::vec::Vec<super::super::HeaderMatcher>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DynamicMetaData {
            #[prost(string, tag="1")]
            pub descriptor_key: ::prost::alloc::string::String,
            #[prost(message, optional, tag="2")]
            pub metadata_key: ::core::option::Option<super::super::super::super::super::r#type::metadata::v3::MetadataKey>,
            #[prost(string, tag="3")]
            pub default_value: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetaData {
            #[prost(string, tag="1")]
            pub descriptor_key: ::prost::alloc::string::String,
            #[prost(message, optional, tag="2")]
            pub metadata_key: ::core::option::Option<super::super::super::super::super::r#type::metadata::v3::MetadataKey>,
            #[prost(string, tag="3")]
            pub default_value: ::prost::alloc::string::String,
            #[prost(enumeration="meta_data::Source", tag="4")]
            pub source: i32,
        }
        /// Nested message and enum types in `MetaData`.
        pub mod meta_data {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum Source {
                Dynamic = 0,
                RouteEntry = 1,
            }
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ActionSpecifier {
            #[prost(message, tag="1")]
            SourceCluster(SourceCluster),
            #[prost(message, tag="2")]
            DestinationCluster(DestinationCluster),
            #[prost(message, tag="3")]
            RequestHeaders(RequestHeaders),
            #[prost(message, tag="4")]
            RemoteAddress(RemoteAddress),
            #[prost(message, tag="5")]
            GenericKey(GenericKey),
            #[prost(message, tag="6")]
            HeaderValueMatch(HeaderValueMatch),
            #[prost(message, tag="7")]
            DynamicMetadata(DynamicMetaData),
            #[prost(message, tag="8")]
            Metadata(MetaData),
            #[prost(message, tag="9")]
            Extension(super::super::super::super::core::v3::TypedExtensionConfig),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Override {
        #[prost(oneof="r#override::OverrideSpecifier", tags="1")]
        pub override_specifier: ::core::option::Option<r#override::OverrideSpecifier>,
    }
    /// Nested message and enum types in `Override`.
    pub mod r#override {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DynamicMetadata {
            #[prost(message, optional, tag="1")]
            pub metadata_key: ::core::option::Option<super::super::super::super::super::r#type::metadata::v3::MetadataKey>,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OverrideSpecifier {
            #[prost(message, tag="1")]
            DynamicMetadata(DynamicMetadata),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMatcher {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="8")]
    pub invert_match: bool,
    #[prost(oneof="header_matcher::HeaderMatchSpecifier", tags="4, 11, 6, 7, 9, 10, 12")]
    pub header_match_specifier: ::core::option::Option<header_matcher::HeaderMatchSpecifier>,
}
/// Nested message and enum types in `HeaderMatcher`.
pub mod header_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HeaderMatchSpecifier {
        #[prost(string, tag="4")]
        ExactMatch(::prost::alloc::string::String),
        #[prost(message, tag="11")]
        SafeRegexMatch(super::super::super::super::r#type::matcher::v3::RegexMatcher),
        #[prost(message, tag="6")]
        RangeMatch(super::super::super::super::r#type::v3::Int64Range),
        #[prost(bool, tag="7")]
        PresentMatch(bool),
        #[prost(string, tag="9")]
        PrefixMatch(::prost::alloc::string::String),
        #[prost(string, tag="10")]
        SuffixMatch(::prost::alloc::string::String),
        #[prost(string, tag="12")]
        ContainsMatch(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParameterMatcher {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="query_parameter_matcher::QueryParameterMatchSpecifier", tags="5, 6")]
    pub query_parameter_match_specifier: ::core::option::Option<query_parameter_matcher::QueryParameterMatchSpecifier>,
}
/// Nested message and enum types in `QueryParameterMatcher`.
pub mod query_parameter_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum QueryParameterMatchSpecifier {
        #[prost(message, tag="5")]
        StringMatch(super::super::super::super::r#type::matcher::v3::StringMatcher),
        #[prost(bool, tag="6")]
        PresentMatch(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalRedirectPolicy {
    #[prost(message, optional, tag="1")]
    pub max_internal_redirects: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub redirect_response_codes: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag="3")]
    pub predicates: ::prost::alloc::vec::Vec<super::super::core::v3::TypedExtensionConfig>,
    #[prost(bool, tag="4")]
    pub allow_cross_scheme_redirect: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<::prost_types::Any>,
    #[prost(bool, tag="2")]
    pub is_optional: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteConfiguration {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub virtual_hosts: ::prost::alloc::vec::Vec<VirtualHost>,
    #[prost(message, optional, tag="9")]
    pub vhds: ::core::option::Option<Vhds>,
    #[prost(string, repeated, tag="3")]
    pub internal_only_headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<super::super::core::v3::HeaderValueOption>,
    #[prost(string, repeated, tag="5")]
    pub response_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="6")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<super::super::core::v3::HeaderValueOption>,
    #[prost(string, repeated, tag="8")]
    pub request_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="10")]
    pub most_specific_header_mutations_wins: bool,
    #[prost(message, optional, tag="7")]
    pub validate_clusters: ::core::option::Option<bool>,
    #[prost(message, optional, tag="11")]
    pub max_direct_response_body_size_bytes: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vhds {
    #[prost(message, optional, tag="1")]
    pub config_source: ::core::option::Option<super::super::core::v3::ConfigSource>,
}
