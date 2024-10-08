// @generated
/// deprecated: please use DomainPolicy instead
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgIamPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(bool, tag="2")]
    pub user_login_must_be_domain: bool,
    #[prost(bool, tag="3")]
    pub is_default: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(bool, tag="2")]
    pub user_login_must_be_domain: bool,
    #[prost(bool, tag="3")]
    pub is_default: bool,
    #[prost(bool, tag="4")]
    pub validate_org_domains: bool,
    #[prost(bool, tag="5")]
    pub smtp_sender_address_matches_instance_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    /// hex value for primary color
    #[prost(string, tag="2")]
    pub primary_color: ::prost::alloc::string::String,
    /// defines if the organization's admin changed the policy
    #[prost(bool, tag="4")]
    pub is_default: bool,
    /// hides the org suffix on the login form if the scope \"urn:zitadel:iam:org:domain:primary:{domainname}\" is set
    #[prost(bool, tag="5")]
    pub hide_login_name_suffix: bool,
    /// hex value for secondary color
    #[prost(string, tag="6")]
    pub warn_color: ::prost::alloc::string::String,
    /// hex value for background color
    #[prost(string, tag="7")]
    pub background_color: ::prost::alloc::string::String,
    /// hex value for font color
    #[prost(string, tag="8")]
    pub font_color: ::prost::alloc::string::String,
    /// hex value for primary color dark theme
    #[prost(string, tag="9")]
    pub primary_color_dark: ::prost::alloc::string::String,
    /// hex value for background color dark theme
    #[prost(string, tag="10")]
    pub background_color_dark: ::prost::alloc::string::String,
    /// hex value for warning color dark theme
    #[prost(string, tag="11")]
    pub warn_color_dark: ::prost::alloc::string::String,
    /// hex value for font color dark theme
    #[prost(string, tag="12")]
    pub font_color_dark: ::prost::alloc::string::String,
    #[prost(bool, tag="13")]
    pub disable_watermark: bool,
    #[prost(string, tag="14")]
    pub logo_url: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub icon_url: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub logo_url_dark: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub icon_url_dark: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub font_url: ::prost::alloc::string::String,
    #[prost(enumeration="ThemeMode", tag="19")]
    pub theme_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(bool, tag="2")]
    pub allow_username_password: bool,
    #[prost(bool, tag="3")]
    pub allow_register: bool,
    #[prost(bool, tag="4")]
    pub allow_external_idp: bool,
    #[prost(bool, tag="5")]
    pub force_mfa: bool,
    #[prost(enumeration="PasswordlessType", tag="6")]
    pub passwordless_type: i32,
    #[prost(bool, tag="7")]
    pub is_default: bool,
    #[prost(bool, tag="8")]
    pub hide_password_reset: bool,
    #[prost(bool, tag="9")]
    pub ignore_unknown_usernames: bool,
    #[prost(string, tag="10")]
    pub default_redirect_uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag="11")]
    pub password_check_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="12")]
    pub external_login_check_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="13")]
    pub mfa_init_skip_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="14")]
    pub second_factor_check_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="15")]
    pub multi_factor_check_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(enumeration="SecondFactorType", repeated, tag="16")]
    pub second_factors: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="MultiFactorType", repeated, tag="17")]
    pub multi_factors: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag="18")]
    pub idps: ::prost::alloc::vec::Vec<super::super::idp::v1::IdpLoginPolicyLink>,
    /// If set to true, the suffix (@domain.com) of an unknown username input on the login screen will be matched against the org domains and will redirect to the registration of that organization on success.
    #[prost(bool, tag="19")]
    pub allow_domain_discovery: bool,
    #[prost(bool, tag="20")]
    pub disable_login_with_email: bool,
    #[prost(bool, tag="21")]
    pub disable_login_with_phone: bool,
    #[prost(bool, tag="22")]
    pub force_mfa_local_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordComplexityPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(uint64, tag="2")]
    pub min_length: u64,
    #[prost(bool, tag="3")]
    pub has_uppercase: bool,
    #[prost(bool, tag="4")]
    pub has_lowercase: bool,
    #[prost(bool, tag="5")]
    pub has_number: bool,
    #[prost(bool, tag="6")]
    pub has_symbol: bool,
    #[prost(bool, tag="7")]
    pub is_default: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordAgePolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    /// Amount of days after which a password will expire. The user will be forced to change the password on the following authentication.
    #[prost(uint64, tag="2")]
    pub max_age_days: u64,
    /// Amount of days after which the user should be notified of the upcoming expiry. ZITADEL will not notify the user.
    #[prost(uint64, tag="3")]
    pub expire_warn_days: u64,
    /// If true, the returned values represent the instance settings, e.g. by an organization without custom settings.
    #[prost(bool, tag="4")]
    pub is_default: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockoutPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(uint64, tag="2")]
    pub max_password_attempts: u64,
    #[prost(uint64, tag="3")]
    pub max_otp_attempts: u64,
    #[prost(bool, tag="4")]
    pub is_default: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacyPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub tos_link: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub privacy_link: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub is_default: bool,
    #[prost(string, tag="5")]
    pub help_link: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub support_email: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub docs_link: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub custom_link: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub custom_link_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationPolicy {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(bool, tag="2")]
    pub is_default: bool,
    #[prost(bool, tag="3")]
    pub password_change: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ThemeMode {
    Unspecified = 0,
    Auto = 1,
    Dark = 2,
    Light = 3,
}
impl ThemeMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ThemeMode::Unspecified => "THEME_MODE_UNSPECIFIED",
            ThemeMode::Auto => "THEME_MODE_AUTO",
            ThemeMode::Dark => "THEME_MODE_DARK",
            ThemeMode::Light => "THEME_MODE_LIGHT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "THEME_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "THEME_MODE_AUTO" => Some(Self::Auto),
            "THEME_MODE_DARK" => Some(Self::Dark),
            "THEME_MODE_LIGHT" => Some(Self::Light),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecondFactorType {
    Unspecified = 0,
    /// SECOND_FACTOR_TYPE_OTP is the type for TOTP
    Otp = 1,
    U2f = 2,
    OtpEmail = 3,
    OtpSms = 4,
}
impl SecondFactorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecondFactorType::Unspecified => "SECOND_FACTOR_TYPE_UNSPECIFIED",
            SecondFactorType::Otp => "SECOND_FACTOR_TYPE_OTP",
            SecondFactorType::U2f => "SECOND_FACTOR_TYPE_U2F",
            SecondFactorType::OtpEmail => "SECOND_FACTOR_TYPE_OTP_EMAIL",
            SecondFactorType::OtpSms => "SECOND_FACTOR_TYPE_OTP_SMS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECOND_FACTOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SECOND_FACTOR_TYPE_OTP" => Some(Self::Otp),
            "SECOND_FACTOR_TYPE_U2F" => Some(Self::U2f),
            "SECOND_FACTOR_TYPE_OTP_EMAIL" => Some(Self::OtpEmail),
            "SECOND_FACTOR_TYPE_OTP_SMS" => Some(Self::OtpSms),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MultiFactorType {
    Unspecified = 0,
    U2fWithVerification = 1,
}
impl MultiFactorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MultiFactorType::Unspecified => "MULTI_FACTOR_TYPE_UNSPECIFIED",
            MultiFactorType::U2fWithVerification => "MULTI_FACTOR_TYPE_U2F_WITH_VERIFICATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MULTI_FACTOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MULTI_FACTOR_TYPE_U2F_WITH_VERIFICATION" => Some(Self::U2fWithVerification),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PasswordlessType {
    NotAllowed = 0,
    /// PLANNED: PASSWORDLESS_TYPE_WITH_CERT
    Allowed = 1,
}
impl PasswordlessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PasswordlessType::NotAllowed => "PASSWORDLESS_TYPE_NOT_ALLOWED",
            PasswordlessType::Allowed => "PASSWORDLESS_TYPE_ALLOWED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PASSWORDLESS_TYPE_NOT_ALLOWED" => Some(Self::NotAllowed),
            "PASSWORDLESS_TYPE_ALLOWED" => Some(Self::Allowed),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
