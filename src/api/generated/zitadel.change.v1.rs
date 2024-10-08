// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Change {
    #[prost(message, optional, tag="1")]
    pub change_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub event_type: ::core::option::Option<super::super::v1::LocalizedMessage>,
    #[prost(uint64, tag="3")]
    pub sequence: u64,
    #[prost(string, tag="4")]
    pub editor_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub editor_display_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub resource_owner_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub editor_preferred_login_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub editor_avatar_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeQuery {
    #[prost(uint64, tag="1")]
    pub sequence: u64,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    #[prost(bool, tag="3")]
    pub asc: bool,
}
// @@protoc_insertion_point(module)
