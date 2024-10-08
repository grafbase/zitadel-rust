// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {
    #[prost(message, optional, tag="1")]
    pub auth_option: ::core::option::Option<AuthOption>,
    #[prost(message, optional, tag="2")]
    pub http_response: ::core::option::Option<CustomHttpResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthOption {
    #[prost(string, tag="1")]
    pub permission: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub org_field: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomHttpResponse {
    #[prost(int32, tag="1")]
    pub success_code: i32,
}
// @@protoc_insertion_point(module)
