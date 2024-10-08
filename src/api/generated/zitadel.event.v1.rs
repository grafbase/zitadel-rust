// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(message, optional, tag="1")]
    pub editor: ::core::option::Option<Editor>,
    #[prost(message, optional, tag="2")]
    pub aggregate: ::core::option::Option<Aggregate>,
    #[prost(uint64, tag="3")]
    pub sequence: u64,
    #[prost(message, optional, tag="4")]
    pub creation_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub payload: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, optional, tag="6")]
    pub r#type: ::core::option::Option<EventType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Editor {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub service: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregate {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub r#type: ::core::option::Option<AggregateType>,
    #[prost(string, tag="3")]
    pub resource_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventType {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub localized: ::core::option::Option<super::super::v1::LocalizedMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateType {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub localized: ::core::option::Option<super::super::v1::LocalizedMessage>,
}
// @@protoc_insertion_point(module)
