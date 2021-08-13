#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryRequest {
    #[prost(string, tag="1")]
    pub version_info: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub node: ::core::option::Option<super::super::super::config::core::v3::Node>,
    #[prost(string, repeated, tag="3")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub response_nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub error_detail: ::core::option::Option<super::super::super::super::google::rpc::Status>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryResponse {
    #[prost(string, tag="1")]
    pub version_info: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub resources: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(bool, tag="3")]
    pub canary: bool,
    #[prost(string, tag="4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub control_plane: ::core::option::Option<super::super::super::config::core::v3::ControlPlane>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryRequest {
    #[prost(message, optional, tag="1")]
    pub node: ::core::option::Option<super::super::super::config::core::v3::Node>,
    #[prost(string, tag="2")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub resource_names_subscribe: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub resource_names_unsubscribe: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, string", tag="5")]
    pub initial_resource_versions: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub response_nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub error_detail: ::core::option::Option<super::super::super::super::google::rpc::Status>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryResponse {
    #[prost(string, tag="1")]
    pub system_version_info: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    #[prost(string, tag="4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub removed_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub control_plane: ::core::option::Option<super::super::super::config::core::v3::ControlPlane>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub resource: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag="6")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="7")]
    pub cache_control: ::core::option::Option<resource::CacheControl>,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CacheControl {
        #[prost(bool, tag="1")]
        pub do_not_cache: bool,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdsDummy {
}
# [doc = r" Generated client implementations."] pub mod aggregated_discovery_service_client { # ! [allow (unused_variables , dead_code , missing_docs , clippy :: let_unit_value ,)] use tonic :: codegen :: * ; # [doc = " See https://github.com/lyft/envoy-api#apis for a description of the role of"] # [doc = " ADS and how it is intended to be used by a management server. ADS requests"] # [doc = " have the same structure as their singleton xDS counterparts, but can"] # [doc = " multiplex many resource types on a single stream. The type_url in the"] # [doc = " DiscoveryRequest/DiscoveryResponse provides sufficient information to recover"] # [doc = " the multiplexed singleton APIs at the Envoy instance and management server."] # [derive (Debug , Clone)] pub struct AggregatedDiscoveryServiceClient < T > { inner : tonic :: client :: Grpc < T > , } impl AggregatedDiscoveryServiceClient < tonic :: transport :: Channel > { # [doc = r" Attempt to create a new client by connecting to a given endpoint."] pub async fn connect < D > (dst : D) -> Result < Self , tonic :: transport :: Error > where D : std :: convert :: TryInto < tonic :: transport :: Endpoint > , D :: Error : Into < StdError > , { let conn = tonic :: transport :: Endpoint :: new (dst) ? . connect () . await ? ; Ok (Self :: new (conn)) } } impl < T > AggregatedDiscoveryServiceClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + Send + Sync + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as Body > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor < F > (inner : T , interceptor : F) -> AggregatedDiscoveryServiceClient < InterceptedService < T , F >> where F : tonic :: service :: Interceptor , T : tonic :: codegen :: Service < http :: Request < tonic :: body :: BoxBody > , Response = http :: Response << T as tonic :: client :: GrpcService < tonic :: body :: BoxBody >> :: ResponseBody > > , < T as tonic :: codegen :: Service < http :: Request < tonic :: body :: BoxBody >> > :: Error : Into < StdError > + Send + Sync , { AggregatedDiscoveryServiceClient :: new (InterceptedService :: new (inner , interceptor)) } # [doc = r" Compress requests with `gzip`."] # [doc = r""] # [doc = r" This requires the server to support it otherwise it might respond with an"] # [doc = r" error."] pub fn send_gzip (mut self) -> Self { self . inner = self . inner . send_gzip () ; self } # [doc = r" Enable decompressing responses with `gzip`."] pub fn accept_gzip (mut self) -> Self { self . inner = self . inner . accept_gzip () ; self } # [doc = " This is a gRPC-only API."] pub async fn stream_aggregated_resources (& mut self , request : impl tonic :: IntoStreamingRequest < Message = super :: DiscoveryRequest >) -> Result < tonic :: Response < tonic :: codec :: Streaming < super :: DiscoveryResponse >> , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/envoy.service.discovery.v3.AggregatedDiscoveryService/StreamAggregatedResources") ; self . inner . streaming (request . into_streaming_request () , path , codec) . await } pub async fn delta_aggregated_resources (& mut self , request : impl tonic :: IntoStreamingRequest < Message = super :: DeltaDiscoveryRequest >) -> Result < tonic :: Response < tonic :: codec :: Streaming < super :: DeltaDiscoveryResponse >> , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/envoy.service.discovery.v3.AggregatedDiscoveryService/DeltaAggregatedResources") ; self . inner . streaming (request . into_streaming_request () , path , codec) . await } } }# [doc = r" Generated server implementations."] pub mod aggregated_discovery_service_server { # ! [allow (unused_variables , dead_code , missing_docs , clippy :: let_unit_value ,)] use tonic :: codegen :: * ; # [doc = "Generated trait containing gRPC methods that should be implemented for use with AggregatedDiscoveryServiceServer."] # [async_trait] pub trait AggregatedDiscoveryService : Send + Sync + 'static { # [doc = "Server streaming response type for the StreamAggregatedResources method."] type StreamAggregatedResourcesStream : futures_core :: Stream < Item = Result < super :: DiscoveryResponse , tonic :: Status >> + Send + Sync + 'static ; # [doc = " This is a gRPC-only API."] async fn stream_aggregated_resources (& self , request : tonic :: Request < tonic :: Streaming < super :: DiscoveryRequest >>) -> Result < tonic :: Response < Self :: StreamAggregatedResourcesStream > , tonic :: Status > ; # [doc = "Server streaming response type for the DeltaAggregatedResources method."] type DeltaAggregatedResourcesStream : futures_core :: Stream < Item = Result < super :: DeltaDiscoveryResponse , tonic :: Status >> + Send + Sync + 'static ; async fn delta_aggregated_resources (& self , request : tonic :: Request < tonic :: Streaming < super :: DeltaDiscoveryRequest >>) -> Result < tonic :: Response < Self :: DeltaAggregatedResourcesStream > , tonic :: Status > ; } # [doc = " See https://github.com/lyft/envoy-api#apis for a description of the role of"] # [doc = " ADS and how it is intended to be used by a management server. ADS requests"] # [doc = " have the same structure as their singleton xDS counterparts, but can"] # [doc = " multiplex many resource types on a single stream. The type_url in the"] # [doc = " DiscoveryRequest/DiscoveryResponse provides sufficient information to recover"] # [doc = " the multiplexed singleton APIs at the Envoy instance and management server."] # [derive (Debug)] pub struct AggregatedDiscoveryServiceServer < T : AggregatedDiscoveryService > { inner : _Inner < T > , accept_compression_encodings : () , send_compression_encodings : () , } struct _Inner < T > (Arc < T >) ; impl < T : AggregatedDiscoveryService > AggregatedDiscoveryServiceServer < T > { pub fn new (inner : T) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner) ; Self { inner , accept_compression_encodings : Default :: default () , send_compression_encodings : Default :: default () , } } pub fn with_interceptor < F > (inner : T , interceptor : F) -> InterceptedService < Self , F > where F : tonic :: service :: Interceptor , { InterceptedService :: new (Self :: new (inner) , interceptor) } } impl < T , B > tonic :: codegen :: Service < http :: Request < B >> for AggregatedDiscoveryServiceServer < T > where T : AggregatedDiscoveryService , B : Body + Send + Sync + 'static , B :: Error : Into < StdError > + Send + 'static , { type Response = http :: Response < tonic :: body :: BoxBody > ; type Error = Never ; type Future = BoxFuture < Self :: Response , Self :: Error > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error >> { Poll :: Ready (Ok (())) } fn call (& mut self , req : http :: Request < B >) -> Self :: Future { let inner = self . inner . clone () ; match req . uri () . path () { "/envoy.service.discovery.v3.AggregatedDiscoveryService/StreamAggregatedResources" => { # [allow (non_camel_case_types)] struct StreamAggregatedResourcesSvc < T : AggregatedDiscoveryService > (pub Arc < T >) ; impl < T : AggregatedDiscoveryService > tonic :: server :: StreamingService < super :: DiscoveryRequest > for StreamAggregatedResourcesSvc < T > { type Response = super :: DiscoveryResponse ; type ResponseStream = T :: StreamAggregatedResourcesStream ; type Future = BoxFuture < tonic :: Response < Self :: ResponseStream > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < tonic :: Streaming < super :: DiscoveryRequest >>) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . stream_aggregated_resources (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = StreamAggregatedResourcesSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . streaming (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/envoy.service.discovery.v3.AggregatedDiscoveryService/DeltaAggregatedResources" => { # [allow (non_camel_case_types)] struct DeltaAggregatedResourcesSvc < T : AggregatedDiscoveryService > (pub Arc < T >) ; impl < T : AggregatedDiscoveryService > tonic :: server :: StreamingService < super :: DeltaDiscoveryRequest > for DeltaAggregatedResourcesSvc < T > { type Response = super :: DeltaDiscoveryResponse ; type ResponseStream = T :: DeltaAggregatedResourcesStream ; type Future = BoxFuture < tonic :: Response < Self :: ResponseStream > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < tonic :: Streaming < super :: DeltaDiscoveryRequest >>) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delta_aggregated_resources (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = DeltaAggregatedResourcesSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . streaming (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . header ("content-type" , "application/grpc") . body (empty_body ()) . unwrap ()) }) , } } } impl < T : AggregatedDiscoveryService > Clone for AggregatedDiscoveryServiceServer < T > { fn clone (& self) -> Self { let inner = self . inner . clone () ; Self { inner , accept_compression_encodings : self . accept_compression_encodings , send_compression_encodings : self . send_compression_encodings , } } } impl < T : AggregatedDiscoveryService > Clone for _Inner < T > { fn clone (& self) -> Self { Self (self . 0 . clone ()) } } impl < T : std :: fmt :: Debug > std :: fmt :: Debug for _Inner < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self . 0) } } impl < T : AggregatedDiscoveryService > tonic :: transport :: NamedService for AggregatedDiscoveryServiceServer < T > { const NAME : & 'static str = "envoy.service.discovery.v3.AggregatedDiscoveryService" ; } }