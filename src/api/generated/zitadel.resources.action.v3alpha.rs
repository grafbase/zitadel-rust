// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Timeout defines the duration until ZITADEL cancels the execution.
    #[prost(message, optional, tag="5")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(string, tag="6")]
    pub endpoint: ::prost::alloc::string::String,
    /// Defines the target type and how the response of the target is treated.
    #[prost(oneof="target::TargetType", tags="2, 3, 4")]
    pub target_type: ::core::option::Option<target::TargetType>,
}
/// Nested message and enum types in `Target`.
pub mod target {
    /// Defines the target type and how the response of the target is treated.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetType {
        #[prost(message, tag="2")]
        RestWebhook(super::SetRestWebhook),
        #[prost(message, tag="3")]
        RestCall(super::SetRestCall),
        #[prost(message, tag="4")]
        RestAsync(super::SetRestAsync),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTarget {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<Target>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchTarget {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Timeout defines the duration until ZITADEL cancels the execution.
    #[prost(message, optional, tag="5")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(string, optional, tag="6")]
    pub endpoint: ::core::option::Option<::prost::alloc::string::String>,
    /// Defines the target type and how the response of the target is treated.
    #[prost(oneof="patch_target::TargetType", tags="2, 3, 4")]
    pub target_type: ::core::option::Option<patch_target::TargetType>,
}
/// Nested message and enum types in `PatchTarget`.
pub mod patch_target {
    /// Defines the target type and how the response of the target is treated.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetType {
        #[prost(message, tag="2")]
        RestWebhook(super::SetRestWebhook),
        #[prost(message, tag="3")]
        RestCall(super::SetRestCall),
        #[prost(message, tag="4")]
        RestAsync(super::SetRestAsync),
    }
}
/// Wait for response but response body is ignored, status is checked, call is sent as post.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRestWebhook {
    /// Define if any error stops the whole execution. By default the process continues as normal.
    #[prost(bool, tag="1")]
    pub interrupt_on_error: bool,
}
/// Wait for response and response body is used, status is checked, call is sent as post.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRestCall {
    /// Define if any error stops the whole execution. By default the process continues as normal.
    #[prost(bool, tag="1")]
    pub interrupt_on_error: bool,
}
/// Call is executed in parallel to others, ZITADEL does not wait until the call is finished. The state is ignored, call is sent as post.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRestAsync {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Execution {
    /// Ordered list of targets/includes called during the execution.
    #[prost(message, repeated, tag="1")]
    pub targets: ::prost::alloc::vec::Vec<ExecutionTargetType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecution {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
    #[prost(message, optional, tag="2")]
    pub condition: ::core::option::Option<Condition>,
    #[prost(message, optional, tag="3")]
    pub execution: ::core::option::Option<Execution>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionTargetType {
    #[prost(oneof="execution_target_type::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<execution_target_type::Type>,
}
/// Nested message and enum types in `ExecutionTargetType`.
pub mod execution_target_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Unique identifier of existing target to call.
        #[prost(string, tag="1")]
        Target(::prost::alloc::string::String),
        /// Unique identifier of existing execution to include targets of.
        #[prost(message, tag="2")]
        Include(super::Condition),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// Condition-types under which conditions the execution should happen, only one possible.
    #[prost(oneof="condition::ConditionType", tags="1, 2, 3, 4")]
    pub condition_type: ::core::option::Option<condition::ConditionType>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    /// Condition-types under which conditions the execution should happen, only one possible.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConditionType {
        /// Condition-type to execute if a request on the defined API point happens.
        #[prost(message, tag="1")]
        Request(super::RequestExecution),
        /// Condition-type to execute on response if a request on the defined API point happens.
        #[prost(message, tag="2")]
        Response(super::ResponseExecution),
        /// Condition-type to execute if function is used, replaces actions v1.
        #[prost(message, tag="3")]
        Function(super::FunctionExecution),
        /// Condition-type to execute if an event is created in the system.
        #[prost(message, tag="4")]
        Event(super::EventExecution),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestExecution {
    /// Condition for the request execution. Only one is possible.
    #[prost(oneof="request_execution::Condition", tags="1, 2, 3")]
    pub condition: ::core::option::Option<request_execution::Condition>,
}
/// Nested message and enum types in `RequestExecution`.
pub mod request_execution {
    /// Condition for the request execution. Only one is possible.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        /// GRPC-method as condition.
        #[prost(string, tag="1")]
        Method(::prost::alloc::string::String),
        /// GRPC-service as condition.
        #[prost(string, tag="2")]
        Service(::prost::alloc::string::String),
        /// All calls to any available services and methods as condition.
        #[prost(bool, tag="3")]
        All(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseExecution {
    /// Condition for the response execution. Only one is possible.
    #[prost(oneof="response_execution::Condition", tags="1, 2, 3")]
    pub condition: ::core::option::Option<response_execution::Condition>,
}
/// Nested message and enum types in `ResponseExecution`.
pub mod response_execution {
    /// Condition for the response execution. Only one is possible.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        /// GRPC-method as condition.
        #[prost(string, tag="1")]
        Method(::prost::alloc::string::String),
        /// GRPC-service as condition.
        #[prost(string, tag="2")]
        Service(::prost::alloc::string::String),
        /// All calls to any available services and methods as condition.
        #[prost(bool, tag="3")]
        All(bool),
    }
}
/// Executed on the specified function
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionExecution {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventExecution {
    /// Condition for the event execution. Only one is possible.
    #[prost(oneof="event_execution::Condition", tags="1, 2, 3")]
    pub condition: ::core::option::Option<event_execution::Condition>,
}
/// Nested message and enum types in `EventExecution`.
pub mod event_execution {
    /// Condition for the event execution. Only one is possible.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        /// Event name as condition.
        #[prost(string, tag="1")]
        Event(::prost::alloc::string::String),
        /// Event group as condition, all events under this group.
        #[prost(string, tag="2")]
        Group(::prost::alloc::string::String),
        /// all events as condition.
        #[prost(bool, tag="3")]
        All(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionSearchFilter {
    #[prost(oneof="execution_search_filter::Filter", tags="1, 2, 3, 4")]
    pub filter: ::core::option::Option<execution_search_filter::Filter>,
}
/// Nested message and enum types in `ExecutionSearchFilter`.
pub mod execution_search_filter {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        #[prost(message, tag="1")]
        InConditionsFilter(super::InConditionsFilter),
        #[prost(message, tag="2")]
        ExecutionTypeFilter(super::ExecutionTypeFilter),
        #[prost(message, tag="3")]
        TargetFilter(super::TargetFilter),
        #[prost(message, tag="4")]
        IncludeFilter(super::IncludeFilter),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InConditionsFilter {
    /// Defines the conditions to query for.
    #[prost(message, repeated, tag="1")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionTypeFilter {
    /// Defines the type to query for.
    #[prost(enumeration="ExecutionType", tag="1")]
    pub execution_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetFilter {
    /// Defines the id to query for.
    #[prost(string, tag="1")]
    pub target_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncludeFilter {
    /// Defines the include to query for.
    #[prost(message, optional, tag="1")]
    pub include: ::core::option::Option<Condition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetSearchFilter {
    #[prost(oneof="target_search_filter::Filter", tags="1, 2")]
    pub filter: ::core::option::Option<target_search_filter::Filter>,
}
/// Nested message and enum types in `TargetSearchFilter`.
pub mod target_search_filter {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        #[prost(message, tag="1")]
        TargetNameFilter(super::TargetNameFilter),
        #[prost(message, tag="2")]
        InTargetIdsFilter(super::InTargetIDsFilter),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetNameFilter {
    /// Defines the name of the target to query for.
    #[prost(string, tag="1")]
    pub target_name: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the name query.
    #[prost(enumeration="super::super::object::v3alpha::TextFilterMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InTargetIDsFilter {
    /// Defines the ids to query for.
    #[prost(string, repeated, tag="1")]
    pub target_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionType {
    Unspecified = 0,
    Request = 1,
    Response = 2,
    Event = 3,
    Function = 4,
}
impl ExecutionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionType::Unspecified => "EXECUTION_TYPE_UNSPECIFIED",
            ExecutionType::Request => "EXECUTION_TYPE_REQUEST",
            ExecutionType::Response => "EXECUTION_TYPE_RESPONSE",
            ExecutionType::Event => "EXECUTION_TYPE_EVENT",
            ExecutionType::Function => "EXECUTION_TYPE_FUNCTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_TYPE_REQUEST" => Some(Self::Request),
            "EXECUTION_TYPE_RESPONSE" => Some(Self::Response),
            "EXECUTION_TYPE_EVENT" => Some(Self::Event),
            "EXECUTION_TYPE_FUNCTION" => Some(Self::Function),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TargetFieldName {
    Unspecified = 0,
    Id = 1,
    CreatedDate = 2,
    ChangedDate = 3,
    Name = 4,
    TargetType = 5,
    Url = 6,
    Timeout = 7,
    InterruptOnError = 8,
}
impl TargetFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TargetFieldName::Unspecified => "TARGET_FIELD_NAME_UNSPECIFIED",
            TargetFieldName::Id => "TARGET_FIELD_NAME_ID",
            TargetFieldName::CreatedDate => "TARGET_FIELD_NAME_CREATED_DATE",
            TargetFieldName::ChangedDate => "TARGET_FIELD_NAME_CHANGED_DATE",
            TargetFieldName::Name => "TARGET_FIELD_NAME_NAME",
            TargetFieldName::TargetType => "TARGET_FIELD_NAME_TARGET_TYPE",
            TargetFieldName::Url => "TARGET_FIELD_NAME_URL",
            TargetFieldName::Timeout => "TARGET_FIELD_NAME_TIMEOUT",
            TargetFieldName::InterruptOnError => "TARGET_FIELD_NAME_INTERRUPT_ON_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TARGET_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "TARGET_FIELD_NAME_ID" => Some(Self::Id),
            "TARGET_FIELD_NAME_CREATED_DATE" => Some(Self::CreatedDate),
            "TARGET_FIELD_NAME_CHANGED_DATE" => Some(Self::ChangedDate),
            "TARGET_FIELD_NAME_NAME" => Some(Self::Name),
            "TARGET_FIELD_NAME_TARGET_TYPE" => Some(Self::TargetType),
            "TARGET_FIELD_NAME_URL" => Some(Self::Url),
            "TARGET_FIELD_NAME_TIMEOUT" => Some(Self::Timeout),
            "TARGET_FIELD_NAME_INTERRUPT_ON_ERROR" => Some(Self::InterruptOnError),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionFieldName {
    Unspecified = 0,
    Id = 1,
    CreatedDate = 2,
    ChangedDate = 3,
}
impl ExecutionFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionFieldName::Unspecified => "EXECUTION_FIELD_NAME_UNSPECIFIED",
            ExecutionFieldName::Id => "EXECUTION_FIELD_NAME_ID",
            ExecutionFieldName::CreatedDate => "EXECUTION_FIELD_NAME_CREATED_DATE",
            ExecutionFieldName::ChangedDate => "EXECUTION_FIELD_NAME_CHANGED_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_FIELD_NAME_ID" => Some(Self::Id),
            "EXECUTION_FIELD_NAME_CREATED_DATE" => Some(Self::CreatedDate),
            "EXECUTION_FIELD_NAME_CHANGED_DATE" => Some(Self::ChangedDate),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<Target>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchTargetRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub target: ::core::option::Option<PatchTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchTargetResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTargetRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTargetResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetResponse {
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<GetTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTargetsRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    /// list limitations and ordering.
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::object::v3alpha::SearchQuery>,
    /// The field the result is sorted by. The default is the creation date. Beware that if you change this, your result pagination might be inconsistent.
    #[prost(enumeration="TargetFieldName", optional, tag="3")]
    pub sorting_column: ::core::option::Option<i32>,
    /// Define the criteria to query for.
    #[prost(message, repeated, tag="4")]
    pub filters: ::prost::alloc::vec::Vec<TargetSearchFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTargetsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<GetTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetExecutionRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    #[prost(message, optional, tag="2")]
    pub condition: ::core::option::Option<Condition>,
    #[prost(message, optional, tag="3")]
    pub execution: ::core::option::Option<Execution>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetExecutionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchExecutionsRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    /// list limitations and ordering.
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::object::v3alpha::SearchQuery>,
    /// The field the result is sorted by. The default is the creation date. Beware that if you change this, your result pagination might be inconsistent.
    #[prost(enumeration="ExecutionFieldName", optional, tag="3")]
    pub sorting_column: ::core::option::Option<i32>,
    /// Define the criteria to query for.
    #[prost(message, repeated, tag="4")]
    pub filters: ::prost::alloc::vec::Vec<ExecutionSearchFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchExecutionsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<GetExecution>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionFunctionsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionFunctionsResponse {
    /// All available methods
    #[prost(string, repeated, tag="1")]
    pub functions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionMethodsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionMethodsResponse {
    /// All available methods
    #[prost(string, repeated, tag="1")]
    pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionServicesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionServicesResponse {
    /// All available methods
    #[prost(string, repeated, tag="1")]
    pub services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
include!("zitadel.resources.action.v3alpha.tonic.rs");
// @@protoc_insertion_point(module)