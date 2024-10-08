// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrganizationRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub admins: ::prost::alloc::vec::Vec<add_organization_request::Admin>,
}
/// Nested message and enum types in `AddOrganizationRequest`.
pub mod add_organization_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Admin {
        /// specify Org Member Roles for the provided user (default is ORG_OWNER if roles are empty)
        #[prost(string, repeated, tag="3")]
        pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(oneof="admin::UserType", tags="1, 2")]
        pub user_type: ::core::option::Option<admin::UserType>,
    }
    /// Nested message and enum types in `Admin`.
    pub mod admin {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum UserType {
            #[prost(string, tag="1")]
            UserId(::prost::alloc::string::String),
            #[prost(message, tag="2")]
            Human(super::super::super::super::user::v2beta::AddHumanUserRequest),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub created_admins: ::prost::alloc::vec::Vec<add_organization_response::CreatedAdmin>,
}
/// Nested message and enum types in `AddOrganizationResponse`.
pub mod add_organization_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreatedAdmin {
        #[prost(string, tag="1")]
        pub user_id: ::prost::alloc::string::String,
        #[prost(string, optional, tag="2")]
        pub email_code: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag="3")]
        pub phone_code: ::core::option::Option<::prost::alloc::string::String>,
    }
}
include!("zitadel.org.v2beta.tonic.rs");
// @@protoc_insertion_point(module)