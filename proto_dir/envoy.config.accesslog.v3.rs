#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLog {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub filter: ::core::option::Option<AccessLogFilter>,
    #[prost(oneof="access_log::ConfigType", tags="4")]
    pub config_type: ::core::option::Option<access_log::ConfigType>,
}
/// Nested message and enum types in `AccessLog`.
pub mod access_log {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag="4")]
        TypedConfig(::prost_types::Any),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLogFilter {
    #[prost(oneof="access_log_filter::FilterSpecifier", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
    pub filter_specifier: ::core::option::Option<access_log_filter::FilterSpecifier>,
}
/// Nested message and enum types in `AccessLogFilter`.
pub mod access_log_filter {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FilterSpecifier {
        #[prost(message, tag="1")]
        StatusCodeFilter(super::StatusCodeFilter),
        #[prost(message, tag="2")]
        DurationFilter(super::DurationFilter),
        #[prost(message, tag="3")]
        NotHealthCheckFilter(super::NotHealthCheckFilter),
        #[prost(message, tag="4")]
        TraceableFilter(super::TraceableFilter),
        #[prost(message, tag="5")]
        RuntimeFilter(super::RuntimeFilter),
        #[prost(message, tag="6")]
        AndFilter(super::AndFilter),
        #[prost(message, tag="7")]
        OrFilter(super::OrFilter),
        #[prost(message, tag="8")]
        HeaderFilter(super::HeaderFilter),
        #[prost(message, tag="9")]
        ResponseFlagFilter(super::ResponseFlagFilter),
        #[prost(message, tag="10")]
        GrpcStatusFilter(super::GrpcStatusFilter),
        #[prost(message, tag="11")]
        ExtensionFilter(super::ExtensionFilter),
        #[prost(message, tag="12")]
        MetadataFilter(super::MetadataFilter),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComparisonFilter {
    #[prost(enumeration="comparison_filter::Op", tag="1")]
    pub op: i32,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<super::super::core::v3::RuntimeUInt32>,
}
/// Nested message and enum types in `ComparisonFilter`.
pub mod comparison_filter {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Op {
        Eq = 0,
        Ge = 1,
        Le = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusCodeFilter {
    #[prost(message, optional, tag="1")]
    pub comparison: ::core::option::Option<ComparisonFilter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DurationFilter {
    #[prost(message, optional, tag="1")]
    pub comparison: ::core::option::Option<ComparisonFilter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotHealthCheckFilter {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceableFilter {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeFilter {
    #[prost(string, tag="1")]
    pub runtime_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub percent_sampled: ::core::option::Option<super::super::super::r#type::v3::FractionalPercent>,
    #[prost(bool, tag="3")]
    pub use_independent_randomness: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndFilter {
    #[prost(message, repeated, tag="1")]
    pub filters: ::prost::alloc::vec::Vec<AccessLogFilter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrFilter {
    #[prost(message, repeated, tag="2")]
    pub filters: ::prost::alloc::vec::Vec<AccessLogFilter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderFilter {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::super::route::v3::HeaderMatcher>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFlagFilter {
    #[prost(string, repeated, tag="1")]
    pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcStatusFilter {
    #[prost(enumeration="grpc_status_filter::Status", repeated, packed="false", tag="1")]
    pub statuses: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, tag="2")]
    pub exclude: bool,
}
/// Nested message and enum types in `GrpcStatusFilter`.
pub mod grpc_status_filter {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Ok = 0,
        Canceled = 1,
        Unknown = 2,
        InvalidArgument = 3,
        DeadlineExceeded = 4,
        NotFound = 5,
        AlreadyExists = 6,
        PermissionDenied = 7,
        ResourceExhausted = 8,
        FailedPrecondition = 9,
        Aborted = 10,
        OutOfRange = 11,
        Unimplemented = 12,
        Internal = 13,
        Unavailable = 14,
        DataLoss = 15,
        Unauthenticated = 16,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataFilter {
    #[prost(message, optional, tag="1")]
    pub matcher: ::core::option::Option<super::super::super::r#type::matcher::v3::MetadataMatcher>,
    #[prost(message, optional, tag="2")]
    pub match_if_key_not_found: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionFilter {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="extension_filter::ConfigType", tags="3")]
    pub config_type: ::core::option::Option<extension_filter::ConfigType>,
}
/// Nested message and enum types in `ExtensionFilter`.
pub mod extension_filter {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag="3")]
        TypedConfig(::prost_types::Any),
    }
}
