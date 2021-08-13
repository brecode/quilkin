#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusAnnotation {
    #[prost(bool, tag="1")]
    pub work_in_progress: bool,
    #[prost(enumeration="PackageVersionStatus", tag="2")]
    pub package_version_status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PackageVersionStatus {
    Unknown = 0,
    Frozen = 1,
    Active = 2,
    NextMajorVersionCandidate = 3,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersioningAnnotation {
    #[prost(string, tag="1")]
    pub previous_message_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateAnnotation {
    #[prost(string, tag="1")]
    pub rename: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldMigrateAnnotation {
    #[prost(string, tag="1")]
    pub rename: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub oneof_promotion: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMigrateAnnotation {
    #[prost(string, tag="2")]
    pub move_to_package: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldSecurityAnnotation {
    #[prost(bool, tag="1")]
    pub configure_for_untrusted_downstream: bool,
    #[prost(bool, tag="2")]
    pub configure_for_untrusted_upstream: bool,
}
