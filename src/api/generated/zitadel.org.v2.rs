// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organization {
    /// Unique identifier of the organization.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// Current state of the organization, for example active, inactive and deleted.
    #[prost(enumeration="OrganizationState", tag="3")]
    pub state: i32,
    /// Name of the organization.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// Primary domain used in the organization.
    #[prost(string, tag="5")]
    pub primary_domain: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrganizationState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
    Removed = 3,
}
impl OrganizationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrganizationState::Unspecified => "ORGANIZATION_STATE_UNSPECIFIED",
            OrganizationState::Active => "ORGANIZATION_STATE_ACTIVE",
            OrganizationState::Inactive => "ORGANIZATION_STATE_INACTIVE",
            OrganizationState::Removed => "ORGANIZATION_STATE_REMOVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORGANIZATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ORGANIZATION_STATE_ACTIVE" => Some(Self::Active),
            "ORGANIZATION_STATE_INACTIVE" => Some(Self::Inactive),
            "ORGANIZATION_STATE_REMOVED" => Some(Self::Removed),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    #[prost(oneof="search_query::Query", tags="1, 2, 3, 4")]
    pub query: ::core::option::Option<search_query::Query>,
}
/// Nested message and enum types in `SearchQuery`.
pub mod search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        NameQuery(super::OrganizationNameQuery),
        #[prost(message, tag="2")]
        DomainQuery(super::OrganizationDomainQuery),
        #[prost(message, tag="3")]
        StateQuery(super::OrganizationStateQuery),
        #[prost(message, tag="4")]
        IdQuery(super::OrganizationIdQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationNameQuery {
    /// Name of the organization.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Defines which text equality method is used.
    #[prost(enumeration="super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationDomainQuery {
    /// Domain used in organization, not necessary primary domain.
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    /// Defines which text equality method is used.
    #[prost(enumeration="super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationStateQuery {
    /// Current state of the organization.
    #[prost(enumeration="OrganizationState", tag="1")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationIdQuery {
    /// Unique identifier of the organization.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrganizationFieldName {
    Unspecified = 0,
    Name = 1,
}
impl OrganizationFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrganizationFieldName::Unspecified => "ORGANIZATION_FIELD_NAME_UNSPECIFIED",
            OrganizationFieldName::Name => "ORGANIZATION_FIELD_NAME_NAME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORGANIZATION_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "ORGANIZATION_FIELD_NAME_NAME" => Some(Self::Name),
            _ => None,
        }
    }
}
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
            Human(super::super::super::super::user::v2::AddHumanUserRequest),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::object::v2::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="OrganizationFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::ListDetails>,
    #[prost(enumeration="OrganizationFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<Organization>,
}
include!("zitadel.org.v2.tonic.rs");
// @@protoc_insertion_point(module)