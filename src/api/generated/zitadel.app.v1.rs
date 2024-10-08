// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct App {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="AppState", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="app::Config", tags="5, 6, 7")]
    pub config: ::core::option::Option<app::Config>,
}
/// Nested message and enum types in `App`.
pub mod app {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag="5")]
        OidcConfig(super::OidcConfig),
        #[prost(message, tag="6")]
        ApiConfig(super::ApiConfig),
        #[prost(message, tag="7")]
        SamlConfig(super::SamlConfig),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppQuery {
    #[prost(oneof="app_query::Query", tags="1")]
    pub query: ::core::option::Option<app_query::Query>,
}
/// Nested message and enum types in `AppQuery`.
pub mod app_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        NameQuery(super::AppNameQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppNameQuery {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OidcConfig {
    #[prost(string, repeated, tag="1")]
    pub redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="OidcResponseType", repeated, packed="false", tag="2")]
    pub response_types: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="OidcGrantType", repeated, packed="false", tag="3")]
    pub grant_types: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="OidcAppType", tag="4")]
    pub app_type: i32,
    #[prost(string, tag="5")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(enumeration="OidcAuthMethodType", tag="7")]
    pub auth_method_type: i32,
    #[prost(string, repeated, tag="8")]
    pub post_logout_redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="OidcVersion", tag="9")]
    pub version: i32,
    #[prost(bool, tag="10")]
    pub none_compliant: bool,
    #[prost(message, repeated, tag="11")]
    pub compliance_problems: ::prost::alloc::vec::Vec<super::super::v1::LocalizedMessage>,
    #[prost(bool, tag="12")]
    pub dev_mode: bool,
    #[prost(enumeration="OidcTokenType", tag="13")]
    pub access_token_type: i32,
    #[prost(bool, tag="14")]
    pub access_token_role_assertion: bool,
    #[prost(bool, tag="15")]
    pub id_token_role_assertion: bool,
    #[prost(bool, tag="16")]
    pub id_token_userinfo_assertion: bool,
    #[prost(message, optional, tag="17")]
    pub clock_skew: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(string, repeated, tag="18")]
    pub additional_origins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="19")]
    pub allowed_origins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="20")]
    pub skip_native_app_success_page: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamlConfig {
    #[prost(oneof="saml_config::Metadata", tags="1, 2")]
    pub metadata: ::core::option::Option<saml_config::Metadata>,
}
/// Nested message and enum types in `SAMLConfig`.
pub mod saml_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(bytes, tag="1")]
        MetadataXml(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag="2")]
        MetadataUrl(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiConfig {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(enumeration="ApiAuthMethodType", tag="3")]
    pub auth_method_type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AppState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
}
impl AppState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AppState::Unspecified => "APP_STATE_UNSPECIFIED",
            AppState::Active => "APP_STATE_ACTIVE",
            AppState::Inactive => "APP_STATE_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "APP_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "APP_STATE_ACTIVE" => Some(Self::Active),
            "APP_STATE_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OidcResponseType {
    Code = 0,
    IdToken = 1,
    IdTokenToken = 2,
}
impl OidcResponseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OidcResponseType::Code => "OIDC_RESPONSE_TYPE_CODE",
            OidcResponseType::IdToken => "OIDC_RESPONSE_TYPE_ID_TOKEN",
            OidcResponseType::IdTokenToken => "OIDC_RESPONSE_TYPE_ID_TOKEN_TOKEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OIDC_RESPONSE_TYPE_CODE" => Some(Self::Code),
            "OIDC_RESPONSE_TYPE_ID_TOKEN" => Some(Self::IdToken),
            "OIDC_RESPONSE_TYPE_ID_TOKEN_TOKEN" => Some(Self::IdTokenToken),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OidcGrantType {
    AuthorizationCode = 0,
    Implicit = 1,
    RefreshToken = 2,
    DeviceCode = 3,
    TokenExchange = 4,
}
impl OidcGrantType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OidcGrantType::AuthorizationCode => "OIDC_GRANT_TYPE_AUTHORIZATION_CODE",
            OidcGrantType::Implicit => "OIDC_GRANT_TYPE_IMPLICIT",
            OidcGrantType::RefreshToken => "OIDC_GRANT_TYPE_REFRESH_TOKEN",
            OidcGrantType::DeviceCode => "OIDC_GRANT_TYPE_DEVICE_CODE",
            OidcGrantType::TokenExchange => "OIDC_GRANT_TYPE_TOKEN_EXCHANGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OIDC_GRANT_TYPE_AUTHORIZATION_CODE" => Some(Self::AuthorizationCode),
            "OIDC_GRANT_TYPE_IMPLICIT" => Some(Self::Implicit),
            "OIDC_GRANT_TYPE_REFRESH_TOKEN" => Some(Self::RefreshToken),
            "OIDC_GRANT_TYPE_DEVICE_CODE" => Some(Self::DeviceCode),
            "OIDC_GRANT_TYPE_TOKEN_EXCHANGE" => Some(Self::TokenExchange),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OidcAppType {
    Web = 0,
    UserAgent = 1,
    Native = 2,
}
impl OidcAppType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OidcAppType::Web => "OIDC_APP_TYPE_WEB",
            OidcAppType::UserAgent => "OIDC_APP_TYPE_USER_AGENT",
            OidcAppType::Native => "OIDC_APP_TYPE_NATIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OIDC_APP_TYPE_WEB" => Some(Self::Web),
            "OIDC_APP_TYPE_USER_AGENT" => Some(Self::UserAgent),
            "OIDC_APP_TYPE_NATIVE" => Some(Self::Native),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OidcAuthMethodType {
    Basic = 0,
    Post = 1,
    None = 2,
    PrivateKeyJwt = 3,
}
impl OidcAuthMethodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OidcAuthMethodType::Basic => "OIDC_AUTH_METHOD_TYPE_BASIC",
            OidcAuthMethodType::Post => "OIDC_AUTH_METHOD_TYPE_POST",
            OidcAuthMethodType::None => "OIDC_AUTH_METHOD_TYPE_NONE",
            OidcAuthMethodType::PrivateKeyJwt => "OIDC_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OIDC_AUTH_METHOD_TYPE_BASIC" => Some(Self::Basic),
            "OIDC_AUTH_METHOD_TYPE_POST" => Some(Self::Post),
            "OIDC_AUTH_METHOD_TYPE_NONE" => Some(Self::None),
            "OIDC_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT" => Some(Self::PrivateKeyJwt),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OidcVersion {
    OidcVersion10 = 0,
}
impl OidcVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OidcVersion::OidcVersion10 => "OIDC_VERSION_1_0",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OIDC_VERSION_1_0" => Some(Self::OidcVersion10),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OidcTokenType {
    Bearer = 0,
    Jwt = 1,
}
impl OidcTokenType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OidcTokenType::Bearer => "OIDC_TOKEN_TYPE_BEARER",
            OidcTokenType::Jwt => "OIDC_TOKEN_TYPE_JWT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OIDC_TOKEN_TYPE_BEARER" => Some(Self::Bearer),
            "OIDC_TOKEN_TYPE_JWT" => Some(Self::Jwt),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiAuthMethodType {
    Basic = 0,
    PrivateKeyJwt = 1,
}
impl ApiAuthMethodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApiAuthMethodType::Basic => "API_AUTH_METHOD_TYPE_BASIC",
            ApiAuthMethodType::PrivateKeyJwt => "API_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "API_AUTH_METHOD_TYPE_BASIC" => Some(Self::Basic),
            "API_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT" => Some(Self::PrivateKeyJwt),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
