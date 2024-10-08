// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomOrgIamPolicyRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    /// the username has to end with the domain of its organization (uniqueness is organization based)
    #[prost(bool, tag="2")]
    pub user_login_must_be_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataOrg {
    #[prost(message, repeated, tag="1")]
    pub orgs: ::prost::alloc::vec::Vec<DataOrg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataOrg {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub org: ::core::option::Option<super::super::management::v1::AddOrgRequest>,
    #[prost(message, optional, tag="4")]
    pub iam_policy: ::core::option::Option<AddCustomOrgIamPolicyRequest>,
    #[prost(message, optional, tag="5")]
    pub label_policy: ::core::option::Option<super::super::management::v1::AddCustomLabelPolicyRequest>,
    #[prost(message, optional, tag="6")]
    pub lockout_policy: ::core::option::Option<super::super::management::v1::AddCustomLockoutPolicyRequest>,
    #[prost(message, optional, tag="7")]
    pub login_policy: ::core::option::Option<super::super::management::v1::AddCustomLoginPolicyRequest>,
    #[prost(message, optional, tag="8")]
    pub password_complexity_policy: ::core::option::Option<super::super::management::v1::AddCustomPasswordComplexityPolicyRequest>,
    #[prost(message, optional, tag="9")]
    pub privacy_policy: ::core::option::Option<super::super::management::v1::AddCustomPrivacyPolicyRequest>,
    #[prost(message, repeated, tag="10")]
    pub projects: ::prost::alloc::vec::Vec<DataProject>,
    #[prost(message, repeated, tag="11")]
    pub project_roles: ::prost::alloc::vec::Vec<super::super::management::v1::AddProjectRoleRequest>,
    #[prost(message, repeated, tag="12")]
    pub api_apps: ::prost::alloc::vec::Vec<DataApiApplication>,
    #[prost(message, repeated, tag="13")]
    pub oidc_apps: ::prost::alloc::vec::Vec<DataOidcApplication>,
    #[prost(message, repeated, tag="14")]
    pub human_users: ::prost::alloc::vec::Vec<DataHumanUser>,
    #[prost(message, repeated, tag="15")]
    pub machine_users: ::prost::alloc::vec::Vec<DataMachineUser>,
    #[prost(message, repeated, tag="16")]
    pub trigger_actions: ::prost::alloc::vec::Vec<SetTriggerActionsRequest>,
    #[prost(message, repeated, tag="17")]
    pub actions: ::prost::alloc::vec::Vec<DataAction>,
    #[prost(message, repeated, tag="18")]
    pub project_grants: ::prost::alloc::vec::Vec<DataProjectGrant>,
    #[prost(message, repeated, tag="19")]
    pub user_grants: ::prost::alloc::vec::Vec<super::super::management::v1::AddUserGrantRequest>,
    #[prost(message, repeated, tag="20")]
    pub org_members: ::prost::alloc::vec::Vec<super::super::management::v1::AddOrgMemberRequest>,
    #[prost(message, repeated, tag="21")]
    pub project_members: ::prost::alloc::vec::Vec<super::super::management::v1::AddProjectMemberRequest>,
    #[prost(message, repeated, tag="22")]
    pub project_grant_members: ::prost::alloc::vec::Vec<super::super::management::v1::AddProjectGrantMemberRequest>,
    #[prost(message, repeated, tag="23")]
    pub user_metadata: ::prost::alloc::vec::Vec<super::super::management::v1::SetUserMetadataRequest>,
    #[prost(message, repeated, tag="24")]
    pub login_texts: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomLoginTextsRequest>,
    #[prost(message, repeated, tag="25")]
    pub init_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomInitMessageTextRequest>,
    #[prost(message, repeated, tag="26")]
    pub password_reset_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomPasswordResetMessageTextRequest>,
    #[prost(message, repeated, tag="27")]
    pub verify_email_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomVerifyEmailMessageTextRequest>,
    #[prost(message, repeated, tag="28")]
    pub verify_phone_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomVerifyPhoneMessageTextRequest>,
    #[prost(message, repeated, tag="29")]
    pub domain_claimed_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomDomainClaimedMessageTextRequest>,
    #[prost(message, repeated, tag="30")]
    pub passwordless_registration_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomPasswordlessRegistrationMessageTextRequest>,
    #[prost(message, repeated, tag="31")]
    pub oidc_idps: ::prost::alloc::vec::Vec<DataOidcidp>,
    #[prost(message, repeated, tag="32")]
    pub jwt_idps: ::prost::alloc::vec::Vec<DataJwtidp>,
    #[prost(message, repeated, tag="33")]
    pub second_factors: ::prost::alloc::vec::Vec<super::super::management::v1::AddSecondFactorToLoginPolicyRequest>,
    #[prost(message, repeated, tag="34")]
    pub multi_factors: ::prost::alloc::vec::Vec<super::super::management::v1::AddMultiFactorToLoginPolicyRequest>,
    #[prost(message, repeated, tag="35")]
    pub idps: ::prost::alloc::vec::Vec<super::super::management::v1::AddIdpToLoginPolicyRequest>,
    #[prost(message, repeated, tag="36")]
    pub user_links: ::prost::alloc::vec::Vec<super::super::idp::v1::IdpUserLink>,
    #[prost(message, repeated, tag="37")]
    pub domains: ::prost::alloc::vec::Vec<super::super::org::v1::Domain>,
    #[prost(message, repeated, tag="38")]
    pub app_keys: ::prost::alloc::vec::Vec<DataAppKey>,
    #[prost(message, repeated, tag="39")]
    pub machine_keys: ::prost::alloc::vec::Vec<DataMachineKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataOidcidp {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub idp: ::core::option::Option<super::super::management::v1::AddOrgOidcidpRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataJwtidp {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="32")]
    pub idp: ::core::option::Option<super::super::management::v1::AddOrgJwtidpRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportHumanUser {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub profile: ::core::option::Option<export_human_user::Profile>,
    #[prost(message, optional, tag="3")]
    pub email: ::core::option::Option<export_human_user::Email>,
    #[prost(message, optional, tag="4")]
    pub phone: ::core::option::Option<export_human_user::Phone>,
    #[prost(string, tag="5")]
    pub password: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub hashed_password: ::core::option::Option<export_human_user::HashedPassword>,
    #[prost(bool, tag="7")]
    pub password_change_required: bool,
    #[prost(bool, tag="8")]
    pub request_passwordless_registration: bool,
    #[prost(string, tag="9")]
    pub otp_code: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExportHumanUser`.
pub mod export_human_user {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Profile {
        #[prost(string, tag="1")]
        pub first_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub last_name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub nick_name: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub display_name: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub preferred_language: ::prost::alloc::string::String,
        #[prost(enumeration="super::super::super::user::v1::Gender", tag="6")]
        pub gender: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Email {
        /// TODO: check if no value is allowed
        #[prost(string, tag="1")]
        pub email: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub is_email_verified: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Phone {
        /// has to be a global number
        #[prost(string, tag="1")]
        pub phone: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub is_phone_verified: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HashedPassword {
        #[prost(string, tag="1")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub algorithm: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataAppKey {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::authn::v1::KeyType", tag="5")]
    pub r#type: i32,
    #[prost(message, optional, tag="6")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bytes="vec", tag="7")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataMachineKey {
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::authn::v1::KeyType", tag="3")]
    pub r#type: i32,
    #[prost(message, optional, tag="4")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bytes="vec", tag="5")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProject {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub project: ::core::option::Option<super::super::management::v1::AddProjectRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataApiApplication {
    #[prost(string, tag="1")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub app: ::core::option::Option<super::super::management::v1::AddApiAppRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataOidcApplication {
    #[prost(string, tag="1")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub app: ::core::option::Option<super::super::management::v1::AddOidcAppRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataHumanUser {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub user: ::core::option::Option<super::super::management::v1::ImportHumanUserRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataMachineUser {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub user: ::core::option::Option<super::super::management::v1::AddMachineUserRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataAction {
    #[prost(string, tag="1")]
    pub action_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub action: ::core::option::Option<super::super::management::v1::CreateActionRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProjectGrant {
    #[prost(string, tag="1")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub project_grant: ::core::option::Option<super::super::management::v1::AddProjectGrantRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTriggerActionsRequest {
    #[prost(enumeration="FlowType", tag="1")]
    pub flow_type: i32,
    #[prost(enumeration="TriggerType", tag="2")]
    pub trigger_type: i32,
    #[prost(string, repeated, tag="3")]
    pub action_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlowType {
    Unspecified = 0,
    ExternalAuthentication = 1,
}
impl FlowType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlowType::Unspecified => "FLOW_TYPE_UNSPECIFIED",
            FlowType::ExternalAuthentication => "FLOW_TYPE_EXTERNAL_AUTHENTICATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FLOW_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "FLOW_TYPE_EXTERNAL_AUTHENTICATION" => Some(Self::ExternalAuthentication),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Unspecified = 0,
    PostAuthentication = 1,
    PreCreation = 2,
    PostCreation = 3,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::Unspecified => "TRIGGER_TYPE_UNSPECIFIED",
            TriggerType::PostAuthentication => "TRIGGER_TYPE_POST_AUTHENTICATION",
            TriggerType::PreCreation => "TRIGGER_TYPE_PRE_CREATION",
            TriggerType::PostCreation => "TRIGGER_TYPE_POST_CREATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRIGGER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRIGGER_TYPE_POST_AUTHENTICATION" => Some(Self::PostAuthentication),
            "TRIGGER_TYPE_PRE_CREATION" => Some(Self::PreCreation),
            "TRIGGER_TYPE_POST_CREATION" => Some(Self::PostCreation),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
