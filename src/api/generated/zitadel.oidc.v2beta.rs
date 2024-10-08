// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub creation_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub scope: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub redirect_uri: ::prost::alloc::string::String,
    #[prost(enumeration="Prompt", repeated, packed="false", tag="6")]
    pub prompt: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, repeated, tag="7")]
    pub ui_locales: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub login_hint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="9")]
    pub max_age: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(string, optional, tag="10")]
    pub hint_user_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationError {
    #[prost(enumeration="ErrorReason", tag="1")]
    pub error: i32,
    #[prost(string, optional, tag="2")]
    pub error_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub error_uri: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Prompt {
    Unspecified = 0,
    None = 1,
    Login = 2,
    Consent = 3,
    SelectAccount = 4,
    Create = 5,
}
impl Prompt {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Prompt::Unspecified => "PROMPT_UNSPECIFIED",
            Prompt::None => "PROMPT_NONE",
            Prompt::Login => "PROMPT_LOGIN",
            Prompt::Consent => "PROMPT_CONSENT",
            Prompt::SelectAccount => "PROMPT_SELECT_ACCOUNT",
            Prompt::Create => "PROMPT_CREATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROMPT_UNSPECIFIED" => Some(Self::Unspecified),
            "PROMPT_NONE" => Some(Self::None),
            "PROMPT_LOGIN" => Some(Self::Login),
            "PROMPT_CONSENT" => Some(Self::Consent),
            "PROMPT_SELECT_ACCOUNT" => Some(Self::SelectAccount),
            "PROMPT_CREATE" => Some(Self::Create),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorReason {
    Unspecified = 0,
    /// Error states from <https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1>
    InvalidRequest = 1,
    UnauthorizedClient = 2,
    AccessDenied = 3,
    UnsupportedResponseType = 4,
    InvalidScope = 5,
    ServerError = 6,
    TemporaryUnavailable = 7,
    /// Error states from <https://openid.net/specs/openid-connect-core-1_0.html#AuthError>
    InteractionRequired = 8,
    LoginRequired = 9,
    AccountSelectionRequired = 10,
    ConsentRequired = 11,
    InvalidRequestUri = 12,
    InvalidRequestObject = 13,
    RequestNotSupported = 14,
    RequestUriNotSupported = 15,
    RegistrationNotSupported = 16,
}
impl ErrorReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorReason::Unspecified => "ERROR_REASON_UNSPECIFIED",
            ErrorReason::InvalidRequest => "ERROR_REASON_INVALID_REQUEST",
            ErrorReason::UnauthorizedClient => "ERROR_REASON_UNAUTHORIZED_CLIENT",
            ErrorReason::AccessDenied => "ERROR_REASON_ACCESS_DENIED",
            ErrorReason::UnsupportedResponseType => "ERROR_REASON_UNSUPPORTED_RESPONSE_TYPE",
            ErrorReason::InvalidScope => "ERROR_REASON_INVALID_SCOPE",
            ErrorReason::ServerError => "ERROR_REASON_SERVER_ERROR",
            ErrorReason::TemporaryUnavailable => "ERROR_REASON_TEMPORARY_UNAVAILABLE",
            ErrorReason::InteractionRequired => "ERROR_REASON_INTERACTION_REQUIRED",
            ErrorReason::LoginRequired => "ERROR_REASON_LOGIN_REQUIRED",
            ErrorReason::AccountSelectionRequired => "ERROR_REASON_ACCOUNT_SELECTION_REQUIRED",
            ErrorReason::ConsentRequired => "ERROR_REASON_CONSENT_REQUIRED",
            ErrorReason::InvalidRequestUri => "ERROR_REASON_INVALID_REQUEST_URI",
            ErrorReason::InvalidRequestObject => "ERROR_REASON_INVALID_REQUEST_OBJECT",
            ErrorReason::RequestNotSupported => "ERROR_REASON_REQUEST_NOT_SUPPORTED",
            ErrorReason::RequestUriNotSupported => "ERROR_REASON_REQUEST_URI_NOT_SUPPORTED",
            ErrorReason::RegistrationNotSupported => "ERROR_REASON_REGISTRATION_NOT_SUPPORTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERROR_REASON_UNSPECIFIED" => Some(Self::Unspecified),
            "ERROR_REASON_INVALID_REQUEST" => Some(Self::InvalidRequest),
            "ERROR_REASON_UNAUTHORIZED_CLIENT" => Some(Self::UnauthorizedClient),
            "ERROR_REASON_ACCESS_DENIED" => Some(Self::AccessDenied),
            "ERROR_REASON_UNSUPPORTED_RESPONSE_TYPE" => Some(Self::UnsupportedResponseType),
            "ERROR_REASON_INVALID_SCOPE" => Some(Self::InvalidScope),
            "ERROR_REASON_SERVER_ERROR" => Some(Self::ServerError),
            "ERROR_REASON_TEMPORARY_UNAVAILABLE" => Some(Self::TemporaryUnavailable),
            "ERROR_REASON_INTERACTION_REQUIRED" => Some(Self::InteractionRequired),
            "ERROR_REASON_LOGIN_REQUIRED" => Some(Self::LoginRequired),
            "ERROR_REASON_ACCOUNT_SELECTION_REQUIRED" => Some(Self::AccountSelectionRequired),
            "ERROR_REASON_CONSENT_REQUIRED" => Some(Self::ConsentRequired),
            "ERROR_REASON_INVALID_REQUEST_URI" => Some(Self::InvalidRequestUri),
            "ERROR_REASON_INVALID_REQUEST_OBJECT" => Some(Self::InvalidRequestObject),
            "ERROR_REASON_REQUEST_NOT_SUPPORTED" => Some(Self::RequestNotSupported),
            "ERROR_REASON_REQUEST_URI_NOT_SUPPORTED" => Some(Self::RequestUriNotSupported),
            "ERROR_REASON_REGISTRATION_NOT_SUPPORTED" => Some(Self::RegistrationNotSupported),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthRequestRequest {
    #[prost(string, tag="1")]
    pub auth_request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthRequestResponse {
    #[prost(message, optional, tag="1")]
    pub auth_request: ::core::option::Option<AuthRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCallbackRequest {
    #[prost(string, tag="1")]
    pub auth_request_id: ::prost::alloc::string::String,
    #[prost(oneof="create_callback_request::CallbackKind", tags="2, 3")]
    pub callback_kind: ::core::option::Option<create_callback_request::CallbackKind>,
}
/// Nested message and enum types in `CreateCallbackRequest`.
pub mod create_callback_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CallbackKind {
        #[prost(message, tag="2")]
        Session(super::Session),
        #[prost(message, tag="3")]
        Error(super::AuthorizationError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub session_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCallbackResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, tag="2")]
    pub callback_url: ::prost::alloc::string::String,
}
include!("zitadel.oidc.v2beta.tonic.rs");
// @@protoc_insertion_point(module)