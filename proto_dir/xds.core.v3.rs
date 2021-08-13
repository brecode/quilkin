#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextParams {
    #[prost(map="string, string", tag="1")]
    pub params: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLocator {
    #[prost(enumeration="resource_locator::Scheme", tag="1")]
    pub scheme: i32,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub directives: ::prost::alloc::vec::Vec<resource_locator::Directive>,
    #[prost(oneof="resource_locator::ContextParamSpecifier", tags="5")]
    pub context_param_specifier: ::core::option::Option<resource_locator::ContextParamSpecifier>,
}
/// Nested message and enum types in `ResourceLocator`.
pub mod resource_locator {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Directive {
        #[prost(oneof="directive::Directive", tags="1, 2")]
        pub directive: ::core::option::Option<directive::Directive>,
    }
    /// Nested message and enum types in `Directive`.
    pub mod directive {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Directive {
            #[prost(message, tag="1")]
            Alt(super::super::ResourceLocator),
            #[prost(string, tag="2")]
            Entry(::prost::alloc::string::String),
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Scheme {
        Xdstp = 0,
        Http = 1,
        File = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ContextParamSpecifier {
        #[prost(message, tag="5")]
        ExactContext(super::ContextParams),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionEntry {
    #[prost(oneof="collection_entry::ResourceSpecifier", tags="1, 2")]
    pub resource_specifier: ::core::option::Option<collection_entry::ResourceSpecifier>,
}
/// Nested message and enum types in `CollectionEntry`.
pub mod collection_entry {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineEntry {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub version: ::prost::alloc::string::String,
        #[prost(message, optional, tag="3")]
        pub resource: ::core::option::Option<::prost_types::Any>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceSpecifier {
        #[prost(message, tag="1")]
        Locator(super::ResourceLocator),
        #[prost(message, tag="2")]
        InlineEntry(InlineEntry),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceName {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub context: ::core::option::Option<ContextParams>,
}
