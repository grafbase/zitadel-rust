// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Project {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="ProjectState", tag="4")]
    pub state: i32,
    /// describes if the roles of the user should be added to the token
    #[prost(bool, tag="5")]
    pub project_role_assertion: bool,
    /// ZITADEL checks if the user has at least one on this project
    #[prost(bool, tag="6")]
    pub project_role_check: bool,
    /// ZITADEL checks if the org of the user has permission for this project
    #[prost(bool, tag="7")]
    pub has_project_check: bool,
    /// Defines from where the private labeling should be triggered
    #[prost(enumeration="PrivateLabelingSetting", tag="8")]
    pub private_labeling_setting: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantedProject {
    #[prost(string, tag="1")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub granted_org_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub granted_org_name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub granted_role_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="ProjectGrantState", tag="5")]
    pub state: i32,
    #[prost(string, tag="6")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub project_owner_id: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub project_owner_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="10")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectQuery {
    #[prost(oneof="project_query::Query", tags="1, 2")]
    pub query: ::core::option::Option<project_query::Query>,
}
/// Nested message and enum types in `ProjectQuery`.
pub mod project_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        NameQuery(super::ProjectNameQuery),
        #[prost(message, tag="2")]
        ProjectResourceOwnerQuery(super::ProjectResourceOwnerQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectNameQuery {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectResourceOwnerQuery {
    #[prost(string, tag="1")]
    pub resource_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub group: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleQuery {
    #[prost(oneof="role_query::Query", tags="1, 2")]
    pub query: ::core::option::Option<role_query::Query>,
}
/// Nested message and enum types in `RoleQuery`.
pub mod role_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        KeyQuery(super::RoleKeyQuery),
        #[prost(message, tag="2")]
        DisplayNameQuery(super::RoleDisplayNameQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleKeyQuery {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleDisplayNameQuery {
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectGrantQuery {
    #[prost(oneof="project_grant_query::Query", tags="1, 2")]
    pub query: ::core::option::Option<project_grant_query::Query>,
}
/// Nested message and enum types in `ProjectGrantQuery`.
pub mod project_grant_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        ProjectNameQuery(super::GrantProjectNameQuery),
        #[prost(message, tag="2")]
        RoleKeyQuery(super::GrantRoleKeyQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllProjectGrantQuery {
    #[prost(oneof="all_project_grant_query::Query", tags="1, 2, 3, 4")]
    pub query: ::core::option::Option<all_project_grant_query::Query>,
}
/// Nested message and enum types in `AllProjectGrantQuery`.
pub mod all_project_grant_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        ProjectNameQuery(super::GrantProjectNameQuery),
        #[prost(message, tag="2")]
        RoleKeyQuery(super::GrantRoleKeyQuery),
        #[prost(message, tag="3")]
        ProjectIdQuery(super::ProjectIdQuery),
        #[prost(message, tag="4")]
        GrantedOrgIdQuery(super::GrantedOrgIdQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantProjectNameQuery {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantRoleKeyQuery {
    #[prost(string, tag="1")]
    pub role_key: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectIdQuery {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantedOrgIdQuery {
    #[prost(string, tag="1")]
    pub granted_org_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProjectState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
}
impl ProjectState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProjectState::Unspecified => "PROJECT_STATE_UNSPECIFIED",
            ProjectState::Active => "PROJECT_STATE_ACTIVE",
            ProjectState::Inactive => "PROJECT_STATE_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROJECT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "PROJECT_STATE_ACTIVE" => Some(Self::Active),
            "PROJECT_STATE_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrivateLabelingSetting {
    Unspecified = 0,
    EnforceProjectResourceOwnerPolicy = 1,
    AllowLoginUserResourceOwnerPolicy = 2,
}
impl PrivateLabelingSetting {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PrivateLabelingSetting::Unspecified => "PRIVATE_LABELING_SETTING_UNSPECIFIED",
            PrivateLabelingSetting::EnforceProjectResourceOwnerPolicy => "PRIVATE_LABELING_SETTING_ENFORCE_PROJECT_RESOURCE_OWNER_POLICY",
            PrivateLabelingSetting::AllowLoginUserResourceOwnerPolicy => "PRIVATE_LABELING_SETTING_ALLOW_LOGIN_USER_RESOURCE_OWNER_POLICY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRIVATE_LABELING_SETTING_UNSPECIFIED" => Some(Self::Unspecified),
            "PRIVATE_LABELING_SETTING_ENFORCE_PROJECT_RESOURCE_OWNER_POLICY" => Some(Self::EnforceProjectResourceOwnerPolicy),
            "PRIVATE_LABELING_SETTING_ALLOW_LOGIN_USER_RESOURCE_OWNER_POLICY" => Some(Self::AllowLoginUserResourceOwnerPolicy),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProjectGrantState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
}
impl ProjectGrantState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProjectGrantState::Unspecified => "PROJECT_GRANT_STATE_UNSPECIFIED",
            ProjectGrantState::Active => "PROJECT_GRANT_STATE_ACTIVE",
            ProjectGrantState::Inactive => "PROJECT_GRANT_STATE_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROJECT_GRANT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "PROJECT_GRANT_STATE_ACTIVE" => Some(Self::Active),
            "PROJECT_GRANT_STATE_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
