// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="State", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub domains: ::prost::alloc::vec::Vec<Domain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceDetail {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="State", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub domains: ::prost::alloc::vec::Vec<Domain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    #[prost(oneof="query::Query", tags="1, 2")]
    pub query: ::core::option::Option<query::Query>,
}
/// Nested message and enum types in `Query`.
pub mod query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        IdQuery(super::IdsQuery),
        #[prost(message, tag="2")]
        DomainQuery(super::DomainsQuery),
    }
}
/// IdQuery always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdsQuery {
    #[prost(string, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainsQuery {
    #[prost(string, repeated, tag="1")]
    pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub primary: bool,
    #[prost(bool, tag="4")]
    pub generated: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainSearchQuery {
    #[prost(oneof="domain_search_query::Query", tags="1, 2, 3")]
    pub query: ::core::option::Option<domain_search_query::Query>,
}
/// Nested message and enum types in `DomainSearchQuery`.
pub mod domain_search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        DomainQuery(super::DomainQuery),
        #[prost(message, tag="2")]
        GeneratedQuery(super::DomainGeneratedQuery),
        #[prost(message, tag="3")]
        PrimaryQuery(super::DomainPrimaryQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainQuery {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// DomainGeneratedQuery is always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainGeneratedQuery {
    #[prost(bool, tag="1")]
    pub generated: bool,
}
/// DomainPrimaryQuery is always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainPrimaryQuery {
    #[prost(bool, tag="1")]
    pub primary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustedDomain {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustedDomainSearchQuery {
    #[prost(oneof="trusted_domain_search_query::Query", tags="1")]
    pub query: ::core::option::Option<trusted_domain_search_query::Query>,
}
/// Nested message and enum types in `TrustedDomainSearchQuery`.
pub mod trusted_domain_search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        DomainQuery(super::DomainQuery),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    Unspecified = 0,
    Creating = 1,
    Running = 2,
    Stopping = 3,
    Stopped = 4,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::Unspecified => "STATE_UNSPECIFIED",
            State::Creating => "STATE_CREATING",
            State::Running => "STATE_RUNNING",
            State::Stopping => "STATE_STOPPING",
            State::Stopped => "STATE_STOPPED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "STATE_CREATING" => Some(Self::Creating),
            "STATE_RUNNING" => Some(Self::Running),
            "STATE_STOPPING" => Some(Self::Stopping),
            "STATE_STOPPED" => Some(Self::Stopped),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldName {
    Unspecified = 0,
    Id = 1,
    Name = 2,
    CreationDate = 3,
}
impl FieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FieldName::Unspecified => "FIELD_NAME_UNSPECIFIED",
            FieldName::Id => "FIELD_NAME_ID",
            FieldName::Name => "FIELD_NAME_NAME",
            FieldName::CreationDate => "FIELD_NAME_CREATION_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "FIELD_NAME_ID" => Some(Self::Id),
            "FIELD_NAME_NAME" => Some(Self::Name),
            "FIELD_NAME_CREATION_DATE" => Some(Self::CreationDate),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DomainFieldName {
    Unspecified = 0,
    Domain = 1,
    Primary = 2,
    Generated = 3,
    CreationDate = 4,
}
impl DomainFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DomainFieldName::Unspecified => "DOMAIN_FIELD_NAME_UNSPECIFIED",
            DomainFieldName::Domain => "DOMAIN_FIELD_NAME_DOMAIN",
            DomainFieldName::Primary => "DOMAIN_FIELD_NAME_PRIMARY",
            DomainFieldName::Generated => "DOMAIN_FIELD_NAME_GENERATED",
            DomainFieldName::CreationDate => "DOMAIN_FIELD_NAME_CREATION_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOMAIN_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "DOMAIN_FIELD_NAME_DOMAIN" => Some(Self::Domain),
            "DOMAIN_FIELD_NAME_PRIMARY" => Some(Self::Primary),
            "DOMAIN_FIELD_NAME_GENERATED" => Some(Self::Generated),
            "DOMAIN_FIELD_NAME_CREATION_DATE" => Some(Self::CreationDate),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
