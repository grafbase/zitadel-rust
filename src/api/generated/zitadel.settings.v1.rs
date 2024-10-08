// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretGenerator {
    #[prost(enumeration="SecretGeneratorType", tag="1")]
    pub generator_type: i32,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(uint32, tag="3")]
    pub length: u32,
    #[prost(message, optional, tag="4")]
    pub expiry: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(bool, tag="5")]
    pub include_lower_letters: bool,
    #[prost(bool, tag="6")]
    pub include_upper_letters: bool,
    #[prost(bool, tag="7")]
    pub include_digits: bool,
    #[prost(bool, tag="8")]
    pub include_symbols: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretGeneratorQuery {
    #[prost(oneof="secret_generator_query::Query", tags="1")]
    pub query: ::core::option::Option<secret_generator_query::Query>,
}
/// Nested message and enum types in `SecretGeneratorQuery`.
pub mod secret_generator_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        TypeQuery(super::SecretGeneratorTypeQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretGeneratorTypeQuery {
    #[prost(enumeration="SecretGeneratorType", tag="1")]
    pub generator_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmtpConfig {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub sender_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sender_name: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub tls: bool,
    #[prost(string, tag="5")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub reply_to_address: ::prost::alloc::string::String,
    #[prost(enumeration="SmtpConfigState", tag="8")]
    pub state: i32,
    #[prost(string, tag="9")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmsProvider {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="SmsProviderConfigState", tag="3")]
    pub state: i32,
    #[prost(oneof="sms_provider::Config", tags="4")]
    pub config: ::core::option::Option<sms_provider::Config>,
}
/// Nested message and enum types in `SMSProvider`.
pub mod sms_provider {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag="4")]
        Twilio(super::TwilioConfig),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwilioConfig {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender_number: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugNotificationProvider {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(bool, tag="2")]
    pub compact: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OidcSettings {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="2")]
    pub access_token_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub id_token_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="4")]
    pub refresh_token_idle_expiration: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="5")]
    pub refresh_token_expiration: ::core::option::Option<::pbjson_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    /// states if iframe embedding is enabled or disabled
    #[prost(bool, tag="2")]
    pub enable_iframe_embedding: bool,
    /// origins allowed loading ZITADEL in an iframe if enable_iframe_embedding is true
    #[prost(string, repeated, tag="3")]
    pub allowed_origins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// allows users to impersonate other users. The impersonator needs the appropriate `*_IMPERSONATOR` roles assigned as well"
    #[prost(bool, tag="4")]
    pub enable_impersonation: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SmtpConfigState {
    Unspecified = 0,
    SmtpConfigActive = 1,
    SmtpConfigInactive = 2,
}
impl SmtpConfigState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SmtpConfigState::Unspecified => "SMTP_CONFIG_STATE_UNSPECIFIED",
            SmtpConfigState::SmtpConfigActive => "SMTP_CONFIG_ACTIVE",
            SmtpConfigState::SmtpConfigInactive => "SMTP_CONFIG_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SMTP_CONFIG_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "SMTP_CONFIG_ACTIVE" => Some(Self::SmtpConfigActive),
            "SMTP_CONFIG_INACTIVE" => Some(Self::SmtpConfigInactive),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecretGeneratorType {
    Unspecified = 0,
    InitCode = 1,
    VerifyEmailCode = 2,
    VerifyPhoneCode = 3,
    PasswordResetCode = 4,
    PasswordlessInitCode = 5,
    AppSecret = 6,
    OtpSms = 7,
    OtpEmail = 8,
}
impl SecretGeneratorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecretGeneratorType::Unspecified => "SECRET_GENERATOR_TYPE_UNSPECIFIED",
            SecretGeneratorType::InitCode => "SECRET_GENERATOR_TYPE_INIT_CODE",
            SecretGeneratorType::VerifyEmailCode => "SECRET_GENERATOR_TYPE_VERIFY_EMAIL_CODE",
            SecretGeneratorType::VerifyPhoneCode => "SECRET_GENERATOR_TYPE_VERIFY_PHONE_CODE",
            SecretGeneratorType::PasswordResetCode => "SECRET_GENERATOR_TYPE_PASSWORD_RESET_CODE",
            SecretGeneratorType::PasswordlessInitCode => "SECRET_GENERATOR_TYPE_PASSWORDLESS_INIT_CODE",
            SecretGeneratorType::AppSecret => "SECRET_GENERATOR_TYPE_APP_SECRET",
            SecretGeneratorType::OtpSms => "SECRET_GENERATOR_TYPE_OTP_SMS",
            SecretGeneratorType::OtpEmail => "SECRET_GENERATOR_TYPE_OTP_EMAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECRET_GENERATOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SECRET_GENERATOR_TYPE_INIT_CODE" => Some(Self::InitCode),
            "SECRET_GENERATOR_TYPE_VERIFY_EMAIL_CODE" => Some(Self::VerifyEmailCode),
            "SECRET_GENERATOR_TYPE_VERIFY_PHONE_CODE" => Some(Self::VerifyPhoneCode),
            "SECRET_GENERATOR_TYPE_PASSWORD_RESET_CODE" => Some(Self::PasswordResetCode),
            "SECRET_GENERATOR_TYPE_PASSWORDLESS_INIT_CODE" => Some(Self::PasswordlessInitCode),
            "SECRET_GENERATOR_TYPE_APP_SECRET" => Some(Self::AppSecret),
            "SECRET_GENERATOR_TYPE_OTP_SMS" => Some(Self::OtpSms),
            "SECRET_GENERATOR_TYPE_OTP_EMAIL" => Some(Self::OtpEmail),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SmsProviderConfigState {
    Unspecified = 0,
    SmsProviderConfigActive = 1,
    SmsProviderConfigInactive = 2,
}
impl SmsProviderConfigState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SmsProviderConfigState::Unspecified => "SMS_PROVIDER_CONFIG_STATE_UNSPECIFIED",
            SmsProviderConfigState::SmsProviderConfigActive => "SMS_PROVIDER_CONFIG_ACTIVE",
            SmsProviderConfigState::SmsProviderConfigInactive => "SMS_PROVIDER_CONFIG_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SMS_PROVIDER_CONFIG_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "SMS_PROVIDER_CONFIG_ACTIVE" => Some(Self::SmsProviderConfigActive),
            "SMS_PROVIDER_CONFIG_INACTIVE" => Some(Self::SmsProviderConfigInactive),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
