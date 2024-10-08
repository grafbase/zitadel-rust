// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Idp {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="IdpState", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="IdpStylingType", tag="5")]
    pub styling_type: i32,
    #[prost(enumeration="IdpOwnerType", tag="6")]
    pub owner: i32,
    #[prost(bool, tag="8")]
    pub auto_register: bool,
    #[prost(oneof="idp::Config", tags="7, 9")]
    pub config: ::core::option::Option<idp::Config>,
}
/// Nested message and enum types in `IDP`.
pub mod idp {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag="7")]
        OidcConfig(super::OidcConfig),
        #[prost(message, tag="9")]
        JwtConfig(super::JwtConfig),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpUserLink {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub idp_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub provided_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub provided_user_name: ::prost::alloc::string::String,
    #[prost(enumeration="IdpType", tag="6")]
    pub idp_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpLoginPolicyLink {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub idp_name: ::prost::alloc::string::String,
    #[prost(enumeration="IdpType", tag="3")]
    pub idp_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OidcConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="OidcMappingField", tag="4")]
    pub display_name_mapping: i32,
    #[prost(enumeration="OidcMappingField", tag="5")]
    pub username_mapping: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtConfig {
    #[prost(string, tag="1")]
    pub jwt_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub keys_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub header_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpidQuery {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpNameQuery {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpOwnerTypeQuery {
    #[prost(enumeration="IdpOwnerType", tag="1")]
    pub owner_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Provider {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="IdpState", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="IdpOwnerType", tag="5")]
    pub owner: i32,
    #[prost(enumeration="ProviderType", tag="6")]
    pub r#type: i32,
    #[prost(message, optional, tag="7")]
    pub config: ::core::option::Option<ProviderConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderConfig {
    #[prost(message, optional, tag="1")]
    pub options: ::core::option::Option<Options>,
    #[prost(oneof="provider_config::Config", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub config: ::core::option::Option<provider_config::Config>,
}
/// Nested message and enum types in `ProviderConfig`.
pub mod provider_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag="2")]
        Ldap(super::LdapConfig),
        #[prost(message, tag="3")]
        Google(super::GoogleConfig),
        #[prost(message, tag="4")]
        Oauth(super::OAuthConfig),
        #[prost(message, tag="5")]
        Oidc(super::GenericOidcConfig),
        #[prost(message, tag="6")]
        Jwt(super::JwtConfig),
        #[prost(message, tag="7")]
        Github(super::GitHubConfig),
        #[prost(message, tag="8")]
        GithubEs(super::GitHubEnterpriseServerConfig),
        #[prost(message, tag="9")]
        Gitlab(super::GitLabConfig),
        #[prost(message, tag="10")]
        GitlabSelfHosted(super::GitLabSelfHostedConfig),
        #[prost(message, tag="11")]
        AzureAd(super::AzureAdConfig),
        #[prost(message, tag="12")]
        Apple(super::AppleConfig),
        #[prost(message, tag="13")]
        Saml(super::SamlConfig),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_endpoint: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub id_attribute: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericOidcConfig {
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="4")]
    pub is_id_token_mapping: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitHubConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitHubEnterpriseServerConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_endpoint: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitLabConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitLabSelfHostedConfig {
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LdapConfig {
    #[prost(string, repeated, tag="1")]
    pub servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="2")]
    pub start_tls: bool,
    #[prost(string, tag="3")]
    pub base_dn: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub bind_dn: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user_base: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub user_object_classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="7")]
    pub user_filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="9")]
    pub attributes: ::core::option::Option<LdapAttributes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamlConfig {
    /// Metadata of the SAML identity provider.
    #[prost(bytes="vec", tag="1")]
    pub metadata_xml: ::prost::alloc::vec::Vec<u8>,
    /// Binding which defines the type of communication with the identity provider.
    #[prost(enumeration="SamlBinding", tag="2")]
    pub binding: i32,
    /// Boolean which defines if the authentication requests are signed.
    #[prost(bool, tag="3")]
    pub with_signed_request: bool,
    /// `nameid-format` for the SAML Request.
    #[prost(enumeration="SamlNameIdFormat", tag="4")]
    pub name_id_format: i32,
    /// Optional name of the attribute, which will be used to map the user
    /// in case the nameid-format returned is `urn:oasis:names:tc:SAML:2.0:nameid-format:transient`.
    #[prost(string, optional, tag="5")]
    pub transient_mapping_attribute_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureAdConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub tenant: ::core::option::Option<AzureAdTenant>,
    #[prost(bool, tag="3")]
    pub email_verified: bool,
    #[prost(string, repeated, tag="4")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {
    #[prost(bool, tag="1")]
    pub is_linking_allowed: bool,
    #[prost(bool, tag="2")]
    pub is_creation_allowed: bool,
    #[prost(bool, tag="3")]
    pub is_auto_creation: bool,
    #[prost(bool, tag="4")]
    pub is_auto_update: bool,
    #[prost(enumeration="AutoLinkingOption", tag="5")]
    pub auto_linking: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LdapAttributes {
    #[prost(string, tag="1")]
    pub id_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub first_name_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub last_name_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub display_name_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub nick_name_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub preferred_username_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub email_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub email_verified_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub phone_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub phone_verified_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub preferred_language_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub avatar_url_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub profile_attribute: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureAdTenant {
    #[prost(oneof="azure_ad_tenant::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<azure_ad_tenant::Type>,
}
/// Nested message and enum types in `AzureADTenant`.
pub mod azure_ad_tenant {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(enumeration="super::AzureAdTenantType", tag="1")]
        TenantType(i32),
        #[prost(string, tag="2")]
        TenantId(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppleConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub team_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub key_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdpState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
}
impl IdpState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdpState::Unspecified => "IDP_STATE_UNSPECIFIED",
            IdpState::Active => "IDP_STATE_ACTIVE",
            IdpState::Inactive => "IDP_STATE_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDP_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "IDP_STATE_ACTIVE" => Some(Self::Active),
            "IDP_STATE_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdpStylingType {
    StylingTypeUnspecified = 0,
    StylingTypeGoogle = 1,
}
impl IdpStylingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdpStylingType::StylingTypeUnspecified => "STYLING_TYPE_UNSPECIFIED",
            IdpStylingType::StylingTypeGoogle => "STYLING_TYPE_GOOGLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STYLING_TYPE_UNSPECIFIED" => Some(Self::StylingTypeUnspecified),
            "STYLING_TYPE_GOOGLE" => Some(Self::StylingTypeGoogle),
            _ => None,
        }
    }
}
/// authorization framework of the identity provider
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdpType {
    Unspecified = 0,
    Oidc = 1,
    Jwt = 3,
}
impl IdpType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdpType::Unspecified => "IDP_TYPE_UNSPECIFIED",
            IdpType::Oidc => "IDP_TYPE_OIDC",
            IdpType::Jwt => "IDP_TYPE_JWT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "IDP_TYPE_OIDC" => Some(Self::Oidc),
            "IDP_TYPE_JWT" => Some(Self::Jwt),
            _ => None,
        }
    }
}
/// the owner of the identity provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdpOwnerType {
    Unspecified = 0,
    /// system is managed by the ZITADEL administrators
    System = 1,
    /// org is managed by de organization administrators
    Org = 2,
}
impl IdpOwnerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdpOwnerType::Unspecified => "IDP_OWNER_TYPE_UNSPECIFIED",
            IdpOwnerType::System => "IDP_OWNER_TYPE_SYSTEM",
            IdpOwnerType::Org => "IDP_OWNER_TYPE_ORG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDP_OWNER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "IDP_OWNER_TYPE_SYSTEM" => Some(Self::System),
            "IDP_OWNER_TYPE_ORG" => Some(Self::Org),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OidcMappingField {
    Unspecified = 0,
    PreferredUsername = 1,
    Email = 2,
}
impl OidcMappingField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OidcMappingField::Unspecified => "OIDC_MAPPING_FIELD_UNSPECIFIED",
            OidcMappingField::PreferredUsername => "OIDC_MAPPING_FIELD_PREFERRED_USERNAME",
            OidcMappingField::Email => "OIDC_MAPPING_FIELD_EMAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OIDC_MAPPING_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "OIDC_MAPPING_FIELD_PREFERRED_USERNAME" => Some(Self::PreferredUsername),
            "OIDC_MAPPING_FIELD_EMAIL" => Some(Self::Email),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdpFieldName {
    Unspecified = 0,
    Name = 1,
}
impl IdpFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdpFieldName::Unspecified => "IDP_FIELD_NAME_UNSPECIFIED",
            IdpFieldName::Name => "IDP_FIELD_NAME_NAME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDP_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "IDP_FIELD_NAME_NAME" => Some(Self::Name),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProviderType {
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
    Apple = 11,
    Saml = 12,
}
impl ProviderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProviderType::Unspecified => "PROVIDER_TYPE_UNSPECIFIED",
            ProviderType::Oidc => "PROVIDER_TYPE_OIDC",
            ProviderType::Jwt => "PROVIDER_TYPE_JWT",
            ProviderType::Ldap => "PROVIDER_TYPE_LDAP",
            ProviderType::Oauth => "PROVIDER_TYPE_OAUTH",
            ProviderType::AzureAd => "PROVIDER_TYPE_AZURE_AD",
            ProviderType::Github => "PROVIDER_TYPE_GITHUB",
            ProviderType::GithubEs => "PROVIDER_TYPE_GITHUB_ES",
            ProviderType::Gitlab => "PROVIDER_TYPE_GITLAB",
            ProviderType::GitlabSelfHosted => "PROVIDER_TYPE_GITLAB_SELF_HOSTED",
            ProviderType::Google => "PROVIDER_TYPE_GOOGLE",
            ProviderType::Apple => "PROVIDER_TYPE_APPLE",
            ProviderType::Saml => "PROVIDER_TYPE_SAML",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROVIDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PROVIDER_TYPE_OIDC" => Some(Self::Oidc),
            "PROVIDER_TYPE_JWT" => Some(Self::Jwt),
            "PROVIDER_TYPE_LDAP" => Some(Self::Ldap),
            "PROVIDER_TYPE_OAUTH" => Some(Self::Oauth),
            "PROVIDER_TYPE_AZURE_AD" => Some(Self::AzureAd),
            "PROVIDER_TYPE_GITHUB" => Some(Self::Github),
            "PROVIDER_TYPE_GITHUB_ES" => Some(Self::GithubEs),
            "PROVIDER_TYPE_GITLAB" => Some(Self::Gitlab),
            "PROVIDER_TYPE_GITLAB_SELF_HOSTED" => Some(Self::GitlabSelfHosted),
            "PROVIDER_TYPE_GOOGLE" => Some(Self::Google),
            "PROVIDER_TYPE_APPLE" => Some(Self::Apple),
            "PROVIDER_TYPE_SAML" => Some(Self::Saml),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SamlBinding {
    Unspecified = 0,
    Post = 1,
    Redirect = 2,
    Artifact = 3,
}
impl SamlBinding {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SamlBinding::Unspecified => "SAML_BINDING_UNSPECIFIED",
            SamlBinding::Post => "SAML_BINDING_POST",
            SamlBinding::Redirect => "SAML_BINDING_REDIRECT",
            SamlBinding::Artifact => "SAML_BINDING_ARTIFACT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SAML_BINDING_UNSPECIFIED" => Some(Self::Unspecified),
            "SAML_BINDING_POST" => Some(Self::Post),
            "SAML_BINDING_REDIRECT" => Some(Self::Redirect),
            "SAML_BINDING_ARTIFACT" => Some(Self::Artifact),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SamlNameIdFormat {
    Unspecified = 0,
    EmailAddress = 1,
    Persistent = 2,
    Transient = 3,
}
impl SamlNameIdFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SamlNameIdFormat::Unspecified => "SAML_NAME_ID_FORMAT_UNSPECIFIED",
            SamlNameIdFormat::EmailAddress => "SAML_NAME_ID_FORMAT_EMAIL_ADDRESS",
            SamlNameIdFormat::Persistent => "SAML_NAME_ID_FORMAT_PERSISTENT",
            SamlNameIdFormat::Transient => "SAML_NAME_ID_FORMAT_TRANSIENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SAML_NAME_ID_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "SAML_NAME_ID_FORMAT_EMAIL_ADDRESS" => Some(Self::EmailAddress),
            "SAML_NAME_ID_FORMAT_PERSISTENT" => Some(Self::Persistent),
            "SAML_NAME_ID_FORMAT_TRANSIENT" => Some(Self::Transient),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AutoLinkingOption {
    /// AUTO_LINKING_OPTION_UNSPECIFIED disables the auto linking prompt.
    Unspecified = 0,
    /// AUTO_LINKING_OPTION_USERNAME will use the username of the external user to check for a corresponding ZITADEL user.
    Username = 1,
    /// AUTO_LINKING_OPTION_EMAIL  will use the email of the external user to check for a corresponding ZITADEL user with the same verified email
    /// Note that in case multiple users match, no prompt will be shown.
    Email = 2,
}
impl AutoLinkingOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AutoLinkingOption::Unspecified => "AUTO_LINKING_OPTION_UNSPECIFIED",
            AutoLinkingOption::Username => "AUTO_LINKING_OPTION_USERNAME",
            AutoLinkingOption::Email => "AUTO_LINKING_OPTION_EMAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTO_LINKING_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTO_LINKING_OPTION_USERNAME" => Some(Self::Username),
            "AUTO_LINKING_OPTION_EMAIL" => Some(Self::Email),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AzureAdTenantType {
    Common = 0,
    Organisations = 1,
    Consumers = 2,
}
impl AzureAdTenantType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AzureAdTenantType::Common => "AZURE_AD_TENANT_TYPE_COMMON",
            AzureAdTenantType::Organisations => "AZURE_AD_TENANT_TYPE_ORGANISATIONS",
            AzureAdTenantType::Consumers => "AZURE_AD_TENANT_TYPE_CONSUMERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AZURE_AD_TENANT_TYPE_COMMON" => Some(Self::Common),
            "AZURE_AD_TENANT_TYPE_ORGANISATIONS" => Some(Self::Organisations),
            "AZURE_AD_TENANT_TYPE_CONSUMERS" => Some(Self::Consumers),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
