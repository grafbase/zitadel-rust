// @generated
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthzRequest {
}
/// This is an empty response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthzResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::instance::v1::FieldName", tag="2")]
    pub sorting_column: i32,
    /// criterias the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::instance::v1::Query>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::instance::v1::FieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::instance::v1::Instance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceResponse {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::instance::v1::InstanceDetail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddInstanceRequest {
    #[prost(string, tag="1")]
    pub instance_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub first_org_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub custom_domain: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner_user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub owner_email: ::core::option::Option<add_instance_request::Email>,
    #[prost(message, optional, tag="6")]
    pub owner_profile: ::core::option::Option<add_instance_request::Profile>,
    #[prost(message, optional, tag="7")]
    pub owner_password: ::core::option::Option<add_instance_request::Password>,
    #[prost(string, tag="8")]
    pub default_language: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AddInstanceRequest`.
pub mod add_instance_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Profile {
        #[prost(string, tag="1")]
        pub first_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub last_name: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub preferred_language: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Email {
        #[prost(string, tag="1")]
        pub email: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub is_email_verified: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Password {
        #[prost(string, tag="1")]
        pub password: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub password_change_required: bool,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddInstanceResponse {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    #[prost(string, tag="1")]
    pub instance_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub first_org_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub custom_domain: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub default_language: ::prost::alloc::string::String,
    #[prost(oneof="create_instance_request::Owner", tags="4, 5")]
    pub owner: ::core::option::Option<create_instance_request::Owner>,
}
/// Nested message and enum types in `CreateInstanceRequest`.
pub mod create_instance_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Profile {
        #[prost(string, tag="1")]
        pub first_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub last_name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub preferred_language: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Email {
        #[prost(string, tag="1")]
        pub email: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub is_email_verified: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Password {
        #[prost(string, tag="1")]
        pub password: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub password_change_required: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Human {
        #[prost(string, tag="1")]
        pub user_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub email: ::core::option::Option<Email>,
        #[prost(message, optional, tag="3")]
        pub profile: ::core::option::Option<Profile>,
        #[prost(message, optional, tag="4")]
        pub password: ::core::option::Option<Password>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PersonalAccessToken {
        #[prost(message, optional, tag="1")]
        pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MachineKey {
        #[prost(enumeration="super::super::super::authn::v1::KeyType", tag="1")]
        pub r#type: i32,
        #[prost(message, optional, tag="2")]
        pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Machine {
        #[prost(string, tag="1")]
        pub user_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="3")]
        pub personal_access_token: ::core::option::Option<PersonalAccessToken>,
        #[prost(message, optional, tag="4")]
        pub machine_key: ::core::option::Option<MachineKey>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Owner {
        /// oneof field for the user managing the instance
        #[prost(message, tag="4")]
        Human(Human),
        #[prost(message, tag="5")]
        Machine(Machine),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceResponse {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="3")]
    pub pat: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub machine_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub instance_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveInstanceRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveInstanceResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIamMembersRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    #[prost(string, tag="2")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::member::v1::SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIamMembersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::member::v1::Member>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsageRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddQuotaRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    /// the unit a quota should be imposed on
    #[prost(enumeration="super::super::quota::v1::Unit", tag="2")]
    pub unit: i32,
    /// the starting time from which the current quota period is calculated from. This is relevant for querying the current usage.
    #[prost(message, optional, tag="3")]
    pub from: ::core::option::Option<::pbjson_types::Timestamp>,
    /// the quota periods duration
    #[prost(message, optional, tag="4")]
    pub reset_interval: ::core::option::Option<::pbjson_types::Duration>,
    /// the quota amount of units
    #[prost(uint64, tag="5")]
    pub amount: u64,
    /// whether ZITADEL should block further usage when the configured amount is used
    #[prost(bool, tag="6")]
    pub limit: bool,
    /// the handlers, ZITADEL executes when certain quota percentages are reached
    #[prost(message, repeated, tag="7")]
    pub notifications: ::prost::alloc::vec::Vec<super::super::quota::v1::Notification>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddQuotaResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetQuotaRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    /// the unit a quota should be imposed on
    #[prost(enumeration="super::super::quota::v1::Unit", tag="2")]
    pub unit: i32,
    /// the starting time from which the current quota period is calculated from. This is relevant for querying the current usage.
    #[prost(message, optional, tag="3")]
    pub from: ::core::option::Option<::pbjson_types::Timestamp>,
    /// the quota periods duration
    #[prost(message, optional, tag="4")]
    pub reset_interval: ::core::option::Option<::pbjson_types::Duration>,
    /// the quota amount of units
    #[prost(uint64, tag="5")]
    pub amount: u64,
    /// whether ZITADEL should block further usage when the configured amount is used
    #[prost(bool, tag="6")]
    pub limit: bool,
    /// the handlers, ZITADEL executes when certain quota percentages are reached
    #[prost(message, repeated, tag="7")]
    pub notifications: ::prost::alloc::vec::Vec<super::super::quota::v1::Notification>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetQuotaResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveQuotaRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::quota::v1::Unit", tag="2")]
    pub unit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveQuotaResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLimitsRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub audit_log_retention: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(bool, optional, tag="3")]
    pub block: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLimitsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkSetLimitsRequest {
    #[prost(message, repeated, tag="1")]
    pub limits: ::prost::alloc::vec::Vec<SetLimitsRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkSetLimitsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, repeated, tag="2")]
    pub target_details: ::prost::alloc::vec::Vec<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetLimitsRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetLimitsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExistsDomainRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExistsDomainResponse {
    #[prost(bool, tag="1")]
    pub exists: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainsRequest {
    /// list limitations and ordering
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::instance::v1::DomainFieldName", tag="3")]
    pub sorting_column: i32,
    /// criterias the client is looking for
    #[prost(message, repeated, tag="4")]
    pub queries: ::prost::alloc::vec::Vec<super::super::instance::v1::DomainSearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::instance::v1::DomainFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::instance::v1::Domain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDomainRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDomainRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPrimaryDomainRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPrimaryDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeSubscriptionRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subscription_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub request_limit: u64,
    #[prost(uint64, tag="4")]
    pub action_mins_limit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeSubscriptionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsResponse {
    /// TODO: list details
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<View>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearViewRequest {
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub view_name: ::prost::alloc::string::String,
}
/// This is an empty response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearViewResponse {
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFailedEventsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFailedEventsResponse {
    /// TODO: list details
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<FailedEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFailedEventRequest {
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub failed_sequence: u64,
    #[prost(string, tag="4")]
    pub instance_id: ::prost::alloc::string::String,
}
/// This is an empty response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFailedEventResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct View {
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub processed_sequence: u64,
    /// The timestamp the event occured
    #[prost(message, optional, tag="4")]
    pub event_timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub last_successful_spooler_run: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="6")]
    pub instance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedEvent {
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub failed_sequence: u64,
    #[prost(uint64, tag="4")]
    pub failure_count: u64,
    #[prost(string, tag="5")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub last_failed: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceFeatureRequest {
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::feature::v1::InstanceFeature", tag="2")]
    pub feature_id: i32,
    /// value based on the feature type
    #[prost(oneof="set_instance_feature_request::Value", tags="3")]
    pub value: ::core::option::Option<set_instance_feature_request::Value>,
}
/// Nested message and enum types in `SetInstanceFeatureRequest`.
pub mod set_instance_feature_request {
    /// value based on the feature type
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(bool, tag="3")]
        Bool(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceFeatureResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
include!("zitadel.system.v1.tonic.rs");
// @@protoc_insertion_point(module)