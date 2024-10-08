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
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupportedLanguagesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupportedLanguagesResponse {
    #[prost(string, repeated, tag="1")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This is an empty request
/// the request parameters are read from the token-header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyUserRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyUserResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<super::super::user::v1::User>,
    #[prost(message, optional, tag="2")]
    pub last_login: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// This is an empty request
/// the request parameters are read from the token-header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyUserRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyUserChangesRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::change::v1::ChangeQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyUserChangesResponse {
    /// zitadel.v1.ListDetails details = 1; was always returned empty (as we cannot get the necessary info)
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::change::v1::Change>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyUserSessionsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyUserSessionsResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::Session>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyMetadataRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::metadata::v1::MetadataQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::metadata::v1::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyMetadataRequest {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::metadata::v1::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMyMetadataRequest {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMyMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkSetMyMetadataRequest {
    #[prost(message, repeated, tag="1")]
    pub metadata: ::prost::alloc::vec::Vec<bulk_set_my_metadata_request::Metadata>,
}
/// Nested message and enum types in `BulkSetMyMetadataRequest`.
pub mod bulk_set_my_metadata_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metadata {
        #[prost(string, tag="1")]
        pub key: ::prost::alloc::string::String,
        #[prost(bytes="vec", tag="2")]
        pub value: ::prost::alloc::vec::Vec<u8>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkSetMyMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyMetadataRequest {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkRemoveMyMetadataRequest {
    #[prost(string, repeated, tag="1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkRemoveMyMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyRefreshTokensRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyRefreshTokensResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::RefreshToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeMyRefreshTokenRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeMyRefreshTokenResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeAllMyRefreshTokensRequest {
}
/// This is an empty response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeAllMyRefreshTokensResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMyUserNameRequest {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMyUserNameResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyPasswordComplexityPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyPasswordComplexityPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::PasswordComplexityPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMyPasswordRequest {
    #[prost(string, tag="1")]
    pub old_password: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMyPasswordResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyProfileRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyProfileResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="2")]
    pub profile: ::core::option::Option<super::super::user::v1::Profile>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMyProfileRequest {
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
    #[prost(enumeration="super::super::user::v1::Gender", tag="6")]
    pub gender: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMyProfileResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyEmailRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="2")]
    pub email: ::core::option::Option<super::super::user::v1::Email>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMyEmailRequest {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMyEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyEmailRequest {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendMyEmailVerificationRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendMyEmailVerificationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyPhoneRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="2")]
    pub phone: ::core::option::Option<super::super::user::v1::Phone>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMyPhoneRequest {
    #[prost(string, tag="1")]
    pub phone: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMyPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyPhoneRequest {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendMyPhoneVerificationRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendMyPhoneVerificationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyPhoneRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAvatarRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAvatarResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyLinkedIdPsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyLinkedIdPsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::idp::v1::IdpUserLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyLinkedIdpRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub linked_user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyLinkedIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyAuthFactorsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyAuthFactorsResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::AuthFactor>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyAuthFactorU2fRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyAuthFactorU2fResponse {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<super::super::user::v1::WebAuthNKey>,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyAuthFactorOtpRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyAuthFactorOtpResponse {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyAuthFactorOtpRequest {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyAuthFactorOtpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyAuthFactorU2fRequest {
    #[prost(message, optional, tag="1")]
    pub verification: ::core::option::Option<super::super::user::v1::WebAuthNVerification>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyAuthFactorU2fResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAuthFactorOtpRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAuthFactorOtpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyAuthFactorOtpsmsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyAuthFactorOtpsmsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAuthFactorOtpsmsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAuthFactorOtpsmsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyAuthFactorOtpEmailRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyAuthFactorOtpEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAuthFactorOtpEmailRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAuthFactorOtpEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAuthFactorU2fRequest {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyAuthFactorU2fResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyPasswordlessRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyPasswordlessResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::WebAuthNToken>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyPasswordlessRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyPasswordlessResponse {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<super::super::user::v1::WebAuthNKey>,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyPasswordlessLinkRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMyPasswordlessLinkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub link: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub expiration: ::core::option::Option<::pbjson_types::Duration>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMyPasswordlessLinkRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMyPasswordlessLinkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyPasswordlessRequest {
    #[prost(message, optional, tag="1")]
    pub verification: ::core::option::Option<super::super::user::v1::WebAuthNVerification>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMyPasswordlessResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyPasswordlessRequest {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMyPasswordlessResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyUserGrantsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyUserGrantsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<UserGrant>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrant {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
    /// Deprecated: user role_keys
    #[prost(string, repeated, tag="4")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub org_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="8")]
    pub org_domain: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub project_grant_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="11")]
    pub role_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::user::v1::Type", tag="12")]
    pub user_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyProjectOrgsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::org::v1::OrgQuery>,
    /// States by which field the results are sorted.
    #[prost(enumeration="super::super::org::v1::OrgFieldName", tag="3")]
    pub sorting_column: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyProjectOrgsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::org::v1::Org>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyZitadelPermissionsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyZitadelPermissionsResponse {
    #[prost(string, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyProjectPermissionsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyProjectPermissionsResponse {
    #[prost(string, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyMembershipsRequest {
    /// the field the result is sorted
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::user::v1::MembershipQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMyMembershipsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::Membership>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyLabelPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LabelPolicy>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyPrivacyPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyPrivacyPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::PrivacyPolicy>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyLoginPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LoginPolicy>,
}
include!("zitadel.auth.v1.tonic.rs");
// @@protoc_insertion_point(module)