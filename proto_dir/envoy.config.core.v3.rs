#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketOption {
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub level: i64,
    #[prost(int64, tag="3")]
    pub name: i64,
    #[prost(enumeration="socket_option::SocketState", tag="6")]
    pub state: i32,
    #[prost(oneof="socket_option::Value", tags="4, 5")]
    pub value: ::core::option::Option<socket_option::Value>,
}
/// Nested message and enum types in `SocketOption`.
pub mod socket_option {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SocketState {
        StatePrebind = 0,
        StateBound = 1,
        StateListening = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(int64, tag="4")]
        IntValue(i64),
        #[prost(bytes, tag="5")]
        BufValue(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pipe {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub mode: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvoyInternalAddress {
    #[prost(oneof="envoy_internal_address::AddressNameSpecifier", tags="1")]
    pub address_name_specifier: ::core::option::Option<envoy_internal_address::AddressNameSpecifier>,
}
/// Nested message and enum types in `EnvoyInternalAddress`.
pub mod envoy_internal_address {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AddressNameSpecifier {
        #[prost(string, tag="1")]
        ServerListenerName(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketAddress {
    #[prost(enumeration="socket_address::Protocol", tag="1")]
    pub protocol: i32,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub resolver_name: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub ipv4_compat: bool,
    #[prost(oneof="socket_address::PortSpecifier", tags="3, 4")]
    pub port_specifier: ::core::option::Option<socket_address::PortSpecifier>,
}
/// Nested message and enum types in `SocketAddress`.
pub mod socket_address {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Protocol {
        Tcp = 0,
        Udp = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PortSpecifier {
        #[prost(uint32, tag="3")]
        PortValue(u32),
        #[prost(string, tag="4")]
        NamedPort(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpKeepalive {
    #[prost(message, optional, tag="1")]
    pub keepalive_probes: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub keepalive_time: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub keepalive_interval: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindConfig {
    #[prost(message, optional, tag="1")]
    pub source_address: ::core::option::Option<SocketAddress>,
    #[prost(message, optional, tag="2")]
    pub freebind: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="3")]
    pub socket_options: ::prost::alloc::vec::Vec<SocketOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(oneof="address::Address", tags="1, 2, 3")]
    pub address: ::core::option::Option<address::Address>,
}
/// Nested message and enum types in `Address`.
pub mod address {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Address {
        #[prost(message, tag="1")]
        SocketAddress(super::SocketAddress),
        #[prost(message, tag="2")]
        Pipe(super::Pipe),
        #[prost(message, tag="3")]
        EnvoyInternalAddress(super::EnvoyInternalAddress),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CidrRange {
    #[prost(string, tag="1")]
    pub address_prefix: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub prefix_len: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackoffStrategy {
    #[prost(message, optional, tag="1")]
    pub base_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="2")]
    pub max_interval: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpUri {
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(oneof="http_uri::HttpUpstreamType", tags="2")]
    pub http_upstream_type: ::core::option::Option<http_uri::HttpUpstreamType>,
}
/// Nested message and enum types in `HttpUri`.
pub mod http_uri {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HttpUpstreamType {
        #[prost(string, tag="2")]
        Cluster(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Locality {
    #[prost(string, tag="1")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sub_zone: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildVersion {
    #[prost(message, optional, tag="1")]
    pub version: ::core::option::Option<super::super::super::r#type::v3::SemanticVersion>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extension {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub type_descriptor: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub version: ::core::option::Option<BuildVersion>,
    #[prost(bool, tag="5")]
    pub disabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub cluster: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
    #[prost(map="string, message", tag="12")]
    pub dynamic_parameters: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::super::xds::core::v3::ContextParams>,
    #[prost(message, optional, tag="4")]
    pub locality: ::core::option::Option<Locality>,
    #[prost(string, tag="6")]
    pub user_agent_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="9")]
    pub extensions: ::prost::alloc::vec::Vec<Extension>,
    #[prost(string, repeated, tag="10")]
    pub client_features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
    #[prost(message, repeated, tag="11")]
    pub listening_addresses: ::prost::alloc::vec::Vec<Address>,
    #[prost(oneof="node::UserAgentVersionType", tags="7, 8")]
    pub user_agent_version_type: ::core::option::Option<node::UserAgentVersionType>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UserAgentVersionType {
        #[prost(string, tag="7")]
        UserAgentVersion(::prost::alloc::string::String),
        #[prost(message, tag="8")]
        UserAgentBuildVersion(super::BuildVersion),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(map="string, message", tag="1")]
    pub filter_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeUInt32 {
    #[prost(uint32, tag="2")]
    pub default_value: u32,
    #[prost(string, tag="3")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimePercent {
    #[prost(message, optional, tag="1")]
    pub default_value: ::core::option::Option<super::super::super::r#type::v3::Percent>,
    #[prost(string, tag="2")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeDouble {
    #[prost(double, tag="1")]
    pub default_value: f64,
    #[prost(string, tag="2")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeFeatureFlag {
    #[prost(message, optional, tag="1")]
    pub default_value: ::core::option::Option<bool>,
    #[prost(string, tag="2")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderValue {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderValueOption {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<HeaderValue>,
    #[prost(message, optional, tag="2")]
    pub append: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMap {
    #[prost(message, repeated, tag="1")]
    pub headers: ::prost::alloc::vec::Vec<HeaderValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchedDirectory {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSource {
    #[prost(oneof="data_source::Specifier", tags="1, 2, 3")]
    pub specifier: ::core::option::Option<data_source::Specifier>,
}
/// Nested message and enum types in `DataSource`.
pub mod data_source {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Specifier {
        #[prost(string, tag="1")]
        Filename(::prost::alloc::string::String),
        #[prost(bytes, tag="2")]
        InlineBytes(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag="3")]
        InlineString(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryPolicy {
    #[prost(message, optional, tag="1")]
    pub retry_back_off: ::core::option::Option<BackoffStrategy>,
    #[prost(message, optional, tag="2")]
    pub num_retries: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteDataSource {
    #[prost(message, optional, tag="1")]
    pub http_uri: ::core::option::Option<HttpUri>,
    #[prost(string, tag="2")]
    pub sha256: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub retry_policy: ::core::option::Option<RetryPolicy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncDataSource {
    #[prost(oneof="async_data_source::Specifier", tags="1, 2")]
    pub specifier: ::core::option::Option<async_data_source::Specifier>,
}
/// Nested message and enum types in `AsyncDataSource`.
pub mod async_data_source {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Specifier {
        #[prost(message, tag="1")]
        Local(super::DataSource),
        #[prost(message, tag="2")]
        Remote(super::RemoteDataSource),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransportSocket {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="transport_socket::ConfigType", tags="3")]
    pub config_type: ::core::option::Option<transport_socket::ConfigType>,
}
/// Nested message and enum types in `TransportSocket`.
pub mod transport_socket {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag="3")]
        TypedConfig(::prost_types::Any),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeFractionalPercent {
    #[prost(message, optional, tag="1")]
    pub default_value: ::core::option::Option<super::super::super::r#type::v3::FractionalPercent>,
    #[prost(string, tag="2")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlPlane {
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoutingPriority {
    Default = 0,
    High = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RequestMethod {
    MethodUnspecified = 0,
    Get = 1,
    Head = 2,
    Post = 3,
    Put = 4,
    Delete = 5,
    Connect = 6,
    Options = 7,
    Trace = 8,
    Patch = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrafficDirection {
    Unspecified = 0,
    Inbound = 1,
    Outbound = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcService {
    #[prost(message, optional, tag="3")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, repeated, tag="5")]
    pub initial_metadata: ::prost::alloc::vec::Vec<HeaderValue>,
    #[prost(oneof="grpc_service::TargetSpecifier", tags="1, 2")]
    pub target_specifier: ::core::option::Option<grpc_service::TargetSpecifier>,
}
/// Nested message and enum types in `GrpcService`.
pub mod grpc_service {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnvoyGrpc {
        #[prost(string, tag="1")]
        pub cluster_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub authority: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoogleGrpc {
        #[prost(string, tag="1")]
        pub target_uri: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub channel_credentials: ::core::option::Option<google_grpc::ChannelCredentials>,
        #[prost(message, repeated, tag="3")]
        pub call_credentials: ::prost::alloc::vec::Vec<google_grpc::CallCredentials>,
        #[prost(string, tag="4")]
        pub stat_prefix: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub credentials_factory_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="6")]
        pub config: ::core::option::Option<::prost_types::Struct>,
        #[prost(message, optional, tag="7")]
        pub per_stream_buffer_limit_bytes: ::core::option::Option<u32>,
        #[prost(message, optional, tag="8")]
        pub channel_args: ::core::option::Option<google_grpc::ChannelArgs>,
    }
    /// Nested message and enum types in `GoogleGrpc`.
    pub mod google_grpc {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SslCredentials {
            #[prost(message, optional, tag="1")]
            pub root_certs: ::core::option::Option<super::super::DataSource>,
            #[prost(message, optional, tag="2")]
            pub private_key: ::core::option::Option<super::super::DataSource>,
            #[prost(message, optional, tag="3")]
            pub cert_chain: ::core::option::Option<super::super::DataSource>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GoogleLocalCredentials {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ChannelCredentials {
            #[prost(oneof="channel_credentials::CredentialSpecifier", tags="1, 2, 3")]
            pub credential_specifier: ::core::option::Option<channel_credentials::CredentialSpecifier>,
        }
        /// Nested message and enum types in `ChannelCredentials`.
        pub mod channel_credentials {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum CredentialSpecifier {
                #[prost(message, tag="1")]
                SslCredentials(super::SslCredentials),
                #[prost(message, tag="2")]
                GoogleDefault(()),
                #[prost(message, tag="3")]
                LocalCredentials(super::GoogleLocalCredentials),
            }
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CallCredentials {
            #[prost(oneof="call_credentials::CredentialSpecifier", tags="1, 2, 3, 4, 5, 6, 7")]
            pub credential_specifier: ::core::option::Option<call_credentials::CredentialSpecifier>,
        }
        /// Nested message and enum types in `CallCredentials`.
        pub mod call_credentials {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ServiceAccountJwtAccessCredentials {
                #[prost(string, tag="1")]
                pub json_key: ::prost::alloc::string::String,
                #[prost(uint64, tag="2")]
                pub token_lifetime_seconds: u64,
            }
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GoogleIamCredentials {
                #[prost(string, tag="1")]
                pub authorization_token: ::prost::alloc::string::String,
                #[prost(string, tag="2")]
                pub authority_selector: ::prost::alloc::string::String,
            }
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct MetadataCredentialsFromPlugin {
                #[prost(string, tag="1")]
                pub name: ::prost::alloc::string::String,
                #[prost(oneof="metadata_credentials_from_plugin::ConfigType", tags="3")]
                pub config_type: ::core::option::Option<metadata_credentials_from_plugin::ConfigType>,
            }
            /// Nested message and enum types in `MetadataCredentialsFromPlugin`.
            pub mod metadata_credentials_from_plugin {
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum ConfigType {
                    #[prost(message, tag="3")]
                    TypedConfig(::prost_types::Any),
                }
            }
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct StsService {
                #[prost(string, tag="1")]
                pub token_exchange_service_uri: ::prost::alloc::string::String,
                #[prost(string, tag="2")]
                pub resource: ::prost::alloc::string::String,
                #[prost(string, tag="3")]
                pub audience: ::prost::alloc::string::String,
                #[prost(string, tag="4")]
                pub scope: ::prost::alloc::string::String,
                #[prost(string, tag="5")]
                pub requested_token_type: ::prost::alloc::string::String,
                #[prost(string, tag="6")]
                pub subject_token_path: ::prost::alloc::string::String,
                #[prost(string, tag="7")]
                pub subject_token_type: ::prost::alloc::string::String,
                #[prost(string, tag="8")]
                pub actor_token_path: ::prost::alloc::string::String,
                #[prost(string, tag="9")]
                pub actor_token_type: ::prost::alloc::string::String,
            }
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum CredentialSpecifier {
                #[prost(string, tag="1")]
                AccessToken(::prost::alloc::string::String),
                #[prost(message, tag="2")]
                GoogleComputeEngine(()),
                #[prost(string, tag="3")]
                GoogleRefreshToken(::prost::alloc::string::String),
                #[prost(message, tag="4")]
                ServiceAccountJwtAccess(ServiceAccountJwtAccessCredentials),
                #[prost(message, tag="5")]
                GoogleIam(GoogleIamCredentials),
                #[prost(message, tag="6")]
                FromPlugin(MetadataCredentialsFromPlugin),
                #[prost(message, tag="7")]
                StsService(StsService),
            }
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ChannelArgs {
            #[prost(map="string, message", tag="1")]
            pub args: ::std::collections::HashMap<::prost::alloc::string::String, channel_args::Value>,
        }
        /// Nested message and enum types in `ChannelArgs`.
        pub mod channel_args {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Value {
                #[prost(oneof="value::ValueSpecifier", tags="1, 2")]
                pub value_specifier: ::core::option::Option<value::ValueSpecifier>,
            }
            /// Nested message and enum types in `Value`.
            pub mod value {
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum ValueSpecifier {
                    #[prost(string, tag="1")]
                    StringValue(::prost::alloc::string::String),
                    #[prost(int64, tag="2")]
                    IntValue(i64),
                }
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetSpecifier {
        #[prost(message, tag="1")]
        EnvoyGrpc(EnvoyGrpc),
        #[prost(message, tag="2")]
        GoogleGrpc(GoogleGrpc),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiConfigSource {
    #[prost(enumeration="api_config_source::ApiType", tag="1")]
    pub api_type: i32,
    #[prost(enumeration="ApiVersion", tag="8")]
    pub transport_api_version: i32,
    #[prost(string, repeated, tag="2")]
    pub cluster_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub grpc_services: ::prost::alloc::vec::Vec<GrpcService>,
    #[prost(message, optional, tag="3")]
    pub refresh_delay: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="5")]
    pub request_timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="6")]
    pub rate_limit_settings: ::core::option::Option<RateLimitSettings>,
    #[prost(bool, tag="7")]
    pub set_node_on_first_message_only: bool,
}
/// Nested message and enum types in `ApiConfigSource`.
pub mod api_config_source {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ApiType {
        DeprecatedAndUnavailableDoNotUse = 0,
        Rest = 1,
        Grpc = 2,
        DeltaGrpc = 3,
        AggregatedGrpc = 5,
        AggregatedDeltaGrpc = 6,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedConfigSource {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfConfigSource {
    #[prost(enumeration="ApiVersion", tag="1")]
    pub transport_api_version: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitSettings {
    #[prost(message, optional, tag="1")]
    pub max_tokens: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub fill_rate: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSource {
    #[prost(message, repeated, tag="7")]
    pub authorities: ::prost::alloc::vec::Vec<super::super::super::super::xds::core::v3::Authority>,
    #[prost(message, optional, tag="4")]
    pub initial_fetch_timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(enumeration="ApiVersion", tag="6")]
    pub resource_api_version: i32,
    #[prost(oneof="config_source::ConfigSourceSpecifier", tags="1, 2, 3, 5")]
    pub config_source_specifier: ::core::option::Option<config_source::ConfigSourceSpecifier>,
}
/// Nested message and enum types in `ConfigSource`.
pub mod config_source {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigSourceSpecifier {
        #[prost(string, tag="1")]
        Path(::prost::alloc::string::String),
        #[prost(message, tag="2")]
        ApiConfigSource(super::ApiConfigSource),
        #[prost(message, tag="3")]
        Ads(super::AggregatedConfigSource),
        #[prost(message, tag="5")]
        Self_(super::SelfConfigSource),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiVersion {
    Auto = 0,
    V2 = 1,
    V3 = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedExtensionConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub typed_config: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionConfigSource {
    #[prost(message, optional, tag="1")]
    pub config_source: ::core::option::Option<ConfigSource>,
    #[prost(message, optional, tag="2")]
    pub default_config: ::core::option::Option<::prost_types::Any>,
    #[prost(bool, tag="3")]
    pub apply_default_config_without_warming: bool,
    #[prost(string, repeated, tag="4")]
    pub type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocolConfig {
    #[prost(enumeration="proxy_protocol_config::Version", tag="1")]
    pub version: i32,
}
/// Nested message and enum types in `ProxyProtocolConfig`.
pub mod proxy_protocol_config {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Version {
        V1 = 0,
        V2 = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventServiceConfig {
    #[prost(oneof="event_service_config::ConfigSourceSpecifier", tags="1")]
    pub config_source_specifier: ::core::option::Option<event_service_config::ConfigSourceSpecifier>,
}
/// Nested message and enum types in `EventServiceConfig`.
pub mod event_service_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigSourceSpecifier {
        #[prost(message, tag="1")]
        GrpcService(super::GrpcService),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheck {
    #[prost(message, optional, tag="1")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="2")]
    pub interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="20")]
    pub initial_jitter: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub interval_jitter: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint32, tag="18")]
    pub interval_jitter_percent: u32,
    #[prost(message, optional, tag="4")]
    pub unhealthy_threshold: ::core::option::Option<u32>,
    #[prost(message, optional, tag="5")]
    pub healthy_threshold: ::core::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub alt_port: ::core::option::Option<u32>,
    #[prost(message, optional, tag="7")]
    pub reuse_connection: ::core::option::Option<bool>,
    #[prost(message, optional, tag="12")]
    pub no_traffic_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="24")]
    pub no_traffic_healthy_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="14")]
    pub unhealthy_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="15")]
    pub unhealthy_edge_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="16")]
    pub healthy_edge_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(string, tag="17")]
    pub event_log_path: ::prost::alloc::string::String,
    #[prost(message, optional, tag="22")]
    pub event_service: ::core::option::Option<EventServiceConfig>,
    #[prost(bool, tag="19")]
    pub always_log_health_check_failures: bool,
    #[prost(message, optional, tag="21")]
    pub tls_options: ::core::option::Option<health_check::TlsOptions>,
    #[prost(message, optional, tag="23")]
    pub transport_socket_match_criteria: ::core::option::Option<::prost_types::Struct>,
    #[prost(oneof="health_check::HealthChecker", tags="8, 9, 11, 13")]
    pub health_checker: ::core::option::Option<health_check::HealthChecker>,
}
/// Nested message and enum types in `HealthCheck`.
pub mod health_check {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(oneof="payload::Payload", tags="1, 2")]
        pub payload: ::core::option::Option<payload::Payload>,
    }
    /// Nested message and enum types in `Payload`.
    pub mod payload {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Payload {
            #[prost(string, tag="1")]
            Text(::prost::alloc::string::String),
            #[prost(bytes, tag="2")]
            Binary(::prost::alloc::vec::Vec<u8>),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HttpHealthCheck {
        #[prost(string, tag="1")]
        pub host: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub path: ::prost::alloc::string::String,
        #[prost(message, optional, tag="3")]
        pub send: ::core::option::Option<Payload>,
        #[prost(message, optional, tag="4")]
        pub receive: ::core::option::Option<Payload>,
        #[prost(message, repeated, tag="6")]
        pub request_headers_to_add: ::prost::alloc::vec::Vec<super::HeaderValueOption>,
        #[prost(string, repeated, tag="8")]
        pub request_headers_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag="9")]
        pub expected_statuses: ::prost::alloc::vec::Vec<super::super::super::super::r#type::v3::Int64Range>,
        #[prost(enumeration="super::super::super::super::r#type::v3::CodecClientType", tag="10")]
        pub codec_client_type: i32,
        #[prost(message, optional, tag="11")]
        pub service_name_matcher: ::core::option::Option<super::super::super::super::r#type::matcher::v3::StringMatcher>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TcpHealthCheck {
        #[prost(message, optional, tag="1")]
        pub send: ::core::option::Option<Payload>,
        #[prost(message, repeated, tag="2")]
        pub receive: ::prost::alloc::vec::Vec<Payload>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RedisHealthCheck {
        #[prost(string, tag="1")]
        pub key: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GrpcHealthCheck {
        #[prost(string, tag="1")]
        pub service_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub authority: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomHealthCheck {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof="custom_health_check::ConfigType", tags="3")]
        pub config_type: ::core::option::Option<custom_health_check::ConfigType>,
    }
    /// Nested message and enum types in `CustomHealthCheck`.
    pub mod custom_health_check {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag="3")]
            TypedConfig(::prost_types::Any),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TlsOptions {
        #[prost(string, repeated, tag="1")]
        pub alpn_protocols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HealthChecker {
        #[prost(message, tag="8")]
        HttpHealthCheck(HttpHealthCheck),
        #[prost(message, tag="9")]
        TcpHealthCheck(TcpHealthCheck),
        #[prost(message, tag="11")]
        GrpcHealthCheck(GrpcHealthCheck),
        #[prost(message, tag="13")]
        CustomHealthCheck(CustomHealthCheck),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HealthStatus {
    Unknown = 0,
    Healthy = 1,
    Unhealthy = 2,
    Draining = 3,
    Timeout = 4,
    Degraded = 5,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpProtocolOptions {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamHttpProtocolOptions {
    #[prost(bool, tag="1")]
    pub auto_sni: bool,
    #[prost(bool, tag="2")]
    pub auto_san_validation: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpProtocolOptions {
    #[prost(message, optional, tag="1")]
    pub idle_timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub max_connection_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="2")]
    pub max_headers_count: ::core::option::Option<u32>,
    #[prost(message, optional, tag="4")]
    pub max_stream_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(enumeration="http_protocol_options::HeadersWithUnderscoresAction", tag="5")]
    pub headers_with_underscores_action: i32,
}
/// Nested message and enum types in `HttpProtocolOptions`.
pub mod http_protocol_options {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HeadersWithUnderscoresAction {
        Allow = 0,
        RejectRequest = 1,
        DropHeader = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http1ProtocolOptions {
    #[prost(message, optional, tag="1")]
    pub allow_absolute_url: ::core::option::Option<bool>,
    #[prost(bool, tag="2")]
    pub accept_http_10: bool,
    #[prost(string, tag="3")]
    pub default_host_for_http_10: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub header_key_format: ::core::option::Option<http1_protocol_options::HeaderKeyFormat>,
    #[prost(bool, tag="5")]
    pub enable_trailers: bool,
    #[prost(bool, tag="6")]
    pub allow_chunked_length: bool,
    #[prost(message, optional, tag="7")]
    pub override_stream_error_on_invalid_http_message: ::core::option::Option<bool>,
}
/// Nested message and enum types in `Http1ProtocolOptions`.
pub mod http1_protocol_options {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderKeyFormat {
        #[prost(oneof="header_key_format::HeaderFormat", tags="1")]
        pub header_format: ::core::option::Option<header_key_format::HeaderFormat>,
    }
    /// Nested message and enum types in `HeaderKeyFormat`.
    pub mod header_key_format {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ProperCaseWords {
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum HeaderFormat {
            #[prost(message, tag="1")]
            ProperCaseWords(ProperCaseWords),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeepaliveSettings {
    #[prost(message, optional, tag="1")]
    pub interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="2")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub interval_jitter: ::core::option::Option<super::super::super::r#type::v3::Percent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http2ProtocolOptions {
    #[prost(message, optional, tag="1")]
    pub hpack_table_size: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub max_concurrent_streams: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub initial_stream_window_size: ::core::option::Option<u32>,
    #[prost(message, optional, tag="4")]
    pub initial_connection_window_size: ::core::option::Option<u32>,
    #[prost(bool, tag="5")]
    pub allow_connect: bool,
    #[prost(bool, tag="6")]
    pub allow_metadata: bool,
    #[prost(message, optional, tag="7")]
    pub max_outbound_frames: ::core::option::Option<u32>,
    #[prost(message, optional, tag="8")]
    pub max_outbound_control_frames: ::core::option::Option<u32>,
    #[prost(message, optional, tag="9")]
    pub max_consecutive_inbound_frames_with_empty_payload: ::core::option::Option<u32>,
    #[prost(message, optional, tag="10")]
    pub max_inbound_priority_frames_per_stream: ::core::option::Option<u32>,
    #[prost(message, optional, tag="11")]
    pub max_inbound_window_update_frames_per_data_frame_sent: ::core::option::Option<u32>,
    #[deprecated]
    #[prost(bool, tag="12")]
    pub stream_error_on_invalid_http_messaging: bool,
    #[prost(message, optional, tag="14")]
    pub override_stream_error_on_invalid_http_message: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="13")]
    pub custom_settings_parameters: ::prost::alloc::vec::Vec<http2_protocol_options::SettingsParameter>,
    #[prost(message, optional, tag="15")]
    pub connection_keepalive: ::core::option::Option<KeepaliveSettings>,
}
/// Nested message and enum types in `Http2ProtocolOptions`.
pub mod http2_protocol_options {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SettingsParameter {
        #[prost(message, optional, tag="1")]
        pub identifier: ::core::option::Option<u32>,
        #[prost(message, optional, tag="2")]
        pub value: ::core::option::Option<u32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcProtocolOptions {
    #[prost(message, optional, tag="1")]
    pub http2_protocol_options: ::core::option::Option<Http2ProtocolOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http3ProtocolOptions {
}
