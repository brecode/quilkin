#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancer {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<load_balancer::PolicyValue>,
}
/// Nested message and enum types in `LoadBalancer`.
pub mod load_balancer {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolicyValue {
        #[prost(enumeration="Policy", tag="1")]
        pub value: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Policy {
        RoundRobin = 0,
        Random = 1,
        ControlPlane = 2,
    }
}
