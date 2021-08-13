#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTag {
    #[prost(string, tag="1")]
    pub tag: ::prost::alloc::string::String,
    #[prost(oneof="custom_tag::Type", tags="2, 3, 4, 5")]
    pub r#type: ::core::option::Option<custom_tag::Type>,
}
/// Nested message and enum types in `CustomTag`.
pub mod custom_tag {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Literal {
        #[prost(string, tag="1")]
        pub value: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Environment {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub default_value: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Header {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub default_value: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metadata {
        #[prost(message, optional, tag="1")]
        pub kind: ::core::option::Option<super::super::super::metadata::v3::MetadataKind>,
        #[prost(message, optional, tag="2")]
        pub metadata_key: ::core::option::Option<super::super::super::metadata::v3::MetadataKey>,
        #[prost(string, tag="3")]
        pub default_value: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="2")]
        Literal(Literal),
        #[prost(message, tag="3")]
        Environment(Environment),
        #[prost(message, tag="4")]
        RequestHeader(Header),
        #[prost(message, tag="5")]
        Metadata(Metadata),
    }
}
