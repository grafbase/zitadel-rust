// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Milestone {
    #[prost(enumeration="MilestoneType", tag="2")]
    pub r#type: i32,
    #[prost(message, optional, tag="3")]
    pub reached_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MilestoneQuery {
    #[prost(oneof="milestone_query::Query", tags="1")]
    pub query: ::core::option::Option<milestone_query::Query>,
}
/// Nested message and enum types in `MilestoneQuery`.
pub mod milestone_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        IsReachedQuery(super::IsReachedQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsReachedQuery {
    #[prost(bool, tag="1")]
    pub reached: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MilestoneType {
    Unspecified = 0,
    InstanceCreated = 1,
    AuthenticationSucceededOnInstance = 2,
    ProjectCreated = 3,
    ApplicationCreated = 4,
    AuthenticationSucceededOnApplication = 5,
    InstanceDeleted = 6,
}
impl MilestoneType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MilestoneType::Unspecified => "MILESTONE_TYPE_UNSPECIFIED",
            MilestoneType::InstanceCreated => "MILESTONE_TYPE_INSTANCE_CREATED",
            MilestoneType::AuthenticationSucceededOnInstance => "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_INSTANCE",
            MilestoneType::ProjectCreated => "MILESTONE_TYPE_PROJECT_CREATED",
            MilestoneType::ApplicationCreated => "MILESTONE_TYPE_APPLICATION_CREATED",
            MilestoneType::AuthenticationSucceededOnApplication => "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_APPLICATION",
            MilestoneType::InstanceDeleted => "MILESTONE_TYPE_INSTANCE_DELETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MILESTONE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MILESTONE_TYPE_INSTANCE_CREATED" => Some(Self::InstanceCreated),
            "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_INSTANCE" => Some(Self::AuthenticationSucceededOnInstance),
            "MILESTONE_TYPE_PROJECT_CREATED" => Some(Self::ProjectCreated),
            "MILESTONE_TYPE_APPLICATION_CREATED" => Some(Self::ApplicationCreated),
            "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_APPLICATION" => Some(Self::AuthenticationSucceededOnApplication),
            "MILESTONE_TYPE_INSTANCE_DELETED" => Some(Self::InstanceDeleted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MilestoneFieldName {
    Unspecified = 0,
    Type = 1,
    ReachedDate = 2,
}
impl MilestoneFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MilestoneFieldName::Unspecified => "MILESTONE_FIELD_NAME_UNSPECIFIED",
            MilestoneFieldName::Type => "MILESTONE_FIELD_NAME_TYPE",
            MilestoneFieldName::ReachedDate => "MILESTONE_FIELD_NAME_REACHED_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MILESTONE_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "MILESTONE_FIELD_NAME_TYPE" => Some(Self::Type),
            "MILESTONE_FIELD_NAME_REACHED_DATE" => Some(Self::ReachedDate),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
