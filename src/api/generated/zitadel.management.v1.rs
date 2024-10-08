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
pub struct GetOidcInformationRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOidcInformationResponse {
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub discovery_endpoint: ::prost::alloc::string::String,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIamRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIamResponse {
    /// deprecated: use default_org_id instead
    #[prost(string, tag="1")]
    pub global_org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub iam_project_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub default_org_id: ::prost::alloc::string::String,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByIdResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<super::super::user::v1::User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByLoginNameGlobalRequest {
    #[prost(string, tag="1")]
    pub login_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByLoginNameGlobalResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<super::super::user::v1::User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::user::v1::UserFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::user::v1::SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::user::v1::UserFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserChangesRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::change::v1::ChangeQuery>,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserChangesResponse {
    /// zitadel.v1.ListDetails details = 1; was always returned empty (as we cannot get the necessary info)
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::change::v1::Change>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsUserUniqueRequest {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsUserUniqueResponse {
    #[prost(bool, tag="1")]
    pub is_unique: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHumanUserRequest {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub profile: ::core::option::Option<add_human_user_request::Profile>,
    #[prost(message, optional, tag="3")]
    pub email: ::core::option::Option<add_human_user_request::Email>,
    #[prost(message, optional, tag="4")]
    pub phone: ::core::option::Option<add_human_user_request::Phone>,
    #[prost(string, tag="5")]
    pub initial_password: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AddHumanUserRequest`.
pub mod add_human_user_request {
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHumanUserResponse {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// Describe my Import Human User Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportHumanUserRequest {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub profile: ::core::option::Option<import_human_user_request::Profile>,
    #[prost(message, optional, tag="3")]
    pub email: ::core::option::Option<import_human_user_request::Email>,
    #[prost(message, optional, tag="4")]
    pub phone: ::core::option::Option<import_human_user_request::Phone>,
    #[prost(string, tag="5")]
    pub password: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub hashed_password: ::core::option::Option<import_human_user_request::HashedPassword>,
    #[prost(bool, tag="7")]
    pub password_change_required: bool,
    #[prost(bool, tag="8")]
    pub request_passwordless_registration: bool,
    #[prost(string, tag="9")]
    pub otp_code: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="10")]
    pub idps: ::prost::alloc::vec::Vec<import_human_user_request::Idp>,
}
/// Nested message and enum types in `ImportHumanUserRequest`.
pub mod import_human_user_request {
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
        #[prost(string, tag="1")]
        pub email: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub is_email_verified: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Phone {
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
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Idp {
        #[prost(string, tag="1")]
        pub config_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub external_user_id: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub display_name: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportHumanUserResponse {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="3")]
    pub passwordless_registration: ::core::option::Option<import_human_user_response::PasswordlessRegistration>,
}
/// Nested message and enum types in `ImportHumanUserResponse`.
pub mod import_human_user_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PasswordlessRegistration {
        #[prost(string, tag="1")]
        pub link: ::prost::alloc::string::String,
        /// deprecated: use expiration instead
        #[prost(message, optional, tag="2")]
        pub lifetime: ::core::option::Option<::pbjson_types::Duration>,
        #[prost(message, optional, tag="3")]
        pub expiration: ::core::option::Option<::pbjson_types::Duration>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMachineUserRequest {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::user::v1::AccessTokenType", tag="4")]
    pub access_token_type: i32,
    /// optionally set your own id unique for the user.
    #[prost(string, optional, tag="5")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMachineUserResponse {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockUserRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockUserRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserNameRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserNameResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::metadata::v1::MetadataQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::metadata::v1::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::metadata::v1::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUserMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUserMetadataResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkSetUserMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub metadata: ::prost::alloc::vec::Vec<bulk_set_user_metadata_request::Metadata>,
}
/// Nested message and enum types in `BulkSetUserMetadataRequest`.
pub mod bulk_set_user_metadata_request {
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
pub struct BulkSetUserMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkRemoveUserMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkRemoveUserMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHumanProfileRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHumanProfileResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="2")]
    pub profile: ::core::option::Option<super::super::user::v1::Profile>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanProfileRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub nick_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub preferred_language: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::user::v1::Gender", tag="7")]
    pub gender: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanProfileResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHumanEmailRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHumanEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="2")]
    pub email: ::core::option::Option<super::super::user::v1::Email>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanEmailRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_email_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendHumanInitializationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendHumanInitializationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendHumanEmailVerificationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendHumanEmailVerificationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHumanPhoneRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHumanPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="2")]
    pub phone: ::core::option::Option<super::super::user::v1::Phone>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanPhoneRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub phone: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_phone_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanPhoneRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendHumanPhoneVerificationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendHumanPhoneVerificationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAvatarRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAvatarResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHumanInitialPasswordRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHumanInitialPasswordResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHumanPasswordRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub no_change_required: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHumanPasswordResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendHumanResetPasswordNotificationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration="send_human_reset_password_notification_request::Type", tag="2")]
    pub r#type: i32,
}
/// Nested message and enum types in `SendHumanResetPasswordNotificationRequest`.
pub mod send_human_reset_password_notification_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Email = 0,
        Sms = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Email => "TYPE_EMAIL",
                Type::Sms => "TYPE_SMS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_EMAIL" => Some(Self::Email),
                "TYPE_SMS" => Some(Self::Sms),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendHumanResetPasswordNotificationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHumanAuthFactorsRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHumanAuthFactorsResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::AuthFactor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAuthFactorOtpRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAuthFactorOtpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAuthFactorU2fRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAuthFactorU2fResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAuthFactorOtpsmsRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAuthFactorOtpsmsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAuthFactorOtpEmailRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanAuthFactorOtpEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHumanPasswordlessRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHumanPasswordlessResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::WebAuthNToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPasswordlessRegistrationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPasswordlessRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub link: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub expiration: ::core::option::Option<::pbjson_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPasswordlessRegistrationRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPasswordlessRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanPasswordlessRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanPasswordlessResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMachineRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::user::v1::AccessTokenType", tag="4")]
    pub access_token_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMachineResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMachineSecretRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMachineSecretResponse {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMachineSecretRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMachineSecretResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMachineKeyByIDsRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMachineKeyByIDsResponse {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<super::super::authn::v1::Key>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMachineKeysRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMachineKeysResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::authn::v1::Key>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMachineKeyRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::authn::v1::KeyType", tag="2")]
    pub r#type: i32,
    #[prost(message, optional, tag="3")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bytes="vec", tag="4")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMachineKeyResponse {
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub key_details: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMachineKeyRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMachineKeyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPersonalAccessTokenByIDsRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPersonalAccessTokenByIDsResponse {
    #[prost(message, optional, tag="1")]
    pub token: ::core::option::Option<super::super::user::v1::PersonalAccessToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPersonalAccessTokensRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPersonalAccessTokensResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::PersonalAccessToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPersonalAccessTokenRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPersonalAccessTokenResponse {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePersonalAccessTokenRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePersonalAccessTokenResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHumanLinkedIdPsRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHumanLinkedIdPsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::idp::v1::IdpUserLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanLinkedIdpRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub linked_user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHumanLinkedIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserMembershipsRequest {
    /// list limitations and ordering
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// the field the result is sorted
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::user::v1::MembershipQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserMembershipsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::Membership>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyOrgRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyOrgResponse {
    #[prost(message, optional, tag="1")]
    pub org: ::core::option::Option<super::super::org::v1::Org>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgByDomainGlobalRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgChangesRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::change::v1::ChangeQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgChangesResponse {
    /// zitadel.v1.ListDetails details = 1; was always returned empty (as we cannot get the necessary info)
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::change::v1::Change>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgByDomainGlobalResponse {
    #[prost(message, optional, tag="1")]
    pub org: ::core::option::Option<super::super::org::v1::Org>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateOrgRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateOrgResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateOrgRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateOrgResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgDomainsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::org::v1::DomainSearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgDomainsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::org::v1::Domain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgDomainRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgDomainRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateOrgDomainValidationRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::org::v1::DomainValidationType", tag="2")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateOrgDomainValidationResponse {
    #[prost(string, tag="1")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateOrgDomainRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateOrgDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPrimaryOrgDomainRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPrimaryOrgDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgMemberRolesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgMemberRolesResponse {
    #[prost(string, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgMembersRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::member::v1::SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgMembersResponse {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::member::v1::Member>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgMemberRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgMemberRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgMemberRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgMetadataRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::metadata::v1::MetadataQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::metadata::v1::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgMetadataRequest {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::metadata::v1::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrgMetadataRequest {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrgMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkSetOrgMetadataRequest {
    #[prost(message, repeated, tag="1")]
    pub metadata: ::prost::alloc::vec::Vec<bulk_set_org_metadata_request::Metadata>,
}
/// Nested message and enum types in `BulkSetOrgMetadataRequest`.
pub mod bulk_set_org_metadata_request {
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
pub struct BulkSetOrgMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgMetadataRequest {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkRemoveOrgMetadataRequest {
    #[prost(string, repeated, tag="1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkRemoveOrgMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectByIdResponse {
    #[prost(message, optional, tag="1")]
    pub project: ::core::option::Option<super::super::project::v1::Project>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGrantedProjectByIdRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGrantedProjectByIdResponse {
    #[prost(message, optional, tag="1")]
    pub granted_project: ::core::option::Option<super::super::project::v1::GrantedProject>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::project::v1::ProjectQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::project::v1::Project>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGrantedProjectsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::project::v1::ProjectQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGrantedProjectsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::project::v1::GrantedProject>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectChangesRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::change::v1::ChangeQuery>,
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectChangesResponse {
    /// zitadel.v1.ListDetails details = 1; was always returned empty (as we cannot get the necessary info)
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::change::v1::Change>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub project_role_assertion: bool,
    #[prost(bool, tag="3")]
    pub project_role_check: bool,
    #[prost(bool, tag="4")]
    pub has_project_check: bool,
    #[prost(enumeration="super::super::project::v1::PrivateLabelingSetting", tag="5")]
    pub private_labeling_setting: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub project_role_assertion: bool,
    #[prost(bool, tag="4")]
    pub project_role_check: bool,
    #[prost(bool, tag="5")]
    pub has_project_check: bool,
    #[prost(enumeration="super::super::project::v1::PrivateLabelingSetting", tag="6")]
    pub private_labeling_setting: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateProjectRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateProjectResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateProjectRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateProjectResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectMemberRolesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectMemberRolesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(string, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectRoleRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub role_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub group: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectRoleResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkAddProjectRolesRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<bulk_add_project_roles_request::Role>,
}
/// Nested message and enum types in `BulkAddProjectRolesRequest`.
pub mod bulk_add_project_roles_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Role {
        #[prost(string, tag="1")]
        pub key: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub display_name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub group: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkAddProjectRolesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectRoleRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub role_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub group: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectRoleResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectRoleRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub role_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectRoleResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectRolesRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::project::v1::RoleQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectRolesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::project::v1::Role>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGrantedProjectRolesRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="3")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="4")]
    pub queries: ::prost::alloc::vec::Vec<super::super::project::v1::RoleQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGrantedProjectRolesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::project::v1::Role>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectMembersRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::member::v1::SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectMembersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::member::v1::Member>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectMemberRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectMemberRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectMemberRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppByIdRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppByIdResponse {
    #[prost(message, optional, tag="1")]
    pub app: ::core::option::Option<super::super::app::v1::App>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppsRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::app::v1::AppQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::app::v1::App>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppChangesRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::change::v1::ChangeQuery>,
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub app_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppChangesResponse {
    /// zitadel.v1.ListDetails details = 1; was always returned empty (as we cannot get the necessary info)
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::change::v1::Change>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOidcAppRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::app::v1::OidcResponseType", repeated, packed="false", tag="4")]
    pub response_types: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::super::app::v1::OidcGrantType", repeated, packed="false", tag="5")]
    pub grant_types: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::super::app::v1::OidcAppType", tag="6")]
    pub app_type: i32,
    #[prost(enumeration="super::super::app::v1::OidcAuthMethodType", tag="7")]
    pub auth_method_type: i32,
    #[prost(string, repeated, tag="8")]
    pub post_logout_redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::app::v1::OidcVersion", tag="9")]
    pub version: i32,
    #[prost(bool, tag="10")]
    pub dev_mode: bool,
    #[prost(enumeration="super::super::app::v1::OidcTokenType", tag="11")]
    pub access_token_type: i32,
    #[prost(bool, tag="12")]
    pub access_token_role_assertion: bool,
    #[prost(bool, tag="13")]
    pub id_token_role_assertion: bool,
    #[prost(bool, tag="14")]
    pub id_token_userinfo_assertion: bool,
    #[prost(message, optional, tag="15")]
    pub clock_skew: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(string, repeated, tag="16")]
    pub additional_origins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="17")]
    pub skip_native_app_success_page: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOidcAppResponse {
    #[prost(string, tag="1")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub none_compliant: bool,
    #[prost(message, repeated, tag="6")]
    pub compliance_problems: ::prost::alloc::vec::Vec<super::super::v1::LocalizedMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSamlAppRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="add_saml_app_request::Metadata", tags="3, 4")]
    pub metadata: ::core::option::Option<add_saml_app_request::Metadata>,
}
/// Nested message and enum types in `AddSAMLAppRequest`.
pub mod add_saml_app_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(bytes, tag="3")]
        MetadataXml(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag="4")]
        MetadataUrl(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSamlAppResponse {
    #[prost(string, tag="1")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddApiAppRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::app::v1::ApiAuthMethodType", tag="3")]
    pub auth_method_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddApiAppResponse {
    #[prost(string, tag="1")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOidcAppConfigRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::app::v1::OidcResponseType", repeated, packed="false", tag="4")]
    pub response_types: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::super::app::v1::OidcGrantType", repeated, packed="false", tag="5")]
    pub grant_types: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::super::app::v1::OidcAppType", tag="6")]
    pub app_type: i32,
    #[prost(enumeration="super::super::app::v1::OidcAuthMethodType", tag="7")]
    pub auth_method_type: i32,
    #[prost(string, repeated, tag="8")]
    pub post_logout_redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="9")]
    pub dev_mode: bool,
    #[prost(enumeration="super::super::app::v1::OidcTokenType", tag="10")]
    pub access_token_type: i32,
    #[prost(bool, tag="11")]
    pub access_token_role_assertion: bool,
    #[prost(bool, tag="12")]
    pub id_token_role_assertion: bool,
    #[prost(bool, tag="13")]
    pub id_token_userinfo_assertion: bool,
    #[prost(message, optional, tag="14")]
    pub clock_skew: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(string, repeated, tag="15")]
    pub additional_origins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="16")]
    pub skip_native_app_success_page: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOidcAppConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSamlAppConfigRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(oneof="update_saml_app_config_request::Metadata", tags="3, 4")]
    pub metadata: ::core::option::Option<update_saml_app_config_request::Metadata>,
}
/// Nested message and enum types in `UpdateSAMLAppConfigRequest`.
pub mod update_saml_app_config_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(bytes, tag="3")]
        MetadataXml(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag="4")]
        MetadataUrl(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSamlAppConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiAppConfigRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::app::v1::ApiAuthMethodType", tag="7")]
    pub auth_method_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiAppConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateAppRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateAppResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateAppRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateAppResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAppRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAppResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegenerateOidcClientSecretRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegenerateOidcClientSecretResponse {
    #[prost(string, tag="1")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegenerateApiClientSecretRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegenerateApiClientSecretResponse {
    #[prost(string, tag="1")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppKeyRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub key_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppKeyResponse {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<super::super::authn::v1::Key>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppKeysRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub project_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppKeysResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::authn::v1::Key>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAppKeyRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::authn::v1::KeyType", tag="3")]
    pub r#type: i32,
    #[prost(message, optional, tag="4")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAppKeyResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(bytes="vec", tag="3")]
    pub key_details: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAppKeyRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub key_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAppKeyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectGrantChangesRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::change::v1::ChangeQuery>,
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectGrantChangesResponse {
    /// zitadel.v1.ListDetails details = 1; was always returned empty (as we cannot get the necessary info)
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::change::v1::Change>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectGrantByIdRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectGrantByIdResponse {
    #[prost(message, optional, tag="1")]
    pub project_grant: ::core::option::Option<super::super::project::v1::GrantedProject>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectGrantsRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::project::v1::ProjectGrantQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectGrantsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::project::v1::GrantedProject>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllProjectGrantsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::project::v1::AllProjectGrantQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllProjectGrantsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::project::v1::GrantedProject>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectGrantRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub granted_org_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub role_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectGrantResponse {
    #[prost(string, tag="1")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectGrantRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub role_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateProjectGrantRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateProjectGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateProjectGrantRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateProjectGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectGrantRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectGrantMemberRolesRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    #[prost(string, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectGrantMemberRolesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(string, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectGrantMembersRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
    /// list limitations and ordering
    #[prost(message, optional, tag="3")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="4")]
    pub queries: ::prost::alloc::vec::Vec<super::super::member::v1::SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectGrantMembersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::member::v1::Member>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectGrantMemberRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectGrantMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectGrantMemberRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectGrantMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectGrantMemberRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectGrantMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserGrantByIdRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserGrantByIdResponse {
    #[prost(message, optional, tag="1")]
    pub user_grant: ::core::option::Option<super::super::user::v1::UserGrant>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserGrantRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::user::v1::UserGrantQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::user::v1::UserGrant>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserGrantRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub project_grant_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub role_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserGrantResponse {
    #[prost(string, tag="1")]
    pub user_grant_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserGrantRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub role_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserGrantRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserGrantRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserGrantRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserGrantResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkRemoveUserGrantRequest {
    #[prost(string, repeated, tag="1")]
    pub grant_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkRemoveUserGrantResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgIamPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgIamPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::OrgIamPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDomainPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDomainPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::DomainPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LoginPolicy>,
    /// deprecated: is_default is also defined in zitadel.policy.v1.LoginPolicy
    #[prost(bool, tag="2")]
    pub is_default: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLoginPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LoginPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomLoginPolicyRequest {
    #[prost(bool, tag="1")]
    pub allow_username_password: bool,
    #[prost(bool, tag="2")]
    pub allow_register: bool,
    #[prost(bool, tag="3")]
    pub allow_external_idp: bool,
    #[prost(bool, tag="4")]
    pub force_mfa: bool,
    #[prost(enumeration="super::super::policy::v1::PasswordlessType", tag="5")]
    pub passwordless_type: i32,
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
    #[prost(enumeration="super::super::policy::v1::SecondFactorType", repeated, tag="14")]
    pub second_factors: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::super::policy::v1::MultiFactorType", repeated, tag="15")]
    pub multi_factors: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag="16")]
    pub idps: ::prost::alloc::vec::Vec<add_custom_login_policy_request::Idp>,
    /// If set to true, the suffix (@domain.com) of an unknown username input on the login screen will be matched against the org domains and will redirect to the registration of that organization on success.
    #[prost(bool, tag="17")]
    pub allow_domain_discovery: bool,
    #[prost(bool, tag="18")]
    pub disable_login_with_email: bool,
    #[prost(bool, tag="19")]
    pub disable_login_with_phone: bool,
    #[prost(bool, tag="20")]
    pub force_mfa_local_only: bool,
}
/// Nested message and enum types in `AddCustomLoginPolicyRequest`.
pub mod add_custom_login_policy_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Idp {
        #[prost(string, tag="1")]
        pub idp_id: ::prost::alloc::string::String,
        #[prost(enumeration="super::super::super::idp::v1::IdpOwnerType", tag="2")]
        pub owner_type: i32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomLoginPolicyRequest {
    #[prost(bool, tag="1")]
    pub allow_username_password: bool,
    #[prost(bool, tag="2")]
    pub allow_register: bool,
    #[prost(bool, tag="3")]
    pub allow_external_idp: bool,
    #[prost(bool, tag="4")]
    pub force_mfa: bool,
    #[prost(enumeration="super::super::policy::v1::PasswordlessType", tag="5")]
    pub passwordless_type: i32,
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
    /// If set to true, the suffix (@domain.com) of an unknown username input on the login screen will be matched against the org domains and will redirect to the registration of that organization on success.
    #[prost(bool, tag="14")]
    pub allow_domain_discovery: bool,
    #[prost(bool, tag="15")]
    pub disable_login_with_email: bool,
    #[prost(bool, tag="16")]
    pub disable_login_with_phone: bool,
    #[prost(bool, tag="17")]
    pub force_mfa_local_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetLoginPolicyToDefaultRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetLoginPolicyToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicyIdPsRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicyIdPsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::idp::v1::IdpLoginPolicyLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddIdpToLoginPolicyRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::idp::v1::IdpOwnerType", tag="2")]
    pub owner_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddIdpToLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIdpFromLoginPolicyRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIdpFromLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicySecondFactorsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicySecondFactorsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::policy::v1::SecondFactorType", repeated, packed="false", tag="2")]
    pub result: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSecondFactorToLoginPolicyRequest {
    #[prost(enumeration="super::super::policy::v1::SecondFactorType", tag="1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSecondFactorToLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSecondFactorFromLoginPolicyRequest {
    #[prost(enumeration="super::super::policy::v1::SecondFactorType", tag="1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSecondFactorFromLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicyMultiFactorsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicyMultiFactorsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::policy::v1::MultiFactorType", repeated, packed="false", tag="2")]
    pub result: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMultiFactorToLoginPolicyRequest {
    #[prost(enumeration="super::super::policy::v1::MultiFactorType", tag="1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMultiFactorToLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMultiFactorFromLoginPolicyRequest {
    #[prost(enumeration="super::super::policy::v1::MultiFactorType", tag="1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMultiFactorFromLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPasswordComplexityPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPasswordComplexityPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::PasswordComplexityPolicy>,
    /// deprecated: is_default is also defined in zitadel.policy.v1.PasswordComplexityPolicy
    #[prost(bool, tag="2")]
    pub is_default: bool,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordComplexityPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordComplexityPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::PasswordComplexityPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomPasswordComplexityPolicyRequest {
    #[prost(uint64, tag="1")]
    pub min_length: u64,
    #[prost(bool, tag="2")]
    pub has_uppercase: bool,
    #[prost(bool, tag="3")]
    pub has_lowercase: bool,
    #[prost(bool, tag="4")]
    pub has_number: bool,
    #[prost(bool, tag="5")]
    pub has_symbol: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomPasswordComplexityPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomPasswordComplexityPolicyRequest {
    #[prost(uint64, tag="1")]
    pub min_length: u64,
    #[prost(bool, tag="2")]
    pub has_uppercase: bool,
    #[prost(bool, tag="3")]
    pub has_lowercase: bool,
    #[prost(bool, tag="4")]
    pub has_number: bool,
    #[prost(bool, tag="5")]
    pub has_symbol: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomPasswordComplexityPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPasswordComplexityPolicyToDefaultRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPasswordComplexityPolicyToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPasswordAgePolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPasswordAgePolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::PasswordAgePolicy>,
    /// deprecated: is_default is also defined in zitadel.policy.v1.PasswordAgePolicy
    #[prost(bool, tag="2")]
    pub is_default: bool,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordAgePolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordAgePolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::PasswordAgePolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomPasswordAgePolicyRequest {
    /// Amount of days after which a password will expire. The user will be forced to change the password on the following authentication.
    #[prost(uint32, tag="1")]
    pub max_age_days: u32,
    /// Amount of days after which the user should be notified of the upcoming expiry. ZITADEL will not notify the user.
    #[prost(uint32, tag="2")]
    pub expire_warn_days: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomPasswordAgePolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomPasswordAgePolicyRequest {
    /// Amount of days after which a password will expire. The user will be forced to change the password on the following authentication.
    #[prost(uint32, tag="1")]
    pub max_age_days: u32,
    /// Amount of days after which the user should be notified of the upcoming expiry. ZITADEL will not notify the user.
    #[prost(uint32, tag="2")]
    pub expire_warn_days: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomPasswordAgePolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPasswordAgePolicyToDefaultRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPasswordAgePolicyToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLockoutPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLockoutPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LockoutPolicy>,
    /// deprecated: is_default is also defined in zitadel.policy.v1.LockoutPolicy
    #[prost(bool, tag="2")]
    pub is_default: bool,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLockoutPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLockoutPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LockoutPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomLockoutPolicyRequest {
    #[prost(uint32, tag="1")]
    pub max_password_attempts: u32,
    #[prost(uint32, tag="2")]
    pub max_otp_attempts: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomLockoutPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomLockoutPolicyRequest {
    #[prost(uint32, tag="1")]
    pub max_password_attempts: u32,
    #[prost(uint32, tag="2")]
    pub max_otp_attempts: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomLockoutPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetLockoutPolicyToDefaultRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetLockoutPolicyToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivacyPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivacyPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::PrivacyPolicy>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPrivacyPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPrivacyPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::PrivacyPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomPrivacyPolicyRequest {
    #[prost(string, tag="1")]
    pub tos_link: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub privacy_link: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub help_link: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub support_email: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub docs_link: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub custom_link: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub custom_link_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomPrivacyPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomPrivacyPolicyRequest {
    #[prost(string, tag="1")]
    pub tos_link: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub privacy_link: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub help_link: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub support_email: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub docs_link: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub custom_link: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub custom_link_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomPrivacyPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPrivacyPolicyToDefaultRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPrivacyPolicyToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::NotificationPolicy>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultNotificationPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultNotificationPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::NotificationPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomNotificationPolicyRequest {
    #[prost(bool, tag="1")]
    pub password_change: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomNotificationPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomNotificationPolicyRequest {
    #[prost(bool, tag="1")]
    pub password_change: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomNotificationPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetNotificationPolicyToDefaultRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetNotificationPolicyToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLabelPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LabelPolicy>,
    /// deprecated: is_default is also defined in zitadel.policy.v1.LabelPolicy
    #[prost(bool, tag="2")]
    pub is_default: bool,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPreviewLabelPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPreviewLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LabelPolicy>,
    /// deprecated: is_default is also defined in zitadel.policy.v1.LabelPolicy
    #[prost(bool, tag="2")]
    pub is_default: bool,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLabelPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LabelPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomLabelPolicyRequest {
    #[prost(string, tag="1")]
    pub primary_color: ::prost::alloc::string::String,
    /// hides the org suffix on the login form if the scope \"urn:zitadel:iam:org:domain:primary:{domainname}\" is set
    #[prost(bool, tag="3")]
    pub hide_login_name_suffix: bool,
    #[prost(string, tag="4")]
    pub warn_color: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub background_color: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub font_color: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub primary_color_dark: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub background_color_dark: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub warn_color_dark: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub font_color_dark: ::prost::alloc::string::String,
    #[prost(bool, tag="11")]
    pub disable_watermark: bool,
    #[prost(enumeration="super::super::policy::v1::ThemeMode", tag="12")]
    pub theme_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomLabelPolicyRequest {
    #[prost(string, tag="1")]
    pub primary_color: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub hide_login_name_suffix: bool,
    #[prost(string, tag="4")]
    pub warn_color: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub background_color: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub font_color: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub primary_color_dark: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub background_color_dark: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub warn_color_dark: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub font_color_dark: ::prost::alloc::string::String,
    #[prost(bool, tag="11")]
    pub disable_watermark: bool,
    #[prost(enumeration="super::super::policy::v1::ThemeMode", tag="12")]
    pub theme_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateCustomLabelPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateCustomLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyLogoRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyLogoResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyLogoDarkRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyLogoDarkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyIconRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyIconResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyIconDarkRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyIconDarkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyFontRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCustomLabelPolicyFontResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetLabelPolicyToDefaultRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetLabelPolicyToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomInitMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomInitMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultInitMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultInitMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomInitMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomInitMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomInitMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomInitMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLoginTextsRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLoginTextsResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::LoginCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomLoginTextsRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomLoginTextsResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::LoginCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomLoginTextsRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub select_account_text: ::core::option::Option<super::super::text::v1::SelectAccountScreenText>,
    #[prost(message, optional, tag="3")]
    pub login_text: ::core::option::Option<super::super::text::v1::LoginScreenText>,
    #[prost(message, optional, tag="4")]
    pub password_text: ::core::option::Option<super::super::text::v1::PasswordScreenText>,
    #[prost(message, optional, tag="5")]
    pub username_change_text: ::core::option::Option<super::super::text::v1::UsernameChangeScreenText>,
    #[prost(message, optional, tag="6")]
    pub username_change_done_text: ::core::option::Option<super::super::text::v1::UsernameChangeDoneScreenText>,
    #[prost(message, optional, tag="7")]
    pub init_password_text: ::core::option::Option<super::super::text::v1::InitPasswordScreenText>,
    #[prost(message, optional, tag="8")]
    pub init_password_done_text: ::core::option::Option<super::super::text::v1::InitPasswordDoneScreenText>,
    #[prost(message, optional, tag="9")]
    pub email_verification_text: ::core::option::Option<super::super::text::v1::EmailVerificationScreenText>,
    #[prost(message, optional, tag="10")]
    pub email_verification_done_text: ::core::option::Option<super::super::text::v1::EmailVerificationDoneScreenText>,
    #[prost(message, optional, tag="11")]
    pub initialize_user_text: ::core::option::Option<super::super::text::v1::InitializeUserScreenText>,
    #[prost(message, optional, tag="12")]
    pub initialize_done_text: ::core::option::Option<super::super::text::v1::InitializeUserDoneScreenText>,
    #[prost(message, optional, tag="13")]
    pub init_mfa_prompt_text: ::core::option::Option<super::super::text::v1::InitMfaPromptScreenText>,
    #[prost(message, optional, tag="14")]
    pub init_mfa_otp_text: ::core::option::Option<super::super::text::v1::InitMfaotpScreenText>,
    #[prost(message, optional, tag="15")]
    pub init_mfa_u2f_text: ::core::option::Option<super::super::text::v1::InitMfau2fScreenText>,
    #[prost(message, optional, tag="16")]
    pub init_mfa_done_text: ::core::option::Option<super::super::text::v1::InitMfaDoneScreenText>,
    #[prost(message, optional, tag="17")]
    pub mfa_providers_text: ::core::option::Option<super::super::text::v1::MfaProvidersText>,
    #[prost(message, optional, tag="18")]
    pub verify_mfa_otp_text: ::core::option::Option<super::super::text::v1::VerifyMfaotpScreenText>,
    #[prost(message, optional, tag="19")]
    pub verify_mfa_u2f_text: ::core::option::Option<super::super::text::v1::VerifyMfau2fScreenText>,
    #[prost(message, optional, tag="20")]
    pub passwordless_text: ::core::option::Option<super::super::text::v1::PasswordlessScreenText>,
    #[prost(message, optional, tag="21")]
    pub password_change_text: ::core::option::Option<super::super::text::v1::PasswordChangeScreenText>,
    #[prost(message, optional, tag="22")]
    pub password_change_done_text: ::core::option::Option<super::super::text::v1::PasswordChangeDoneScreenText>,
    #[prost(message, optional, tag="23")]
    pub password_reset_done_text: ::core::option::Option<super::super::text::v1::PasswordResetDoneScreenText>,
    #[prost(message, optional, tag="24")]
    pub registration_option_text: ::core::option::Option<super::super::text::v1::RegistrationOptionScreenText>,
    #[prost(message, optional, tag="25")]
    pub registration_user_text: ::core::option::Option<super::super::text::v1::RegistrationUserScreenText>,
    #[prost(message, optional, tag="26")]
    pub registration_org_text: ::core::option::Option<super::super::text::v1::RegistrationOrgScreenText>,
    #[prost(message, optional, tag="27")]
    pub linking_user_done_text: ::core::option::Option<super::super::text::v1::LinkingUserDoneScreenText>,
    #[prost(message, optional, tag="28")]
    pub external_user_not_found_text: ::core::option::Option<super::super::text::v1::ExternalUserNotFoundScreenText>,
    #[prost(message, optional, tag="29")]
    pub success_login_text: ::core::option::Option<super::super::text::v1::SuccessLoginScreenText>,
    #[prost(message, optional, tag="30")]
    pub logout_text: ::core::option::Option<super::super::text::v1::LogoutDoneScreenText>,
    #[prost(message, optional, tag="31")]
    pub footer_text: ::core::option::Option<super::super::text::v1::FooterText>,
    #[prost(message, optional, tag="32")]
    pub passwordless_prompt_text: ::core::option::Option<super::super::text::v1::PasswordlessPromptScreenText>,
    #[prost(message, optional, tag="33")]
    pub passwordless_registration_text: ::core::option::Option<super::super::text::v1::PasswordlessRegistrationScreenText>,
    #[prost(message, optional, tag="34")]
    pub passwordless_registration_done_text: ::core::option::Option<super::super::text::v1::PasswordlessRegistrationDoneScreenText>,
    #[prost(message, optional, tag="35")]
    pub external_registration_user_overview_text: ::core::option::Option<super::super::text::v1::ExternalRegistrationUserOverviewScreenText>,
    #[prost(message, optional, tag="36")]
    pub linking_user_prompt_text: ::core::option::Option<super::super::text::v1::LinkingUserPromptScreenText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomLoginTextsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomLoginTextsToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomLoginTextsToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomPasswordResetMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomPasswordResetMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordResetMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordResetMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomPasswordResetMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomPasswordResetMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomPasswordResetMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomPasswordResetMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomVerifyEmailMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomVerifyEmailMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultVerifyEmailMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultVerifyEmailMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomVerifyEmailMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomVerifyEmailMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomVerifyEmailMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomVerifyEmailMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomVerifyPhoneMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomVerifyPhoneMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultVerifyPhoneMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultVerifyPhoneMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomVerifyPhoneMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomVerifyPhoneMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomVerifyPhoneMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomVerifyPhoneMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomVerifySmsotpMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomVerifySmsotpMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultVerifySmsotpMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultVerifySmsotpMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomVerifySmsotpMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomVerifySmsotpMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomVerifySmsotpMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomVerifySmsotpMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomVerifyEmailOtpMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomVerifyEmailOtpMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultVerifyEmailOtpMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultVerifyEmailOtpMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomVerifyEmailOtpMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomVerifyEmailOtpMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomVerifyEmailOtpMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomVerifyEmailOtpMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomDomainClaimedMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomDomainClaimedMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultDomainClaimedMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultDomainClaimedMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomDomainClaimedMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomDomainClaimedMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomDomainClaimedMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomDomainClaimedMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomPasswordlessRegistrationMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomPasswordlessRegistrationMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordlessRegistrationMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordlessRegistrationMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomPasswordlessRegistrationMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomPasswordlessRegistrationMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomPasswordlessRegistrationMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomPasswordlessRegistrationMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomPasswordChangeMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomPasswordChangeMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordChangeMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultPasswordChangeMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub custom_text: ::core::option::Option<super::super::text::v1::MessageCustomText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomPasswordChangeMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCustomPasswordChangeMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomPasswordChangeMessageTextToDefaultRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomPasswordChangeMessageTextToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgIdpByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgIdpByIdResponse {
    #[prost(message, optional, tag="1")]
    pub idp: ::core::option::Option<super::super::idp::v1::Idp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgIdPsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::idp::v1::IdpFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<IdpQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpQuery {
    #[prost(oneof="idp_query::Query", tags="1, 2, 3")]
    pub query: ::core::option::Option<idp_query::Query>,
}
/// Nested message and enum types in `IDPQuery`.
pub mod idp_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        IdpIdQuery(super::super::super::idp::v1::IdpidQuery),
        #[prost(message, tag="2")]
        IdpNameQuery(super::super::super::idp::v1::IdpNameQuery),
        #[prost(message, tag="3")]
        OwnerTypeQuery(super::super::super::idp::v1::IdpOwnerTypeQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgIdPsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::idp::v1::IdpFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::idp::v1::Idp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgOidcidpRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::idp::v1::IdpStylingType", tag="2")]
    pub styling_type: i32,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::idp::v1::OidcMappingField", tag="7")]
    pub display_name_mapping: i32,
    #[prost(enumeration="super::super::idp::v1::OidcMappingField", tag="8")]
    pub username_mapping: i32,
    #[prost(bool, tag="9")]
    pub auto_register: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgOidcidpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgJwtidpRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::idp::v1::IdpStylingType", tag="2")]
    pub styling_type: i32,
    #[prost(string, tag="3")]
    pub jwt_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub keys_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub header_name: ::prost::alloc::string::String,
    #[prost(bool, tag="7")]
    pub auto_register: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrgJwtidpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateOrgIdpRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateOrgIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateOrgIdpRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateOrgIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgIdpRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
}
/// This is an empty response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgIdpResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgIdpRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::idp::v1::IdpStylingType", tag="3")]
    pub styling_type: i32,
    #[prost(bool, tag="4")]
    pub auto_register: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgIdpoidcConfigRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::idp::v1::OidcMappingField", tag="6")]
    pub display_name_mapping: i32,
    #[prost(enumeration="super::super::idp::v1::OidcMappingField", tag="7")]
    pub username_mapping: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgIdpoidcConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgIdpjwtConfigRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub jwt_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub keys_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub header_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgIdpjwtConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProvidersRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<ProviderQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderQuery {
    #[prost(oneof="provider_query::Query", tags="1, 2, 3")]
    pub query: ::core::option::Option<provider_query::Query>,
}
/// Nested message and enum types in `ProviderQuery`.
pub mod provider_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        IdpIdQuery(super::super::super::idp::v1::IdpidQuery),
        #[prost(message, tag="2")]
        IdpNameQuery(super::super::super::idp::v1::IdpNameQuery),
        #[prost(message, tag="3")]
        OwnerTypeQuery(super::super::super::idp::v1::IdpOwnerTypeQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProvidersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::idp::v1::Provider>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProviderByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProviderByIdResponse {
    #[prost(message, optional, tag="1")]
    pub idp: ::core::option::Option<super::super::idp::v1::Provider>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGenericOAuthProviderRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user_endpoint: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="7")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// identifying attribute of the user in the response of the user_endpoint
    #[prost(string, tag="8")]
    pub id_attribute: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGenericOAuthProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGenericOAuthProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// client_secret will only be updated if provided
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub token_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub user_endpoint: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="8")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// identifying attribute of the user in the response of the user_endpoint
    #[prost(string, tag="9")]
    pub id_attribute: ::prost::alloc::string::String,
    #[prost(message, optional, tag="10")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGenericOAuthProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGenericOidcProviderRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
    #[prost(bool, tag="7")]
    pub is_id_token_mapping: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGenericOidcProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGenericOidcProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_id: ::prost::alloc::string::String,
    /// client_secret will only be updated if provided
    #[prost(string, tag="5")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="7")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
    #[prost(bool, tag="8")]
    pub is_id_token_mapping: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGenericOidcProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateGenericOidcProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof="migrate_generic_oidc_provider_request::Template", tags="2, 3")]
    pub template: ::core::option::Option<migrate_generic_oidc_provider_request::Template>,
}
/// Nested message and enum types in `MigrateGenericOIDCProviderRequest`.
pub mod migrate_generic_oidc_provider_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Template {
        #[prost(message, tag="2")]
        Azure(super::AddAzureAdProviderRequest),
        #[prost(message, tag="3")]
        Google(super::AddGoogleProviderRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateGenericOidcProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddJwtProviderRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub jwt_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub keys_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub header_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddJwtProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJwtProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub jwt_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub keys_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub header_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJwtProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAzureAdProviderRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
    /// if not provided the `common` tenant will be used
    #[prost(message, optional, tag="4")]
    pub tenant: ::core::option::Option<super::super::idp::v1::AzureAdTenant>,
    #[prost(bool, tag="5")]
    pub email_verified: bool,
    #[prost(string, repeated, tag="6")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="7")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAzureAdProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAzureAdProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// client_secret will only be updated if provided
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    /// if not provided the `common` tenant will be used
    #[prost(message, optional, tag="5")]
    pub tenant: ::core::option::Option<super::super::idp::v1::AzureAdTenant>,
    #[prost(bool, tag="6")]
    pub email_verified: bool,
    #[prost(string, repeated, tag="7")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAzureAdProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGitHubProviderRequest {
    /// GitHub will be used as default, if no name is provided
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGitHubProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitHubProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// client_secret will only be updated if provided
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitHubProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGitHubEnterpriseServerProviderRequest {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user_endpoint: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="7")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGitHubEnterpriseServerProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitHubEnterpriseServerProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// client_secret will only be updated if provided
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub token_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub user_endpoint: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="8")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="9")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitHubEnterpriseServerProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGitLabProviderRequest {
    /// GitLab will be used as default, if no name is provided
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGitLabProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitLabProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// client_secret will only be updated if provided
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitLabProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGitLabSelfHostedProviderRequest {
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGitLabSelfHostedProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitLabSelfHostedProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_id: ::prost::alloc::string::String,
    /// client_secret will only be updated if provided
    #[prost(string, tag="5")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="7")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitLabSelfHostedProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGoogleProviderRequest {
    /// Google will be used as default, if no name is provided
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGoogleProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// client_secret will only be updated if provided
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLdapProviderRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="3")]
    pub start_tls: bool,
    #[prost(string, tag="4")]
    pub base_dn: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub bind_dn: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub bind_password: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub user_base: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="8")]
    pub user_object_classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="9")]
    pub user_filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="10")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="11")]
    pub attributes: ::core::option::Option<super::super::idp::v1::LdapAttributes>,
    #[prost(message, optional, tag="12")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLdapProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLdapProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="4")]
    pub start_tls: bool,
    #[prost(string, tag="5")]
    pub base_dn: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub bind_dn: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub bind_password: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub user_base: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="9")]
    pub user_object_classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="10")]
    pub user_filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="11")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="12")]
    pub attributes: ::core::option::Option<super::super::idp::v1::LdapAttributes>,
    #[prost(message, optional, tag="13")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLdapProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSamlProviderRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Binding which defines the type of communication with the identity provider.
    #[prost(enumeration="super::super::idp::v1::SamlBinding", tag="4")]
    pub binding: i32,
    /// Boolean which defines if the authentication requests are signed.
    #[prost(bool, tag="5")]
    pub with_signed_request: bool,
    #[prost(message, optional, tag="6")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
    /// Optionally specify the `nameid-format` requested.
    #[prost(enumeration="super::super::idp::v1::SamlNameIdFormat", optional, tag="7")]
    pub name_id_format: ::core::option::Option<i32>,
    /// Optionally specify the name of the attribute, which will be used to map the user
    /// in case the nameid-format returned is `urn:oasis:names:tc:SAML:2.0:nameid-format:transient`.
    #[prost(string, optional, tag="8")]
    pub transient_mapping_attribute_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="add_saml_provider_request::Metadata", tags="2, 3")]
    pub metadata: ::core::option::Option<add_saml_provider_request::Metadata>,
}
/// Nested message and enum types in `AddSAMLProviderRequest`.
pub mod add_saml_provider_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// Metadata of the SAML identity provider.
        #[prost(bytes, tag="2")]
        MetadataXml(::prost::alloc::vec::Vec<u8>),
        /// Url to the metadata of the SAML identity provider.
        #[prost(string, tag="3")]
        MetadataUrl(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSamlProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSamlProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Binding which defines the type of communication with the identity provider.
    #[prost(enumeration="super::super::idp::v1::SamlBinding", tag="5")]
    pub binding: i32,
    /// Boolean which defines if the authentication requests are signed.
    #[prost(bool, tag="6")]
    pub with_signed_request: bool,
    #[prost(message, optional, tag="7")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
    /// Optionally specify the `nameid-format` requested.
    #[prost(enumeration="super::super::idp::v1::SamlNameIdFormat", optional, tag="8")]
    pub name_id_format: ::core::option::Option<i32>,
    /// Optionally specify the name of the attribute, which will be used to map the user
    /// in case the nameid-format returned is `urn:oasis:names:tc:SAML:2.0:nameid-format:transient`.
    #[prost(string, optional, tag="9")]
    pub transient_mapping_attribute_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Metadata of the SAML identity provider.
    #[prost(oneof="update_saml_provider_request::Metadata", tags="3, 4")]
    pub metadata: ::core::option::Option<update_saml_provider_request::Metadata>,
}
/// Nested message and enum types in `UpdateSAMLProviderRequest`.
pub mod update_saml_provider_request {
    /// Metadata of the SAML identity provider.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(bytes, tag="3")]
        MetadataXml(::prost::alloc::vec::Vec<u8>),
        /// Url to the metadata of the SAML identity provider.
        #[prost(string, tag="4")]
        MetadataUrl(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSamlProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegenerateSamlProviderCertificateRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegenerateSamlProviderCertificateResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAppleProviderRequest {
    /// Apple will be used as default, if no name is provided
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub team_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub key_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="6")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="7")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAppleProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppleProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub team_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub key_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="7")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub provider_options: ::core::option::Option<super::super::idp::v1::Options>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppleProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActionsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::action::v1::ActionFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<ActionQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionQuery {
    #[prost(oneof="action_query::Query", tags="1, 2, 3")]
    pub query: ::core::option::Option<action_query::Query>,
}
/// Nested message and enum types in `ActionQuery`.
pub mod action_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        ActionIdQuery(super::super::super::action::v1::ActionIdQuery),
        #[prost(message, tag="2")]
        ActionNameQuery(super::super::super::action::v1::ActionNameQuery),
        #[prost(message, tag="3")]
        ActionStateQuery(super::super::super::action::v1::ActionStateQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActionsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::action::v1::ActionFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::action::v1::Action>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateActionRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub script: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(bool, tag="4")]
    pub allowed_to_fail: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateActionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActionResponse {
    #[prost(message, optional, tag="1")]
    pub action: ::core::option::Option<super::super::action::v1::Action>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub script: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(bool, tag="5")]
    pub allowed_to_fail: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteActionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteActionResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFlowTypesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFlowTypesResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<super::super::action::v1::FlowType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFlowTriggerTypesRequest {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFlowTriggerTypesResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<super::super::action::v1::TriggerType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateActionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateActionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateActionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateActionResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowRequest {
    /// id of the flow
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowResponse {
    #[prost(message, optional, tag="1")]
    pub flow: ::core::option::Option<super::super::action::v1::Flow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearFlowRequest {
    /// id of the flow
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearFlowResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTriggerActionsRequest {
    /// id of the flow type.
    /// Following flows are currently allowed:
    /// - External Authentication: FLOW_TYPE_EXTERNAL_AUTHENTICATION or 1
    /// - Internal Authentication: 3
    /// - Complement Token: 2
    /// - Complement SAML Response: 4
    #[prost(string, tag="1")]
    pub flow_type: ::prost::alloc::string::String,
    /// id of the trigger type.
    /// Following triggers are currently allowed:
    /// - External Authentication:
    ///    - Post Authentication: TRIGGER_TYPE_POST_AUTHENTICATION or 1
    ///    - Pre Creation: TRIGGER_TYPE_PRE_CREATION or 2
    ///    - Post Creation: TRIGGER_TYPE_POST_CREATION or 3 
    /// - Internal Authentication:
    ///    - Post Authentication: TRIGGER_TYPE_POST_AUTHENTICATION or 1
    ///    - Pre Creation: TRIGGER_TYPE_PRE_CREATION or 2
    ///    - Post Creation: TRIGGER_TYPE_POST_CREATION or 3 
    /// - Complement Token:
    ///    - Pre Userinfo Creation: 4
    ///    - Pre Access Token Creation: 5
    /// - Complement SAML Response:
    ///    - Pre SAML Response Creation: 6
    #[prost(string, tag="2")]
    pub trigger_type: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub action_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTriggerActionsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
include!("zitadel.management.v1.tonic.rs");
// @@protoc_insertion_point(module)