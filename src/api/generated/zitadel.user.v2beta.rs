// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPasskeyRegistrationLink {
    #[prost(string, optional, tag="1")]
    pub url_template: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnPasskeyRegistrationCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasskeyRegistrationCode {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub code: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PasskeyAuthenticator {
    Unspecified = 0,
    Platform = 1,
    CrossPlatform = 2,
}
impl PasskeyAuthenticator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PasskeyAuthenticator::Unspecified => "PASSKEY_AUTHENTICATOR_UNSPECIFIED",
            PasskeyAuthenticator::Platform => "PASSKEY_AUTHENTICATOR_PLATFORM",
            PasskeyAuthenticator::CrossPlatform => "PASSKEY_AUTHENTICATOR_CROSS_PLATFORM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PASSKEY_AUTHENTICATOR_UNSPECIFIED" => Some(Self::Unspecified),
            "PASSKEY_AUTHENTICATOR_PLATFORM" => Some(Self::Platform),
            "PASSKEY_AUTHENTICATOR_CROSS_PLATFORM" => Some(Self::CrossPlatform),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHumanEmail {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    /// if no verification is specified, an email is sent with the default url
    #[prost(oneof="set_human_email::Verification", tags="2, 3, 4")]
    pub verification: ::core::option::Option<set_human_email::Verification>,
}
/// Nested message and enum types in `SetHumanEmail`.
pub mod set_human_email {
    /// if no verification is specified, an email is sent with the default url
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        #[prost(message, tag="2")]
        SendCode(super::SendEmailVerificationCode),
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnEmailVerificationCode),
        #[prost(bool, tag="4")]
        IsVerified(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanEmail {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEmailVerificationCode {
    #[prost(string, optional, tag="1")]
    pub url_template: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnEmailVerificationCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LdapCredentials {
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedirectUrLs {
    #[prost(string, tag="1")]
    pub success_url: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub failure_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpIntent {
    #[prost(string, tag="1")]
    pub idp_intent_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub idp_intent_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpInformation {
    #[prost(string, tag="2")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub raw_information: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(oneof="idp_information::Access", tags="1, 6, 7")]
    pub access: ::core::option::Option<idp_information::Access>,
}
/// Nested message and enum types in `IDPInformation`.
pub mod idp_information {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Access {
        #[prost(message, tag="1")]
        Oauth(super::IdpoAuthAccessInformation),
        #[prost(message, tag="6")]
        Ldap(super::IdpldapAccessInformation),
        #[prost(message, tag="7")]
        Saml(super::IdpsamlAccessInformation),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpoAuthAccessInformation {
    #[prost(string, tag="1")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub id_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpldapAccessInformation {
    #[prost(message, optional, tag="1")]
    pub attributes: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpsamlAccessInformation {
    #[prost(bytes="vec", tag="1")]
    pub assertion: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpLink {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Password {
    #[prost(string, tag="1")]
    pub password: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub change_required: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashedPassword {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub change_required: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPasswordResetLink {
    #[prost(enumeration="NotificationType", tag="1")]
    pub notification_type: i32,
    #[prost(string, optional, tag="2")]
    pub url_template: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnPasswordResetCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPassword {
    #[prost(oneof="set_password::PasswordType", tags="1, 2")]
    pub password_type: ::core::option::Option<set_password::PasswordType>,
    #[prost(oneof="set_password::Verification", tags="3, 4")]
    pub verification: ::core::option::Option<set_password::Verification>,
}
/// Nested message and enum types in `SetPassword`.
pub mod set_password {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PasswordType {
        #[prost(message, tag="1")]
        Password(super::Password),
        #[prost(message, tag="2")]
        HashedPassword(super::HashedPassword),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        #[prost(string, tag="3")]
        CurrentPassword(::prost::alloc::string::String),
        #[prost(string, tag="4")]
        VerificationCode(::prost::alloc::string::String),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationType {
    Unspecified = 0,
    Email = 1,
    Sms = 2,
}
impl NotificationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationType::Unspecified => "NOTIFICATION_TYPE_Unspecified",
            NotificationType::Email => "NOTIFICATION_TYPE_Email",
            NotificationType::Sms => "NOTIFICATION_TYPE_SMS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTIFICATION_TYPE_Unspecified" => Some(Self::Unspecified),
            "NOTIFICATION_TYPE_Email" => Some(Self::Email),
            "NOTIFICATION_TYPE_SMS" => Some(Self::Sms),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHumanPhone {
    #[prost(string, tag="1")]
    pub phone: ::prost::alloc::string::String,
    #[prost(oneof="set_human_phone::Verification", tags="2, 3, 4")]
    pub verification: ::core::option::Option<set_human_phone::Verification>,
}
/// Nested message and enum types in `SetHumanPhone`.
pub mod set_human_phone {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        #[prost(message, tag="2")]
        SendCode(super::SendPhoneVerificationCode),
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnPhoneVerificationCode),
        #[prost(bool, tag="4")]
        IsVerified(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanPhone {
    #[prost(string, tag="1")]
    pub phone: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPhoneVerificationCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnPhoneVerificationCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHumanProfile {
    #[prost(string, tag="1")]
    pub given_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub family_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub nick_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub preferred_language: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="Gender", optional, tag="6")]
    pub gender: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanProfile {
    #[prost(string, tag="1")]
    pub given_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub family_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub nick_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub preferred_language: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="Gender", optional, tag="6")]
    pub gender: ::core::option::Option<i32>,
    #[prost(string, tag="7")]
    pub avatar_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMetadataEntry {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanUser {
    /// Unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// State of the user, for example active, inactive, locked, deleted, initial.
    #[prost(enumeration="UserState", tag="2")]
    pub state: i32,
    /// Username of the user, which can be globally unique or unique on organization level.
    #[prost(string, tag="3")]
    pub username: ::prost::alloc::string::String,
    /// Possible usable login names for the user.
    #[prost(string, repeated, tag="4")]
    pub login_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Preferred login name of the user.
    #[prost(string, tag="5")]
    pub preferred_login_name: ::prost::alloc::string::String,
    /// Profile information of the user.
    #[prost(message, optional, tag="6")]
    pub profile: ::core::option::Option<HumanProfile>,
    /// Email of the user, if defined.
    #[prost(message, optional, tag="7")]
    pub email: ::core::option::Option<HumanEmail>,
    /// Phone of the user, if defined.
    #[prost(message, optional, tag="8")]
    pub phone: ::core::option::Option<HumanPhone>,
    /// User is required to change the used password on the next login.
    #[prost(bool, tag="9")]
    pub password_change_required: bool,
    /// The time the user last changed their password.
    #[prost(message, optional, tag="10")]
    pub password_changed: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(enumeration="UserState", tag="2")]
    pub state: i32,
    #[prost(string, tag="3")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub login_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub preferred_login_name: ::prost::alloc::string::String,
    #[prost(oneof="user::Type", tags="6, 7")]
    pub r#type: ::core::option::Option<user::Type>,
}
/// Nested message and enum types in `User`.
pub mod user {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="6")]
        Human(super::HumanUser),
        #[prost(message, tag="7")]
        Machine(super::MachineUser),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachineUser {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub has_secret: bool,
    #[prost(enumeration="AccessTokenType", tag="4")]
    pub access_token_type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
    Diverse = 3,
}
impl Gender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Gender::Unspecified => "GENDER_UNSPECIFIED",
            Gender::Female => "GENDER_FEMALE",
            Gender::Male => "GENDER_MALE",
            Gender::Diverse => "GENDER_DIVERSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GENDER_UNSPECIFIED" => Some(Self::Unspecified),
            "GENDER_FEMALE" => Some(Self::Female),
            "GENDER_MALE" => Some(Self::Male),
            "GENDER_DIVERSE" => Some(Self::Diverse),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessTokenType {
    Bearer = 0,
    Jwt = 1,
}
impl AccessTokenType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessTokenType::Bearer => "ACCESS_TOKEN_TYPE_BEARER",
            AccessTokenType::Jwt => "ACCESS_TOKEN_TYPE_JWT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_TOKEN_TYPE_BEARER" => Some(Self::Bearer),
            "ACCESS_TOKEN_TYPE_JWT" => Some(Self::Jwt),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
    Deleted = 3,
    Locked = 4,
    Initial = 5,
}
impl UserState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserState::Unspecified => "USER_STATE_UNSPECIFIED",
            UserState::Active => "USER_STATE_ACTIVE",
            UserState::Inactive => "USER_STATE_INACTIVE",
            UserState::Deleted => "USER_STATE_DELETED",
            UserState::Locked => "USER_STATE_LOCKED",
            UserState::Initial => "USER_STATE_INITIAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "USER_STATE_ACTIVE" => Some(Self::Active),
            "USER_STATE_INACTIVE" => Some(Self::Inactive),
            "USER_STATE_DELETED" => Some(Self::Deleted),
            "USER_STATE_LOCKED" => Some(Self::Locked),
            "USER_STATE_INITIAL" => Some(Self::Initial),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    #[prost(oneof="search_query::Query", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15")]
    pub query: ::core::option::Option<search_query::Query>,
}
/// Nested message and enum types in `SearchQuery`.
pub mod search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        UserNameQuery(super::UserNameQuery),
        #[prost(message, tag="2")]
        FirstNameQuery(super::FirstNameQuery),
        #[prost(message, tag="3")]
        LastNameQuery(super::LastNameQuery),
        #[prost(message, tag="4")]
        NickNameQuery(super::NickNameQuery),
        #[prost(message, tag="5")]
        DisplayNameQuery(super::DisplayNameQuery),
        #[prost(message, tag="6")]
        EmailQuery(super::EmailQuery),
        #[prost(message, tag="7")]
        StateQuery(super::StateQuery),
        #[prost(message, tag="8")]
        TypeQuery(super::TypeQuery),
        #[prost(message, tag="9")]
        LoginNameQuery(super::LoginNameQuery),
        #[prost(message, tag="10")]
        InUserIdsQuery(super::InUserIdQuery),
        #[prost(message, tag="11")]
        OrQuery(super::OrQuery),
        #[prost(message, tag="12")]
        AndQuery(super::AndQuery),
        #[prost(message, tag="13")]
        NotQuery(::prost::alloc::boxed::Box<super::NotQuery>),
        #[prost(message, tag="14")]
        InUserEmailsQuery(super::InUserEmailsQuery),
        #[prost(message, tag="15")]
        OrganizationIdQuery(super::OrganizationIdQuery),
    }
}
/// Connect multiple sub-condition with and OR operator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrQuery {
    #[prost(message, repeated, tag="1")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
}
/// Connect multiple sub-condition with and AND operator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndQuery {
    #[prost(message, repeated, tag="1")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
}
/// Negate the sub-condition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotQuery {
    #[prost(message, optional, boxed, tag="1")]
    pub query: ::core::option::Option<::prost::alloc::boxed::Box<SearchQuery>>,
}
/// Query for users with ID in list of IDs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InUserIdQuery {
    #[prost(string, repeated, tag="1")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Query for users with a specific user name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserNameQuery {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::object::v2beta::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// Query for users with a specific first name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirstNameQuery {
    #[prost(string, tag="1")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::object::v2beta::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// Query for users with a specific last name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastNameQuery {
    #[prost(string, tag="1")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::object::v2beta::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// Query for users with a specific nickname.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NickNameQuery {
    #[prost(string, tag="1")]
    pub nick_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::object::v2beta::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// Query for users with a specific display name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayNameQuery {
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::object::v2beta::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// Query for users with a specific email.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmailQuery {
    #[prost(string, tag="1")]
    pub email_address: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::object::v2beta::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// Query for users with a specific state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginNameQuery {
    #[prost(string, tag="1")]
    pub login_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::object::v2beta::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// Query for users with a specific state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateQuery {
    #[prost(enumeration="UserState", tag="1")]
    pub state: i32,
}
/// Query for users with a specific type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeQuery {
    #[prost(enumeration="Type", tag="1")]
    pub r#type: i32,
}
/// Query for users with email in list of emails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InUserEmailsQuery {
    #[prost(string, repeated, tag="1")]
    pub user_emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Query for users under a specific organization as resource owner.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationIdQuery {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    Unspecified = 0,
    Human = 1,
    Machine = 2,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "TYPE_UNSPECIFIED",
            Type::Human => "TYPE_HUMAN",
            Type::Machine => "TYPE_MACHINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TYPE_HUMAN" => Some(Self::Human),
            "TYPE_MACHINE" => Some(Self::Machine),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserFieldName {
    Unspecified = 0,
    UserName = 1,
    FirstName = 2,
    LastName = 3,
    NickName = 4,
    DisplayName = 5,
    Email = 6,
    State = 7,
    Type = 8,
    CreationDate = 9,
}
impl UserFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserFieldName::Unspecified => "USER_FIELD_NAME_UNSPECIFIED",
            UserFieldName::UserName => "USER_FIELD_NAME_USER_NAME",
            UserFieldName::FirstName => "USER_FIELD_NAME_FIRST_NAME",
            UserFieldName::LastName => "USER_FIELD_NAME_LAST_NAME",
            UserFieldName::NickName => "USER_FIELD_NAME_NICK_NAME",
            UserFieldName::DisplayName => "USER_FIELD_NAME_DISPLAY_NAME",
            UserFieldName::Email => "USER_FIELD_NAME_EMAIL",
            UserFieldName::State => "USER_FIELD_NAME_STATE",
            UserFieldName::Type => "USER_FIELD_NAME_TYPE",
            UserFieldName::CreationDate => "USER_FIELD_NAME_CREATION_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "USER_FIELD_NAME_USER_NAME" => Some(Self::UserName),
            "USER_FIELD_NAME_FIRST_NAME" => Some(Self::FirstName),
            "USER_FIELD_NAME_LAST_NAME" => Some(Self::LastName),
            "USER_FIELD_NAME_NICK_NAME" => Some(Self::NickName),
            "USER_FIELD_NAME_DISPLAY_NAME" => Some(Self::DisplayName),
            "USER_FIELD_NAME_EMAIL" => Some(Self::Email),
            "USER_FIELD_NAME_STATE" => Some(Self::State),
            "USER_FIELD_NAME_TYPE" => Some(Self::Type),
            "USER_FIELD_NAME_CREATION_DATE" => Some(Self::CreationDate),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHumanUserRequest {
    /// optionally set your own id unique for the user.
    #[prost(string, optional, tag="1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    /// optionally set a unique username, if none is provided the email will be used.
    #[prost(string, optional, tag="2")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="11")]
    pub organization: ::core::option::Option<super::super::object::v2beta::Organization>,
    #[prost(message, optional, tag="4")]
    pub profile: ::core::option::Option<SetHumanProfile>,
    #[prost(message, optional, tag="5")]
    pub email: ::core::option::Option<SetHumanEmail>,
    #[prost(message, optional, tag="10")]
    pub phone: ::core::option::Option<SetHumanPhone>,
    #[prost(message, repeated, tag="6")]
    pub metadata: ::prost::alloc::vec::Vec<SetMetadataEntry>,
    #[prost(message, repeated, tag="9")]
    pub idp_links: ::prost::alloc::vec::Vec<IdpLink>,
    /// An Implementation of RFC 6238 is used, with HMAC-SHA-1 and time-step of 30 seconds.
    /// Currently no other options are supported, and if anything different is used the validation will fail.
    #[prost(string, optional, tag="12")]
    pub totp_secret: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="add_human_user_request::PasswordType", tags="7, 8")]
    pub password_type: ::core::option::Option<add_human_user_request::PasswordType>,
}
/// Nested message and enum types in `AddHumanUserRequest`.
pub mod add_human_user_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PasswordType {
        #[prost(message, tag="7")]
        Password(super::Password),
        #[prost(message, tag="8")]
        HashedPassword(super::HashedPassword),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHumanUserResponse {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, optional, tag="3")]
    pub email_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub phone_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByIdRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByIdResponse {
    /// deprecated: details is moved into user
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::object::v2beta::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="UserFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::ListDetails>,
    #[prost(enumeration="UserFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetEmailRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    /// if no verification is specified, an email is sent with the default url
    #[prost(oneof="set_email_request::Verification", tags="3, 4, 5")]
    pub verification: ::core::option::Option<set_email_request::Verification>,
}
/// Nested message and enum types in `SetEmailRequest`.
pub mod set_email_request {
    /// if no verification is specified, an email is sent with the default url
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        #[prost(message, tag="3")]
        SendCode(super::SendEmailVerificationCode),
        #[prost(message, tag="4")]
        ReturnCode(super::ReturnEmailVerificationCode),
        #[prost(bool, tag="5")]
        IsVerified(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    /// in case the verification was set to return_code, the code will be returned
    #[prost(string, optional, tag="2")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendEmailCodeRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// if no verification is specified, an email is sent with the default url
    #[prost(oneof="resend_email_code_request::Verification", tags="2, 3")]
    pub verification: ::core::option::Option<resend_email_code_request::Verification>,
}
/// Nested message and enum types in `ResendEmailCodeRequest`.
pub mod resend_email_code_request {
    /// if no verification is specified, an email is sent with the default url
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        #[prost(message, tag="2")]
        SendCode(super::SendEmailVerificationCode),
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnEmailVerificationCode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendEmailCodeResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    /// in case the verification was set to return_code, the code will be returned
    #[prost(string, optional, tag="2")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyEmailRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub verification_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPhoneRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub phone: ::prost::alloc::string::String,
    /// if no verification is specified, an sms is sent
    #[prost(oneof="set_phone_request::Verification", tags="3, 4, 5")]
    pub verification: ::core::option::Option<set_phone_request::Verification>,
}
/// Nested message and enum types in `SetPhoneRequest`.
pub mod set_phone_request {
    /// if no verification is specified, an sms is sent
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        #[prost(message, tag="3")]
        SendCode(super::SendPhoneVerificationCode),
        #[prost(message, tag="4")]
        ReturnCode(super::ReturnPhoneVerificationCode),
        #[prost(bool, tag="5")]
        IsVerified(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    /// in case the verification was set to return_code, the code will be returned
    #[prost(string, optional, tag="2")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePhoneRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendPhoneCodeRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// if no verification is specified, an sms is sent
    #[prost(oneof="resend_phone_code_request::Verification", tags="3, 4")]
    pub verification: ::core::option::Option<resend_phone_code_request::Verification>,
}
/// Nested message and enum types in `ResendPhoneCodeRequest`.
pub mod resend_phone_code_request {
    /// if no verification is specified, an sms is sent
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        #[prost(message, tag="3")]
        SendCode(super::SendPhoneVerificationCode),
        #[prost(message, tag="4")]
        ReturnCode(super::ReturnPhoneVerificationCode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendPhoneCodeResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    /// in case the verification was set to return_code, the code will be returned
    #[prost(string, optional, tag="2")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyPhoneRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub verification_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanUserRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub profile: ::core::option::Option<SetHumanProfile>,
    #[prost(message, optional, tag="4")]
    pub email: ::core::option::Option<SetHumanEmail>,
    #[prost(message, optional, tag="5")]
    pub phone: ::core::option::Option<SetHumanPhone>,
    #[prost(message, optional, tag="6")]
    pub password: ::core::option::Option<SetPassword>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, optional, tag="2")]
    pub email_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub phone_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockUserRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockUserRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterPasskeyRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub code: ::core::option::Option<PasskeyRegistrationCode>,
    #[prost(enumeration="PasskeyAuthenticator", tag="3")]
    pub authenticator: i32,
    #[prost(string, tag="4")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterPasskeyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, tag="2")]
    pub passkey_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub public_key_credential_creation_options: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyPasskeyRegistrationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub passkey_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub public_key_credential: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(string, tag="4")]
    pub passkey_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyPasskeyRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterU2fRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterU2fResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, tag="2")]
    pub u2f_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub public_key_credential_creation_options: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyU2fRegistrationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub u2f_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub public_key_credential: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(string, tag="4")]
    pub token_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyU2fRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterTotpRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterTotpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub secret: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyTotpRegistrationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyTotpRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTotpRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTotpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOtpsmsRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOtpsmsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOtpsmsRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOtpsmsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOtpEmailRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOtpEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOtpEmailRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOtpEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePasskeyRegistrationLinkRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// if no medium is specified, an email is sent with the default url
    #[prost(oneof="create_passkey_registration_link_request::Medium", tags="2, 3")]
    pub medium: ::core::option::Option<create_passkey_registration_link_request::Medium>,
}
/// Nested message and enum types in `CreatePasskeyRegistrationLinkRequest`.
pub mod create_passkey_registration_link_request {
    /// if no medium is specified, an email is sent with the default url
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Medium {
        #[prost(message, tag="2")]
        SendLink(super::SendPasskeyRegistrationLink),
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnPasskeyRegistrationCode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePasskeyRegistrationLinkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    /// in case the medium was set to return_code, the code will be returned
    #[prost(message, optional, tag="2")]
    pub code: ::core::option::Option<PasskeyRegistrationCode>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartIdentityProviderIntentRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(oneof="start_identity_provider_intent_request::Content", tags="2, 3")]
    pub content: ::core::option::Option<start_identity_provider_intent_request::Content>,
}
/// Nested message and enum types in `StartIdentityProviderIntentRequest`.
pub mod start_identity_provider_intent_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag="2")]
        Urls(super::RedirectUrLs),
        #[prost(message, tag="3")]
        Ldap(super::LdapCredentials),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartIdentityProviderIntentResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(oneof="start_identity_provider_intent_response::NextStep", tags="2, 3, 4")]
    pub next_step: ::core::option::Option<start_identity_provider_intent_response::NextStep>,
}
/// Nested message and enum types in `StartIdentityProviderIntentResponse`.
pub mod start_identity_provider_intent_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NextStep {
        #[prost(string, tag="2")]
        AuthUrl(::prost::alloc::string::String),
        #[prost(message, tag="3")]
        IdpIntent(super::IdpIntent),
        #[prost(bytes, tag="4")]
        PostForm(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveIdentityProviderIntentRequest {
    #[prost(string, tag="1")]
    pub idp_intent_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub idp_intent_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveIdentityProviderIntentResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub idp_information: ::core::option::Option<IdpInformation>,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddIdpLinkRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub idp_link: ::core::option::Option<IdpLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddIdpLinkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordResetRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// if no medium is specified, an email is sent with the default url
    #[prost(oneof="password_reset_request::Medium", tags="2, 3")]
    pub medium: ::core::option::Option<password_reset_request::Medium>,
}
/// Nested message and enum types in `PasswordResetRequest`.
pub mod password_reset_request {
    /// if no medium is specified, an email is sent with the default url
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Medium {
        #[prost(message, tag="2")]
        SendLink(super::SendPasswordResetLink),
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnPasswordResetCode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordResetResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    /// in case the medium was set to return_code, the code will be returned
    #[prost(string, optional, tag="2")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPasswordRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub new_password: ::core::option::Option<Password>,
    /// if neither, the current password must be provided nor a verification code generated by the PasswordReset is provided,
    /// the user must be granted permission to set a password
    #[prost(oneof="set_password_request::Verification", tags="3, 4")]
    pub verification: ::core::option::Option<set_password_request::Verification>,
}
/// Nested message and enum types in `SetPasswordRequest`.
pub mod set_password_request {
    /// if neither, the current password must be provided nor a verification code generated by the PasswordReset is provided,
    /// the user must be granted permission to set a password
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        #[prost(string, tag="3")]
        CurrentPassword(::prost::alloc::string::String),
        #[prost(string, tag="4")]
        VerificationCode(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPasswordResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthenticationMethodTypesRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthenticationMethodTypesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::ListDetails>,
    #[prost(enumeration="AuthenticationMethodType", repeated, tag="2")]
    pub auth_method_types: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthenticationMethodType {
    Unspecified = 0,
    Password = 1,
    Passkey = 2,
    Idp = 3,
    Totp = 4,
    U2f = 5,
    OtpSms = 6,
    OtpEmail = 7,
}
impl AuthenticationMethodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthenticationMethodType::Unspecified => "AUTHENTICATION_METHOD_TYPE_UNSPECIFIED",
            AuthenticationMethodType::Password => "AUTHENTICATION_METHOD_TYPE_PASSWORD",
            AuthenticationMethodType::Passkey => "AUTHENTICATION_METHOD_TYPE_PASSKEY",
            AuthenticationMethodType::Idp => "AUTHENTICATION_METHOD_TYPE_IDP",
            AuthenticationMethodType::Totp => "AUTHENTICATION_METHOD_TYPE_TOTP",
            AuthenticationMethodType::U2f => "AUTHENTICATION_METHOD_TYPE_U2F",
            AuthenticationMethodType::OtpSms => "AUTHENTICATION_METHOD_TYPE_OTP_SMS",
            AuthenticationMethodType::OtpEmail => "AUTHENTICATION_METHOD_TYPE_OTP_EMAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTHENTICATION_METHOD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTHENTICATION_METHOD_TYPE_PASSWORD" => Some(Self::Password),
            "AUTHENTICATION_METHOD_TYPE_PASSKEY" => Some(Self::Passkey),
            "AUTHENTICATION_METHOD_TYPE_IDP" => Some(Self::Idp),
            "AUTHENTICATION_METHOD_TYPE_TOTP" => Some(Self::Totp),
            "AUTHENTICATION_METHOD_TYPE_U2F" => Some(Self::U2f),
            "AUTHENTICATION_METHOD_TYPE_OTP_SMS" => Some(Self::OtpSms),
            "AUTHENTICATION_METHOD_TYPE_OTP_EMAIL" => Some(Self::OtpEmail),
            _ => None,
        }
    }
}
include!("zitadel.user.v2beta.tonic.rs");
// @@protoc_insertion_point(module)