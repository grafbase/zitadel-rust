// @generated
/// FeatureFlag is a simple boolean Feature setting, without further payload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureFlag {
    #[prost(bool, tag="1")]
    pub enabled: bool,
    #[prost(enumeration="Source", tag="2")]
    pub source: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImprovedPerformanceFeatureFlag {
    #[prost(enumeration="ImprovedPerformance", repeated, packed="false", tag="1")]
    pub execution_paths: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="Source", tag="2")]
    pub source: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Source {
    Unspecified = 0,
    System = 2,
    Instance = 3,
    Organization = 4,
    /// reserved for future use
    Project = 5,
    /// reserved for future use
    App = 6,
    User = 7,
}
impl Source {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Source::Unspecified => "SOURCE_UNSPECIFIED",
            Source::System => "SOURCE_SYSTEM",
            Source::Instance => "SOURCE_INSTANCE",
            Source::Organization => "SOURCE_ORGANIZATION",
            Source::Project => "SOURCE_PROJECT",
            Source::App => "SOURCE_APP",
            Source::User => "SOURCE_USER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
            "SOURCE_SYSTEM" => Some(Self::System),
            "SOURCE_INSTANCE" => Some(Self::Instance),
            "SOURCE_ORGANIZATION" => Some(Self::Organization),
            "SOURCE_PROJECT" => Some(Self::Project),
            "SOURCE_APP" => Some(Self::App),
            "SOURCE_USER" => Some(Self::User),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImprovedPerformance {
    Unspecified = 0,
    /// Uses the eventstore to query the org by id
    /// instead of the sql table.
    OrgById = 1,
    /// Improves performance on write side by using
    /// optimized processes to query data to determine
    /// correctnes of data.
    ProjectGrant = 2,
    Project = 3,
    UserGrant = 4,
    /// Improve performance on write side when
    /// users are checked against verified domains
    /// from other organizations.
    OrgDomainVerified = 5,
}
impl ImprovedPerformance {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImprovedPerformance::Unspecified => "IMPROVED_PERFORMANCE_UNSPECIFIED",
            ImprovedPerformance::OrgById => "IMPROVED_PERFORMANCE_ORG_BY_ID",
            ImprovedPerformance::ProjectGrant => "IMPROVED_PERFORMANCE_PROJECT_GRANT",
            ImprovedPerformance::Project => "IMPROVED_PERFORMANCE_PROJECT",
            ImprovedPerformance::UserGrant => "IMPROVED_PERFORMANCE_USER_GRANT",
            ImprovedPerformance::OrgDomainVerified => "IMPROVED_PERFORMANCE_ORG_DOMAIN_VERIFIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IMPROVED_PERFORMANCE_UNSPECIFIED" => Some(Self::Unspecified),
            "IMPROVED_PERFORMANCE_ORG_BY_ID" => Some(Self::OrgById),
            "IMPROVED_PERFORMANCE_PROJECT_GRANT" => Some(Self::ProjectGrant),
            "IMPROVED_PERFORMANCE_PROJECT" => Some(Self::Project),
            "IMPROVED_PERFORMANCE_USER_GRANT" => Some(Self::UserGrant),
            "IMPROVED_PERFORMANCE_ORG_DOMAIN_VERIFIED" => Some(Self::OrgDomainVerified),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSystemFeaturesRequest {
    #[prost(bool, optional, tag="1")]
    pub login_default_org: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub oidc_trigger_introspection_projections: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub oidc_legacy_introspection: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub user_schema: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="5")]
    pub oidc_token_exchange: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6")]
    pub actions: ::core::option::Option<bool>,
    #[prost(enumeration="ImprovedPerformance", repeated, packed="false", tag="7")]
    pub improved_performance: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSystemFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetSystemFeaturesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetSystemFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemFeaturesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    #[prost(message, optional, tag="2")]
    pub login_default_org: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="3")]
    pub oidc_trigger_introspection_projections: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="4")]
    pub oidc_legacy_introspection: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="5")]
    pub user_schema: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="6")]
    pub oidc_token_exchange: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="7")]
    pub actions: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="8")]
    pub improved_performance: ::core::option::Option<ImprovedPerformanceFeatureFlag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceFeaturesRequest {
    #[prost(bool, optional, tag="1")]
    pub login_default_org: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub oidc_trigger_introspection_projections: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub oidc_legacy_introspection: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub user_schema: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="5")]
    pub oidc_token_exchange: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6")]
    pub actions: ::core::option::Option<bool>,
    #[prost(enumeration="ImprovedPerformance", repeated, packed="false", tag="7")]
    pub improved_performance: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag="8")]
    pub web_key: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceFeaturesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceFeaturesRequest {
    #[prost(bool, tag="1")]
    pub inheritance: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    #[prost(message, optional, tag="2")]
    pub login_default_org: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="3")]
    pub oidc_trigger_introspection_projections: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="4")]
    pub oidc_legacy_introspection: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="5")]
    pub user_schema: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="6")]
    pub oidc_token_exchange: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="7")]
    pub actions: ::core::option::Option<FeatureFlag>,
    #[prost(message, optional, tag="8")]
    pub improved_performance: ::core::option::Option<ImprovedPerformanceFeatureFlag>,
    #[prost(message, optional, tag="9")]
    pub web_key: ::core::option::Option<FeatureFlag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrganizationFeaturesRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrganizationFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetOrganizationFeaturesRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetOrganizationFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationFeaturesRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub inheritance: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUserFeatureRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUserFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetUserFeaturesRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetUserFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserFeaturesRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub inheritance: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserFeaturesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
include!("zitadel.feature.v2.tonic.rs");
// @@protoc_insertion_point(module)