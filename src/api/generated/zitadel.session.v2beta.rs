// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestChallenges {
    #[prost(message, optional, tag="1")]
    pub web_auth_n: ::core::option::Option<request_challenges::WebAuthN>,
    #[prost(message, optional, tag="2")]
    pub otp_sms: ::core::option::Option<request_challenges::Otpsms>,
    #[prost(message, optional, tag="3")]
    pub otp_email: ::core::option::Option<request_challenges::OtpEmail>,
}
/// Nested message and enum types in `RequestChallenges`.
pub mod request_challenges {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebAuthN {
        #[prost(string, tag="1")]
        pub domain: ::prost::alloc::string::String,
        #[prost(enumeration="super::UserVerificationRequirement", tag="2")]
        pub user_verification_requirement: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Otpsms {
        #[prost(bool, tag="1")]
        pub return_code: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OtpEmail {
        /// if no delivery_type is specified, an email is sent with the default url
        #[prost(oneof="otp_email::DeliveryType", tags="2, 3")]
        pub delivery_type: ::core::option::Option<otp_email::DeliveryType>,
    }
    /// Nested message and enum types in `OTPEmail`.
    pub mod otp_email {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SendCode {
            #[prost(string, optional, tag="1")]
            pub url_template: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ReturnCode {
        }
        /// if no delivery_type is specified, an email is sent with the default url
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DeliveryType {
            #[prost(message, tag="2")]
            SendCode(SendCode),
            #[prost(message, tag="3")]
            ReturnCode(ReturnCode),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Challenges {
    #[prost(message, optional, tag="1")]
    pub web_auth_n: ::core::option::Option<challenges::WebAuthN>,
    #[prost(string, optional, tag="2")]
    pub otp_sms: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub otp_email: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Challenges`.
pub mod challenges {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebAuthN {
        #[prost(message, optional, tag="1")]
        pub public_key_credential_request_options: ::core::option::Option<::pbjson_types::Struct>,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserVerificationRequirement {
    Unspecified = 0,
    Required = 1,
    Preferred = 2,
    Discouraged = 3,
}
impl UserVerificationRequirement {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserVerificationRequirement::Unspecified => "USER_VERIFICATION_REQUIREMENT_UNSPECIFIED",
            UserVerificationRequirement::Required => "USER_VERIFICATION_REQUIREMENT_REQUIRED",
            UserVerificationRequirement::Preferred => "USER_VERIFICATION_REQUIREMENT_PREFERRED",
            UserVerificationRequirement::Discouraged => "USER_VERIFICATION_REQUIREMENT_DISCOURAGED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_VERIFICATION_REQUIREMENT_UNSPECIFIED" => Some(Self::Unspecified),
            "USER_VERIFICATION_REQUIREMENT_REQUIRED" => Some(Self::Required),
            "USER_VERIFICATION_REQUIREMENT_PREFERRED" => Some(Self::Preferred),
            "USER_VERIFICATION_REQUIREMENT_DISCOURAGED" => Some(Self::Discouraged),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub creation_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub change_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub sequence: u64,
    #[prost(message, optional, tag="5")]
    pub factors: ::core::option::Option<Factors>,
    #[prost(map="string, bytes", tag="6")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="7")]
    pub user_agent: ::core::option::Option<UserAgent>,
    #[prost(message, optional, tag="8")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Factors {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<UserFactor>,
    #[prost(message, optional, tag="2")]
    pub password: ::core::option::Option<PasswordFactor>,
    #[prost(message, optional, tag="3")]
    pub web_auth_n: ::core::option::Option<WebAuthNFactor>,
    #[prost(message, optional, tag="4")]
    pub intent: ::core::option::Option<IntentFactor>,
    #[prost(message, optional, tag="5")]
    pub totp: ::core::option::Option<TotpFactor>,
    #[prost(message, optional, tag="6")]
    pub otp_sms: ::core::option::Option<OtpFactor>,
    #[prost(message, optional, tag="7")]
    pub otp_email: ::core::option::Option<OtpFactor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserFactor {
    #[prost(message, optional, tag="1")]
    pub verified_at: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub login_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordFactor {
    #[prost(message, optional, tag="1")]
    pub verified_at: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentFactor {
    #[prost(message, optional, tag="1")]
    pub verified_at: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebAuthNFactor {
    #[prost(message, optional, tag="1")]
    pub verified_at: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bool, tag="2")]
    pub user_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotpFactor {
    #[prost(message, optional, tag="1")]
    pub verified_at: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OtpFactor {
    #[prost(message, optional, tag="1")]
    pub verified_at: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    #[prost(oneof="search_query::Query", tags="1, 2, 3")]
    pub query: ::core::option::Option<search_query::Query>,
}
/// Nested message and enum types in `SearchQuery`.
pub mod search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        IdsQuery(super::IDsQuery),
        #[prost(message, tag="2")]
        UserIdQuery(super::UserIdQuery),
        #[prost(message, tag="3")]
        CreationDateQuery(super::CreationDateQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IDsQuery {
    #[prost(string, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdQuery {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreationDateQuery {
    #[prost(message, optional, tag="1")]
    pub creation_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(enumeration="super::super::v1::TimestampQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAgent {
    #[prost(string, optional, tag="1")]
    pub fingerprint_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(map="string, message", tag="4")]
    pub header: ::std::collections::HashMap<::prost::alloc::string::String, user_agent::HeaderValues>,
}
/// Nested message and enum types in `UserAgent`.
pub mod user_agent {
    /// A header may have multiple values.
    /// In Go, headers are defined
    /// as map[string][]string, but protobuf
    /// doesn't allow this scheme.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderValues {
        #[prost(string, repeated, tag="1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionFieldName {
    Unspecified = 0,
    CreationDate = 1,
}
impl SessionFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SessionFieldName::Unspecified => "SESSION_FIELD_NAME_UNSPECIFIED",
            SessionFieldName::CreationDate => "SESSION_FIELD_NAME_CREATION_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SESSION_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "SESSION_FIELD_NAME_CREATION_DATE" => Some(Self::CreationDate),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionsRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::object::v2beta::ListQuery>,
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
    #[prost(enumeration="SessionFieldName", tag="3")]
    pub sorting_column: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub sessions: ::prost::alloc::vec::Vec<Session>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionRequest {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub session_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionResponse {
    #[prost(message, optional, tag="1")]
    pub session: ::core::option::Option<Session>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionRequest {
    #[prost(message, optional, tag="1")]
    pub checks: ::core::option::Option<Checks>,
    #[prost(map="string, bytes", tag="2")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="3")]
    pub challenges: ::core::option::Option<RequestChallenges>,
    #[prost(message, optional, tag="4")]
    pub user_agent: ::core::option::Option<UserAgent>,
    #[prost(message, optional, tag="5")]
    pub lifetime: ::core::option::Option<::pbjson_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, tag="2")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub session_token: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub challenges: ::core::option::Option<Challenges>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSessionRequest {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub session_token: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub checks: ::core::option::Option<Checks>,
    #[prost(map="string, bytes", tag="4")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="5")]
    pub challenges: ::core::option::Option<RequestChallenges>,
    #[prost(message, optional, tag="6")]
    pub lifetime: ::core::option::Option<::pbjson_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSessionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
    #[prost(string, tag="2")]
    pub session_token: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub challenges: ::core::option::Option<Challenges>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionRequest {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub session_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2beta::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checks {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<CheckUser>,
    #[prost(message, optional, tag="2")]
    pub password: ::core::option::Option<CheckPassword>,
    #[prost(message, optional, tag="3")]
    pub web_auth_n: ::core::option::Option<CheckWebAuthN>,
    #[prost(message, optional, tag="4")]
    pub idp_intent: ::core::option::Option<CheckIdpIntent>,
    #[prost(message, optional, tag="5")]
    pub totp: ::core::option::Option<CheckTotp>,
    #[prost(message, optional, tag="6")]
    pub otp_sms: ::core::option::Option<CheckOtp>,
    #[prost(message, optional, tag="7")]
    pub otp_email: ::core::option::Option<CheckOtp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUser {
    #[prost(oneof="check_user::Search", tags="1, 2")]
    pub search: ::core::option::Option<check_user::Search>,
}
/// Nested message and enum types in `CheckUser`.
pub mod check_user {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Search {
        #[prost(string, tag="1")]
        UserId(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        LoginName(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPassword {
    #[prost(string, tag="1")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckWebAuthN {
    #[prost(message, optional, tag="1")]
    pub credential_assertion_data: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckIdpIntent {
    #[prost(string, tag="1")]
    pub idp_intent_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub idp_intent_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckTotp {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckOtp {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
}
include!("zitadel.session.v2beta.tonic.rs");
// @@protoc_insertion_point(module)