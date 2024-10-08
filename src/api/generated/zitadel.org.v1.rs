// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Org {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="OrgState", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub primary_domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="3")]
    pub domain_name: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub is_verified: bool,
    #[prost(bool, tag="5")]
    pub is_primary: bool,
    #[prost(enumeration="DomainValidationType", tag="6")]
    pub validation_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgQuery {
    #[prost(oneof="org_query::Query", tags="1, 2, 3, 4")]
    pub query: ::core::option::Option<org_query::Query>,
}
/// Nested message and enum types in `OrgQuery`.
pub mod org_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        NameQuery(super::OrgNameQuery),
        #[prost(message, tag="2")]
        DomainQuery(super::OrgDomainQuery),
        #[prost(message, tag="3")]
        StateQuery(super::OrgStateQuery),
        #[prost(message, tag="4")]
        IdQuery(super::OrgIdQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgNameQuery {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgDomainQuery {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgStateQuery {
    #[prost(enumeration="OrgState", tag="1")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgIdQuery {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainSearchQuery {
    #[prost(oneof="domain_search_query::Query", tags="1")]
    pub query: ::core::option::Option<domain_search_query::Query>,
}
/// Nested message and enum types in `DomainSearchQuery`.
pub mod domain_search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        DomainNameQuery(super::DomainNameQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainNameQuery {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrgState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
    Removed = 3,
}
impl OrgState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrgState::Unspecified => "ORG_STATE_UNSPECIFIED",
            OrgState::Active => "ORG_STATE_ACTIVE",
            OrgState::Inactive => "ORG_STATE_INACTIVE",
            OrgState::Removed => "ORG_STATE_REMOVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORG_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ORG_STATE_ACTIVE" => Some(Self::Active),
            "ORG_STATE_INACTIVE" => Some(Self::Inactive),
            "ORG_STATE_REMOVED" => Some(Self::Removed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DomainValidationType {
    Unspecified = 0,
    Http = 1,
    Dns = 2,
}
impl DomainValidationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DomainValidationType::Unspecified => "DOMAIN_VALIDATION_TYPE_UNSPECIFIED",
            DomainValidationType::Http => "DOMAIN_VALIDATION_TYPE_HTTP",
            DomainValidationType::Dns => "DOMAIN_VALIDATION_TYPE_DNS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOMAIN_VALIDATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "DOMAIN_VALIDATION_TYPE_HTTP" => Some(Self::Http),
            "DOMAIN_VALIDATION_TYPE_DNS" => Some(Self::Dns),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrgFieldName {
    Unspecified = 0,
    Name = 1,
}
impl OrgFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrgFieldName::Unspecified => "ORG_FIELD_NAME_UNSPECIFIED",
            OrgFieldName::Name => "ORG_FIELD_NAME_NAME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORG_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "ORG_FIELD_NAME_NAME" => Some(Self::Name),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
