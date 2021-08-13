#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataKey {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub path: ::prost::alloc::vec::Vec<metadata_key::PathSegment>,
}
/// Nested message and enum types in `MetadataKey`.
pub mod metadata_key {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PathSegment {
        #[prost(oneof="path_segment::Segment", tags="1")]
        pub segment: ::core::option::Option<path_segment::Segment>,
    }
    /// Nested message and enum types in `PathSegment`.
    pub mod path_segment {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Segment {
            #[prost(string, tag="1")]
            Key(::prost::alloc::string::String),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataKind {
    #[prost(oneof="metadata_kind::Kind", tags="1, 2, 3, 4")]
    pub kind: ::core::option::Option<metadata_kind::Kind>,
}
/// Nested message and enum types in `MetadataKind`.
pub mod metadata_kind {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Route {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cluster {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Host {
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag="1")]
        Request(Request),
        #[prost(message, tag="2")]
        Route(Route),
        #[prost(message, tag="3")]
        Cluster(Cluster),
        #[prost(message, tag="4")]
        Host(Host),
    }
}
