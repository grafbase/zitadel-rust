// @generated
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceOwnerType {
    Unspecified = 0,
    Instance = 1,
    Org = 2,
}
impl ResourceOwnerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceOwnerType::Unspecified => "RESOURCE_OWNER_TYPE_UNSPECIFIED",
            ResourceOwnerType::Instance => "RESOURCE_OWNER_TYPE_INSTANCE",
            ResourceOwnerType::Org => "RESOURCE_OWNER_TYPE_ORG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESOURCE_OWNER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RESOURCE_OWNER_TYPE_INSTANCE" => Some(Self::Instance),
            "RESOURCE_OWNER_TYPE_ORG" => Some(Self::Org),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrandingSettings {
    #[prost(message, optional, tag="1")]
    pub light_theme: ::core::option::Option<Theme>,
    #[prost(message, optional, tag="2")]
    pub dark_theme: ::core::option::Option<Theme>,
    #[prost(string, tag="3")]
    pub font_url: ::prost::alloc::string::String,
    /// hides the org suffix on the login form if the scope \"urn:zitadel:iam:org:domain:primary:{domainname}\" is set
    #[prost(bool, tag="4")]
    pub hide_login_name_suffix: bool,
    #[prost(bool, tag="5")]
    pub disable_watermark: bool,
    /// resource_owner_type returns if the setting is managed on the organization or on the instance
    #[prost(enumeration="ResourceOwnerType", tag="6")]
    pub resource_owner_type: i32,
    #[prost(enumeration="ThemeMode", tag="7")]
    pub theme_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Theme {
    /// hex value for primary color
    #[prost(string, tag="1")]
    pub primary_color: ::prost::alloc::string::String,
    /// hex value for background color
    #[prost(string, tag="2")]
    pub background_color: ::prost::alloc::string::String,
    /// hex value for warning color
    #[prost(string, tag="3")]
    pub warn_color: ::prost::alloc::string::String,
    /// hex value for font color
    #[prost(string, tag="4")]
    pub font_color: ::prost::alloc::string::String,
    /// url where the logo is served
    #[prost(string, tag="5")]
    pub logo_url: ::prost::alloc::string::String,
    /// url where the icon is served
    #[prost(string, tag="6")]
    pub icon_url: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ThemeMode {
    Unspecified = 0,
    Auto = 1,
    Light = 2,
    Dark = 3,
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
            ThemeMode::Light => "THEME_MODE_LIGHT",
            ThemeMode::Dark => "THEME_MODE_DARK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "THEME_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "THEME_MODE_AUTO" => Some(Self::Auto),
            "THEME_MODE_LIGHT" => Some(Self::Light),
            "THEME_MODE_DARK" => Some(Self::Dark),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainSettings {
    #[prost(bool, tag="1")]
    pub login_name_includes_domain: bool,
    #[prost(bool, tag="2")]
    pub require_org_domain_verification: bool,
    #[prost(bool, tag="3")]
    pub smtp_sender_address_matches_instance_domain: bool,
    /// resource_owner_type returns if the setting is managed on the organization or on the instance
    #[prost(enumeration="ResourceOwnerType", tag="6")]
    pub resource_owner_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegalAndSupportSettings {
    #[prost(string, tag="1")]
    pub tos_link: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub privacy_policy_link: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub help_link: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub support_email: ::prost::alloc::string::String,
    /// resource_owner_type returns if the setting is managed on the organization or on the instance
    #[prost(enumeration="ResourceOwnerType", tag="5")]
    pub resource_owner_type: i32,
    #[prost(string, tag="6")]
    pub docs_link: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub custom_link: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub custom_link_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockoutSettings {
    #[prost(uint64, tag="1")]
    pub max_password_attempts: u64,
    /// resource_owner_type returns if the settings is managed on the organization or on the instance
    #[prost(enumeration="ResourceOwnerType", tag="2")]
    pub resource_owner_type: i32,
    #[prost(uint64, tag="3")]
    pub max_otp_attempts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginSettings {
    #[prost(bool, tag="1")]
    pub allow_username_password: bool,
    #[prost(bool, tag="2")]
    pub allow_register: bool,
    #[prost(bool, tag="3")]
    pub allow_external_idp: bool,
    #[prost(bool, tag="4")]
    pub force_mfa: bool,
    #[prost(enumeration="PasskeysType", tag="5")]
    pub passkeys_type: i32,
    #[prost(bool, tag="6")]
    pub hide_password_reset: bool,
    #[prost(bool, tag="7")]
    pub ignore_unknown_usernames: bool,
    #[prost(string, tag="8")]
    pub default_redirect_uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub password_check_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="10")]
    pub external_login_check_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="11")]
    pub mfa_init_skip_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="12")]
    pub second_factor_check_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="13")]
    pub multi_factor_check_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(enumeration="SecondFactorType", repeated, tag="14")]
    pub second_factors: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="MultiFactorType", repeated, tag="15")]
    pub multi_factors: ::prost::alloc::vec::Vec<i32>,
    /// If set to true, the suffix (@domain.com) of an unknown username input on the login screen will be matched against the org domains and will redirect to the registration of that organization on success.
    #[prost(bool, tag="16")]
    pub allow_domain_discovery: bool,
    #[prost(bool, tag="17")]
    pub disable_login_with_email: bool,
    #[prost(bool, tag="18")]
    pub disable_login_with_phone: bool,
    /// resource_owner_type returns if the settings is managed on the organization or on the instance
    #[prost(enumeration="ResourceOwnerType", tag="19")]
    pub resource_owner_type: i32,
    #[prost(bool, tag="22")]
    pub force_mfa_local_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityProvider {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="IdentityProviderType", tag="3")]
    pub r#type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecondFactorType {
    Unspecified = 0,
    /// This is the type for TOTP
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
pub enum PasskeysType {
    NotAllowed = 0,
    Allowed = 1,
}
impl PasskeysType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PasskeysType::NotAllowed => "PASSKEYS_TYPE_NOT_ALLOWED",
            PasskeysType::Allowed => "PASSKEYS_TYPE_ALLOWED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PASSKEYS_TYPE_NOT_ALLOWED" => Some(Self::NotAllowed),
            "PASSKEYS_TYPE_ALLOWED" => Some(Self::Allowed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdentityProviderType {
    Unspecified = 0,
    Oidc = 1,
    Jwt = 2,
    Ldap = 3,
    Oauth = 4,
    AzureAd = 5,
    Github = 6,
    GithubEs = 7,
    Gitlab = 8,
    GitlabSelfHosted = 9,
    Google = 10,
    Saml = 11,
}
impl IdentityProviderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdentityProviderType::Unspecified => "IDENTITY_PROVIDER_TYPE_UNSPECIFIED",
            IdentityProviderType::Oidc => "IDENTITY_PROVIDER_TYPE_OIDC",
            IdentityProviderType::Jwt => "IDENTITY_PROVIDER_TYPE_JWT",
            IdentityProviderType::Ldap => "IDENTITY_PROVIDER_TYPE_LDAP",
            IdentityProviderType::Oauth => "IDENTITY_PROVIDER_TYPE_OAUTH",
            IdentityProviderType::AzureAd => "IDENTITY_PROVIDER_TYPE_AZURE_AD",
            IdentityProviderType::Github => "IDENTITY_PROVIDER_TYPE_GITHUB",
            IdentityProviderType::GithubEs => "IDENTITY_PROVIDER_TYPE_GITHUB_ES",
            IdentityProviderType::Gitlab => "IDENTITY_PROVIDER_TYPE_GITLAB",
            IdentityProviderType::GitlabSelfHosted => "IDENTITY_PROVIDER_TYPE_GITLAB_SELF_HOSTED",
            IdentityProviderType::Google => "IDENTITY_PROVIDER_TYPE_GOOGLE",
            IdentityProviderType::Saml => "IDENTITY_PROVIDER_TYPE_SAML",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDENTITY_PROVIDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "IDENTITY_PROVIDER_TYPE_OIDC" => Some(Self::Oidc),
            "IDENTITY_PROVIDER_TYPE_JWT" => Some(Self::Jwt),
            "IDENTITY_PROVIDER_TYPE_LDAP" => Some(Self::Ldap),
            "IDENTITY_PROVIDER_TYPE_OAUTH" => Some(Self::Oauth),
            "IDENTITY_PROVIDER_TYPE_AZURE_AD" => Some(Self::AzureAd),
            "IDENTITY_PROVIDER_TYPE_GITHUB" => Some(Self::Github),
            "IDENTITY_PROVIDER_TYPE_GITHUB_ES" => Some(Self::GithubEs),
            "IDENTITY_PROVIDER_TYPE_GITLAB" => Some(Self::Gitlab),
            "IDENTITY_PROVIDER_TYPE_GITLAB_SELF_HOSTED" => Some(Self::GitlabSelfHosted),
            "IDENTITY_PROVIDER_TYPE_GOOGLE" => Some(Self::Google),
            "IDENTITY_PROVIDER_TYPE_SAML" => Some(Self::Saml),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordComplexitySettings {
    #[prost(uint64, tag="1")]
    pub min_length: u64,
    #[prost(bool, tag="2")]
    pub requires_uppercase: bool,
    #[prost(bool, tag="3")]
    pub requires_lowercase: bool,
    #[prost(bool, tag="4")]
    pub requires_number: bool,
    #[prost(bool, tag="5")]
    pub requires_symbol: bool,
    /// resource_owner_type returns if the settings is managed on the organization or on the instance
    #[prost(enumeration="ResourceOwnerType", tag="6")]
    pub resource_owner_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordExpirySettings {
    /// Amount of days after which a password will expire. The user will be forced to change the password on the following authentication.
    #[prost(uint64, tag="1")]
    pub max_age_days: u64,
    /// Amount of days after which the user should be notified of the upcoming expiry. ZITADEL will not notify the user.
    #[prost(uint64, tag="2")]
    pub expire_warn_days: u64,
    /// resource_owner_type returns if the settings is managed on the organization or on the instance
    #[prost(enumeration="ResourceOwnerType", tag="3")]
    pub resource_owner_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecuritySettings {
    #[prost(message, optional, tag="1")]
    pub embedded_iframe: ::core::option::Option<EmbeddedIframeSettings>,
    #[prost(bool, tag="2")]
    pub enable_impersonation: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbeddedIframeSettings {
    #[prost(bool, tag="1")]
    pub enabled: bool,
    #[prost(string, repeated, tag="2")]
    pub allowed_origins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginSettingsRequest {
    #[prost(message, optional, tag="1")]
    pub ctx: ::core::option::Option<super::super::object::v2beta::RequestContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginSettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub settings: ::core::option::Option<LoginSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPasswordComplexitySettingsRequest {
    #[prost(message, optional, tag="1")]
    pub ctx: ::core::option::Option<super::super::object::v2beta::RequestContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPasswordComplexitySettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub settings: ::core::option::Option<PasswordComplexitySettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPasswordExpirySettingsRequest {
    #[prost(message, optional, tag="1")]
    pub ctx: ::core::option::Option<super::super::object::v2beta::RequestContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPasswordExpirySettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub settings: ::core::option::Option<PasswordExpirySettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandingSettingsRequest {
    #[prost(message, optional, tag="1")]
    pub ctx: ::core::option::Option<super::super::object::v2beta::RequestContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandingSettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub settings: ::core::option::Option<BrandingSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDomainSettingsRequest {
    #[prost(message, optional, tag="1")]
    pub ctx: ::core::option::Option<super::super::object::v2beta::RequestContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDomainSettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub settings: ::core::option::Option<DomainSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLegalAndSupportSettingsRequest {
    #[prost(message, optional, tag="1")]
    pub ctx: ::core::option::Option<super::super::object::v2beta::RequestContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLegalAndSupportSettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub settings: ::core::option::Option<LegalAndSupportSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLockoutSettingsRequest {
    #[prost(message, optional, tag="1")]
    pub ctx: ::core::option::Option<super::super::object::v2beta::RequestContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLockoutSettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub settings: ::core::option::Option<LockoutSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveIdentityProvidersRequest {
    #[prost(message, optional, tag="1")]
    pub ctx: ::core::option::Option<super::super::object::v2beta::RequestContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveIdentityProvidersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub identity_providers: ::prost::alloc::vec::Vec<IdentityProvider>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeneralSettingsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeneralSettingsResponse {
    #[prost(string, tag="1")]
    pub default_org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub default_language: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub supported_languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecuritySettingsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecuritySettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(message, optional, tag="2")]
    pub settings: ::core::option::Option<SecuritySettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSecuritySettingsRequest {
    #[prost(message, optional, tag="1")]
    pub embedded_iframe: ::core::option::Option<EmbeddedIframeSettings>,
    #[prost(bool, tag="2")]
    pub enable_impersonation: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSecuritySettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
include!("zitadel.settings.v2beta.tonic.rs");
// @@protoc_insertion_point(module)