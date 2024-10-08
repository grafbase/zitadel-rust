// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="ActionState", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub script: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(bool, tag="7")]
    pub allowed_to_fail: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionIdQuery {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionNameQuery {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// ActionStateQuery always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionStateQuery {
    #[prost(enumeration="ActionState", tag="1")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flow {
    /// id of the flow type
    #[prost(message, optional, tag="1")]
    pub r#type: ::core::option::Option<FlowType>,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="FlowState", tag="3")]
    pub state: i32,
    #[prost(message, repeated, tag="4")]
    pub trigger_actions: ::prost::alloc::vec::Vec<TriggerAction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowType {
    /// identifier of the type
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// key and name of the type
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<super::super::v1::LocalizedMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerType {
    /// identifier of the type
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// key and name of the type
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<super::super::v1::LocalizedMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerAction {
    /// id of the trigger type
    #[prost(message, optional, tag="1")]
    pub trigger_type: ::core::option::Option<TriggerType>,
    #[prost(message, repeated, tag="2")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionState {
    Unspecified = 0,
    Inactive = 1,
    Active = 2,
}
impl ActionState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionState::Unspecified => "ACTION_STATE_UNSPECIFIED",
            ActionState::Inactive => "ACTION_STATE_INACTIVE",
            ActionState::Active => "ACTION_STATE_ACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACTION_STATE_INACTIVE" => Some(Self::Inactive),
            "ACTION_STATE_ACTIVE" => Some(Self::Active),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionFieldName {
    Unspecified = 0,
    Name = 1,
    Id = 2,
    State = 3,
}
impl ActionFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionFieldName::Unspecified => "ACTION_FIELD_NAME_UNSPECIFIED",
            ActionFieldName::Name => "ACTION_FIELD_NAME_NAME",
            ActionFieldName::Id => "ACTION_FIELD_NAME_ID",
            ActionFieldName::State => "ACTION_FIELD_NAME_STATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "ACTION_FIELD_NAME_NAME" => Some(Self::Name),
            "ACTION_FIELD_NAME_ID" => Some(Self::Id),
            "ACTION_FIELD_NAME_STATE" => Some(Self::State),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlowState {
    Unspecified = 0,
    Inactive = 1,
    Active = 2,
}
impl FlowState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlowState::Unspecified => "FLOW_STATE_UNSPECIFIED",
            FlowState::Inactive => "FLOW_STATE_INACTIVE",
            FlowState::Active => "FLOW_STATE_ACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FLOW_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "FLOW_STATE_INACTIVE" => Some(Self::Inactive),
            "FLOW_STATE_ACTIVE" => Some(Self::Active),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
