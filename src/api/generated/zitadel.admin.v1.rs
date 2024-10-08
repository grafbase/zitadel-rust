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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllowedLanguagesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllowedLanguagesResponse {
    #[prost(string, repeated, tag="1")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultLanguageRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultLanguageResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLanguageRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultLanguageResponse {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultOrgRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultOrgResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultOrgRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultOrgResponse {
    #[prost(message, optional, tag="1")]
    pub org: ::core::option::Option<super::super::org::v1::Org>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyInstanceRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyInstanceResponse {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::instance::v1::InstanceDetail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceDomainsRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::instance::v1::DomainFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::instance::v1::DomainSearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceDomainsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::instance::v1::DomainFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::instance::v1::Domain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceTrustedDomainsRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::instance::v1::DomainFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::instance::v1::TrustedDomainSearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceTrustedDomainsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::instance::v1::DomainFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::instance::v1::TrustedDomain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddInstanceTrustedDomainRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddInstanceTrustedDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveInstanceTrustedDomainRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveInstanceTrustedDomainResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretGeneratorsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::settings::v1::SecretGeneratorQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretGeneratorsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::settings::v1::SecretGenerator>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretGeneratorRequest {
    #[prost(enumeration="super::super::settings::v1::SecretGeneratorType", tag="1")]
    pub generator_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretGeneratorResponse {
    #[prost(message, optional, tag="1")]
    pub secret_generator: ::core::option::Option<super::super::settings::v1::SecretGenerator>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecretGeneratorRequest {
    #[prost(enumeration="super::super::settings::v1::SecretGeneratorType", tag="1")]
    pub generator_type: i32,
    #[prost(uint32, tag="2")]
    pub length: u32,
    #[prost(message, optional, tag="3")]
    pub expiry: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(bool, tag="4")]
    pub include_lower_letters: bool,
    #[prost(bool, tag="5")]
    pub include_upper_letters: bool,
    #[prost(bool, tag="6")]
    pub include_digits: bool,
    #[prost(bool, tag="7")]
    pub include_symbols: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecretGeneratorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSmtpConfigRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSmtpConfigResponse {
    #[prost(message, optional, tag="1")]
    pub smtp_config: ::core::option::Option<super::super::settings::v1::SmtpConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSmtpConfigByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSmtpConfigByIdResponse {
    #[prost(message, optional, tag="1")]
    pub smtp_config: ::core::option::Option<super::super::settings::v1::SmtpConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSmtpConfigsRequest {
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSmtpConfigsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::settings::v1::SmtpConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSmtpConfigRequest {
    #[prost(string, tag="1")]
    pub sender_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender_name: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub tls: bool,
    #[prost(string, tag="4")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub reply_to_address: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSmtpConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSmtpConfigRequest {
    #[prost(string, tag="1")]
    pub sender_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender_name: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub tls: bool,
    #[prost(string, tag="4")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reply_to_address: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSmtpConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSmtpConfigPasswordRequest {
    #[prost(string, tag="1")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSmtpConfigPasswordResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateSmtpConfigRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateSmtpConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateSmtpConfigRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateSmtpConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSmtpConfigRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSmtpConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSmtpConfigByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub receiver_address: ::prost::alloc::string::String,
}
/// This is an empty response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSmtpConfigByIdResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSmtpConfigRequest {
    #[prost(string, tag="1")]
    pub sender_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender_name: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub tls: bool,
    #[prost(string, tag="4")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub receiver_address: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub id: ::prost::alloc::string::String,
}
/// This is an empty response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSmtpConfigResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSmsProvidersRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSmsProvidersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::settings::v1::SmsProvider>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSmsProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSmsProviderResponse {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<super::super::settings::v1::SmsProvider>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSmsProviderTwilioRequest {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sender_number: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSmsProviderTwilioResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSmsProviderTwilioRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sender_number: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSmsProviderTwilioResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSmsProviderTwilioTokenRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSmsProviderTwilioTokenResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateSmsProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateSmsProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateSmsProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateSmsProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSmsProviderRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSmsProviderResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileSystemNotificationProviderRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileSystemNotificationProviderResponse {
    #[prost(message, optional, tag="1")]
    pub provider: ::core::option::Option<super::super::settings::v1::DebugNotificationProvider>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogNotificationProviderRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogNotificationProviderResponse {
    #[prost(message, optional, tag="1")]
    pub provider: ::core::option::Option<super::super::settings::v1::DebugNotificationProvider>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOidcSettingsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOidcSettingsResponse {
    #[prost(message, optional, tag="1")]
    pub settings: ::core::option::Option<super::super::settings::v1::OidcSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOidcSettingsRequest {
    #[prost(message, optional, tag="1")]
    pub access_token_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="2")]
    pub id_token_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub refresh_token_idle_expiration: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="4")]
    pub refresh_token_expiration: ::core::option::Option<::pbjson_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOidcSettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOidcSettingsRequest {
    #[prost(message, optional, tag="1")]
    pub access_token_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="2")]
    pub id_token_lifetime: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub refresh_token_idle_expiration: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="4")]
    pub refresh_token_expiration: ::core::option::Option<::pbjson_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOidcSettingsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecurityPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecurityPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::settings::v1::SecurityPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSecurityPolicyRequest {
    /// states if iframe embedding is enabled or disabled
    #[prost(bool, tag="1")]
    pub enable_iframe_embedding: bool,
    /// origins allowed loading ZITADEL in an iframe if enable_iframe_embedding is true
    #[prost(string, repeated, tag="2")]
    pub allowed_origins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// allows users to impersonate other users. The impersonator needs the appropriate `*_IMPERSONATOR` roles assigned as well"
    #[prost(bool, tag="3")]
    pub enable_impersonation: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSecurityPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// if name or domain is already in use, org is not unique
/// at least one argument has to be provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsOrgUniqueRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsOrgUniqueResponse {
    #[prost(bool, tag="1")]
    pub is_unique: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgByIdResponse {
    #[prost(message, optional, tag="1")]
    pub org: ::core::option::Option<super::super::org::v1::Org>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgsRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::org::v1::OrgFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::org::v1::OrgQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::org::v1::OrgFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::org::v1::Org>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpOrgRequest {
    #[prost(message, optional, tag="1")]
    pub org: ::core::option::Option<set_up_org_request::Org>,
    /// specify Org Member Roles for the provided user (default is ORG_OWNER if roles are empty)
    #[prost(string, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(oneof="set_up_org_request::User", tags="2")]
    pub user: ::core::option::Option<set_up_org_request::User>,
}
/// Nested message and enum types in `SetUpOrgRequest`.
pub mod set_up_org_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Org {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub domain: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Human {
        #[prost(string, tag="1")]
        pub user_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub profile: ::core::option::Option<human::Profile>,
        #[prost(message, optional, tag="3")]
        pub email: ::core::option::Option<human::Email>,
        #[prost(message, optional, tag="4")]
        pub phone: ::core::option::Option<human::Phone>,
        #[prost(string, tag="5")]
        pub password: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Human`.
    pub mod human {
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
            #[prost(enumeration="super::super::super::super::user::v1::Gender", tag="6")]
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
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum User {
        /// oneof field for the user managing the organization
        #[prost(message, tag="2")]
        Human(Human),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpOrgResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOrgResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdpByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdpByIdResponse {
    #[prost(message, optional, tag="1")]
    pub idp: ::core::option::Option<super::super::idp::v1::Idp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIdPsRequest {
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
    #[prost(oneof="idp_query::Query", tags="1, 2")]
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
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIdPsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::idp::v1::IdpFieldName", tag="2")]
    pub sorting_column: i32,
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<super::super::idp::v1::Idp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOidcidpRequest {
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
pub struct AddOidcidpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddJwtidpRequest {
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
pub struct AddJwtidpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIdpRequest {
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
pub struct UpdateIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateIdpRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateIdpRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIdpRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIdpResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIdpoidcConfigRequest {
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::idp::v1::OidcMappingField", tag="6")]
    pub display_name_mapping: i32,
    #[prost(enumeration="super::super::idp::v1::OidcMappingField", tag="7")]
    pub username_mapping: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIdpoidcConfigResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIdpjwtConfigRequest {
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
pub struct UpdateIdpjwtConfigResponse {
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
    #[prost(oneof="provider_query::Query", tags="1, 2")]
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
    /// Boolean which defines if the authentication requests are signed
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
        /// Url to the metadata of the SAML identity provider
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
pub struct UpdateOrgIamPolicyRequest {
    #[prost(bool, tag="1")]
    pub user_login_must_be_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrgIamPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomOrgIamPolicyRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomOrgIamPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::OrgIamPolicy>,
    /// deprecated: is_default is also defined in zitadel.policy.v1.OrgIAMPolicy
    #[prost(bool, tag="2")]
    pub is_default: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomOrgIamPolicyRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    /// the username has to end with the domain of its organization (uniqueness is organization based)
    #[prost(bool, tag="2")]
    pub user_login_must_be_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomOrgIamPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomOrgIamPolicyRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub user_login_must_be_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomOrgIamPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomOrgIamPolicyToDefaultRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomOrgIamPolicyToDefaultResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
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
pub struct UpdateDomainPolicyRequest {
    #[prost(bool, tag="1")]
    pub user_login_must_be_domain: bool,
    #[prost(bool, tag="2")]
    pub validate_org_domains: bool,
    #[prost(bool, tag="3")]
    pub smtp_sender_address_matches_instance_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDomainPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomDomainPolicyRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomDomainPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::DomainPolicy>,
    /// deprecated: is_default is also defined in zitadel.policy.v1.DomainPolicy
    #[prost(bool, tag="2")]
    pub is_default: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomDomainPolicyRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    /// the username has to end with the domain of its organization (uniqueness is organization based)
    #[prost(bool, tag="2")]
    pub user_login_must_be_domain: bool,
    #[prost(bool, tag="3")]
    pub validate_org_domains: bool,
    #[prost(bool, tag="4")]
    pub smtp_sender_address_matches_instance_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCustomDomainPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomDomainPolicyRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub user_login_must_be_domain: bool,
    #[prost(bool, tag="3")]
    pub validate_org_domains: bool,
    #[prost(bool, tag="4")]
    pub smtp_sender_address_matches_instance_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomDomainPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomDomainPolicyToDefaultRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetCustomDomainPolicyToDefaultResponse {
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLabelPolicyRequest {
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
pub struct UpdateLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateLabelPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateLabelPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyLogoRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyLogoResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyLogoDarkRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyLogoDarkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyIconRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyIconResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyIconDarkRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyIconDarkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyFontRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLabelPolicyFontResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginPolicyRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::policy::v1::LoginPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLoginPolicyRequest {
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
pub struct UpdateLoginPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicyIdPsRequest {
    /// list limitations and ordering
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
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicySecondFactorsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicySecondFactorsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::policy::v1::SecondFactorType", repeated, tag="2")]
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
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicyMultiFactorsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoginPolicyMultiFactorsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(enumeration="super::super::policy::v1::MultiFactorType", repeated, tag="2")]
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePasswordComplexityPolicyRequest {
    #[prost(uint32, tag="1")]
    pub min_length: u32,
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
pub struct UpdatePasswordComplexityPolicyResponse {
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePasswordAgePolicyRequest {
    /// Amount of days after which a password will expire. The user will be forced to change the password on the following authentication.
    #[prost(uint32, tag="1")]
    pub max_age_days: u32,
    /// Amount of days after which the user should be notified of the upcoming expiry. ZITADEL will not notify the user.
    #[prost(uint32, tag="2")]
    pub expire_warn_days: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePasswordAgePolicyResponse {
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLockoutPolicyRequest {
    /// failed attempts until a user gets locked
    #[prost(uint32, tag="1")]
    pub max_password_attempts: u32,
    #[prost(uint32, tag="2")]
    pub max_otp_attempts: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLockoutPolicyResponse {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePrivacyPolicyRequest {
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
pub struct UpdatePrivacyPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNotificationPolicyRequest {
    #[prost(bool, tag="1")]
    pub password_change: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNotificationPolicyResponse {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotificationPolicyRequest {
    #[prost(bool, tag="1")]
    pub password_change: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotificationPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
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
pub struct SetDefaultInitMessageTextRequest {
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
pub struct SetDefaultInitMessageTextResponse {
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
pub struct SetDefaultPasswordResetMessageTextRequest {
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
pub struct SetDefaultPasswordResetMessageTextResponse {
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
pub struct SetDefaultVerifyEmailMessageTextRequest {
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
pub struct SetDefaultVerifyEmailMessageTextResponse {
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
pub struct SetDefaultVerifyPhoneMessageTextRequest {
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
pub struct SetDefaultVerifyPhoneMessageTextResponse {
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
pub struct SetDefaultVerifySmsotpMessageTextRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultVerifySmsotpMessageTextResponse {
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
pub struct SetDefaultVerifyEmailOtpMessageTextRequest {
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
pub struct SetDefaultVerifyEmailOtpMessageTextResponse {
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
pub struct SetDefaultDomainClaimedMessageTextRequest {
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
pub struct SetDefaultDomainClaimedMessageTextResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
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
pub struct SetDefaultPasswordChangeMessageTextRequest {
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
pub struct SetDefaultPasswordChangeMessageTextResponse {
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
pub struct SetDefaultPasswordlessRegistrationMessageTextRequest {
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
pub struct SetDefaultPasswordlessRegistrationMessageTextResponse {
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
pub struct AddIamMemberRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddIamMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIamMemberRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIamMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIamMemberRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIamMemberResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIamMemberRolesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIamMemberRolesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(string, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIamMembersRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<super::super::member::v1::SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIamMembersResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::member::v1::Member>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsResponse {
    /// TODO: list details
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<View>,
}
/// This is an empty request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFailedEventsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFailedEventsResponse {
    /// TODO: list details
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<FailedEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFailedEventRequest {
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub failed_sequence: u64,
}
/// This is an empty response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFailedEventResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct View {
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub processed_sequence: u64,
    /// The timestamp the event occurred
    #[prost(message, optional, tag="4")]
    pub event_timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub last_successful_spooler_run: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedEvent {
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub failed_sequence: u64,
    #[prost(uint64, tag="4")]
    pub failure_count: u64,
    #[prost(string, tag="5")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub last_failed: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataRequest {
    #[prost(string, tag="9")]
    pub timeout: ::prost::alloc::string::String,
    #[prost(oneof="import_data_request::Data", tags="1, 2, 3, 4, 5, 6, 7, 8")]
    pub data: ::core::option::Option<import_data_request::Data>,
}
/// Nested message and enum types in `ImportDataRequest`.
pub mod import_data_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalInput {
        #[prost(string, tag="1")]
        pub path: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct S3Input {
        #[prost(string, tag="1")]
        pub path: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub endpoint: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub access_key_id: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub secret_access_key: ::prost::alloc::string::String,
        #[prost(bool, tag="5")]
        pub ssl: bool,
        #[prost(string, tag="6")]
        pub bucket: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsInput {
        #[prost(string, tag="1")]
        pub bucket: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub serviceaccount_json: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub path: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="1")]
        DataOrgs(super::ImportDataOrg),
        #[prost(message, tag="2")]
        DataOrgsv1(super::super::super::v1::v1::ImportDataOrg),
        #[prost(message, tag="3")]
        DataOrgsLocal(LocalInput),
        #[prost(message, tag="4")]
        DataOrgsv1Local(LocalInput),
        #[prost(message, tag="5")]
        DataOrgsS3(S3Input),
        #[prost(message, tag="6")]
        DataOrgsv1S3(S3Input),
        #[prost(message, tag="7")]
        DataOrgsGcs(GcsInput),
        #[prost(message, tag="8")]
        DataOrgsv1Gcs(GcsInput),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataOrg {
    #[prost(message, repeated, tag="1")]
    pub orgs: ::prost::alloc::vec::Vec<DataOrg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataOrg {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub org: ::core::option::Option<super::super::management::v1::AddOrgRequest>,
    #[prost(message, optional, tag="4")]
    pub domain_policy: ::core::option::Option<AddCustomDomainPolicyRequest>,
    #[prost(message, optional, tag="5")]
    pub label_policy: ::core::option::Option<super::super::management::v1::AddCustomLabelPolicyRequest>,
    #[prost(message, optional, tag="6")]
    pub lockout_policy: ::core::option::Option<super::super::management::v1::AddCustomLockoutPolicyRequest>,
    #[prost(message, optional, tag="7")]
    pub login_policy: ::core::option::Option<super::super::management::v1::AddCustomLoginPolicyRequest>,
    #[prost(message, optional, tag="8")]
    pub password_complexity_policy: ::core::option::Option<super::super::management::v1::AddCustomPasswordComplexityPolicyRequest>,
    #[prost(message, optional, tag="9")]
    pub privacy_policy: ::core::option::Option<super::super::management::v1::AddCustomPrivacyPolicyRequest>,
    #[prost(message, repeated, tag="10")]
    pub projects: ::prost::alloc::vec::Vec<super::super::v1::v1::DataProject>,
    #[prost(message, repeated, tag="11")]
    pub project_roles: ::prost::alloc::vec::Vec<super::super::management::v1::AddProjectRoleRequest>,
    #[prost(message, repeated, tag="12")]
    pub api_apps: ::prost::alloc::vec::Vec<super::super::v1::v1::DataApiApplication>,
    #[prost(message, repeated, tag="13")]
    pub oidc_apps: ::prost::alloc::vec::Vec<super::super::v1::v1::DataOidcApplication>,
    #[prost(message, repeated, tag="14")]
    pub human_users: ::prost::alloc::vec::Vec<super::super::v1::v1::DataHumanUser>,
    #[prost(message, repeated, tag="15")]
    pub machine_users: ::prost::alloc::vec::Vec<super::super::v1::v1::DataMachineUser>,
    #[prost(message, repeated, tag="16")]
    pub trigger_actions: ::prost::alloc::vec::Vec<super::super::management::v1::SetTriggerActionsRequest>,
    #[prost(message, repeated, tag="17")]
    pub actions: ::prost::alloc::vec::Vec<super::super::v1::v1::DataAction>,
    #[prost(message, repeated, tag="18")]
    pub project_grants: ::prost::alloc::vec::Vec<super::super::v1::v1::DataProjectGrant>,
    #[prost(message, repeated, tag="19")]
    pub user_grants: ::prost::alloc::vec::Vec<super::super::management::v1::AddUserGrantRequest>,
    #[prost(message, repeated, tag="20")]
    pub org_members: ::prost::alloc::vec::Vec<super::super::management::v1::AddOrgMemberRequest>,
    #[prost(message, repeated, tag="21")]
    pub project_members: ::prost::alloc::vec::Vec<super::super::management::v1::AddProjectMemberRequest>,
    #[prost(message, repeated, tag="22")]
    pub project_grant_members: ::prost::alloc::vec::Vec<super::super::management::v1::AddProjectGrantMemberRequest>,
    #[prost(message, repeated, tag="23")]
    pub user_metadata: ::prost::alloc::vec::Vec<super::super::management::v1::SetUserMetadataRequest>,
    #[prost(message, repeated, tag="24")]
    pub login_texts: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomLoginTextsRequest>,
    #[prost(message, repeated, tag="25")]
    pub init_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomInitMessageTextRequest>,
    #[prost(message, repeated, tag="26")]
    pub password_reset_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomPasswordResetMessageTextRequest>,
    #[prost(message, repeated, tag="27")]
    pub verify_email_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomVerifyEmailMessageTextRequest>,
    #[prost(message, repeated, tag="28")]
    pub verify_phone_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomVerifyPhoneMessageTextRequest>,
    #[prost(message, repeated, tag="29")]
    pub domain_claimed_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomDomainClaimedMessageTextRequest>,
    #[prost(message, repeated, tag="30")]
    pub passwordless_registration_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomPasswordlessRegistrationMessageTextRequest>,
    #[prost(message, repeated, tag="31")]
    pub oidc_idps: ::prost::alloc::vec::Vec<super::super::v1::v1::DataOidcidp>,
    #[prost(message, repeated, tag="32")]
    pub jwt_idps: ::prost::alloc::vec::Vec<super::super::v1::v1::DataJwtidp>,
    #[prost(message, repeated, tag="33")]
    pub user_links: ::prost::alloc::vec::Vec<super::super::idp::v1::IdpUserLink>,
    #[prost(message, repeated, tag="34")]
    pub domains: ::prost::alloc::vec::Vec<super::super::org::v1::Domain>,
    #[prost(message, repeated, tag="35")]
    pub app_keys: ::prost::alloc::vec::Vec<super::super::v1::v1::DataAppKey>,
    #[prost(message, repeated, tag="36")]
    pub machine_keys: ::prost::alloc::vec::Vec<super::super::v1::v1::DataMachineKey>,
    #[prost(message, repeated, tag="37")]
    pub verify_sms_otp_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomVerifySmsotpMessageTextRequest>,
    #[prost(message, repeated, tag="38")]
    pub verify_email_otp_messages: ::prost::alloc::vec::Vec<super::super::management::v1::SetCustomVerifyEmailOtpMessageTextRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataResponse {
    #[prost(message, repeated, tag="1")]
    pub errors: ::prost::alloc::vec::Vec<ImportDataError>,
    #[prost(message, optional, tag="2")]
    pub success: ::core::option::Option<ImportDataSuccess>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataError {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataSuccess {
    #[prost(message, repeated, tag="1")]
    pub orgs: ::prost::alloc::vec::Vec<ImportDataSuccessOrg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataSuccessOrg {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub project_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub project_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub oidc_app_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub api_app_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="6")]
    pub human_user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="7")]
    pub machine_user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="8")]
    pub action_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="9")]
    pub trigger_actions: ::prost::alloc::vec::Vec<super::super::management::v1::SetTriggerActionsRequest>,
    #[prost(message, repeated, tag="10")]
    pub project_grants: ::prost::alloc::vec::Vec<ImportDataSuccessProjectGrant>,
    #[prost(message, repeated, tag="11")]
    pub user_grants: ::prost::alloc::vec::Vec<ImportDataSuccessUserGrant>,
    #[prost(string, repeated, tag="12")]
    pub org_members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="13")]
    pub project_members: ::prost::alloc::vec::Vec<ImportDataSuccessProjectMember>,
    #[prost(message, repeated, tag="14")]
    pub project_grant_members: ::prost::alloc::vec::Vec<ImportDataSuccessProjectGrantMember>,
    #[prost(string, repeated, tag="15")]
    pub oidc_ipds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="16")]
    pub jwt_idps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="17")]
    pub idp_links: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="18")]
    pub user_links: ::prost::alloc::vec::Vec<ImportDataSuccessUserLinks>,
    #[prost(message, repeated, tag="19")]
    pub user_metadata: ::prost::alloc::vec::Vec<ImportDataSuccessUserMetadata>,
    #[prost(string, repeated, tag="20")]
    pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="21")]
    pub app_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="22")]
    pub machine_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataSuccessProjectGrant {
    #[prost(string, tag="1")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataSuccessUserGrant {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataSuccessProjectMember {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataSuccessProjectGrantMember {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grant_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataSuccessUserLinks {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub external_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataSuccessUserMetadata {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataRequest {
    #[prost(string, repeated, tag="1")]
    pub org_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub excluded_org_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="3")]
    pub with_passwords: bool,
    #[prost(bool, tag="4")]
    pub with_otp: bool,
    #[prost(bool, tag="5")]
    pub response_output: bool,
    #[prost(message, optional, tag="6")]
    pub local_output: ::core::option::Option<export_data_request::LocalOutput>,
    #[prost(message, optional, tag="7")]
    pub s3_output: ::core::option::Option<export_data_request::S3Output>,
    #[prost(message, optional, tag="8")]
    pub gcs_output: ::core::option::Option<export_data_request::GcsOutput>,
    #[prost(string, tag="9")]
    pub timeout: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExportDataRequest`.
pub mod export_data_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalOutput {
        #[prost(string, tag="1")]
        pub path: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct S3Output {
        #[prost(string, tag="1")]
        pub path: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub endpoint: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub access_key_id: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub secret_access_key: ::prost::alloc::string::String,
        #[prost(bool, tag="5")]
        pub ssl: bool,
        #[prost(string, tag="6")]
        pub bucket: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsOutput {
        #[prost(string, tag="1")]
        pub bucket: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub serviceaccount_json: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub path: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataResponse {
    #[prost(message, repeated, tag="1")]
    pub orgs: ::prost::alloc::vec::Vec<DataOrg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsRequest {
    #[prost(uint64, tag="1")]
    pub sequence: u64,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    #[prost(bool, tag="3")]
    pub asc: bool,
    #[prost(string, tag="4")]
    pub editor_user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub event_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub aggregate_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="7")]
    pub aggregate_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="8")]
    pub resource_owner: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(message, optional, tag="9")]
    pub creation_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(oneof="list_events_request::CreationDateFilter", tags="10, 11")]
    pub creation_date_filter: ::core::option::Option<list_events_request::CreationDateFilter>,
}
/// Nested message and enum types in `ListEventsRequest`.
pub mod list_events_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreationDateRange {
        #[prost(message, optional, tag="1")]
        pub since: ::core::option::Option<::pbjson_types::Timestamp>,
        #[prost(message, optional, tag="2")]
        pub until: ::core::option::Option<::pbjson_types::Timestamp>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CreationDateFilter {
        #[prost(message, tag="10")]
        Range(CreationDateRange),
        #[prost(message, tag="11")]
        From(::pbjson_types::Timestamp),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsResponse {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<super::super::event::v1::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventTypesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventTypesResponse {
    #[prost(message, repeated, tag="1")]
    pub event_types: ::prost::alloc::vec::Vec<super::super::event::v1::EventType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAggregateTypesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAggregateTypesResponse {
    #[prost(message, repeated, tag="1")]
    pub aggregate_types: ::prost::alloc::vec::Vec<super::super::event::v1::AggregateType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateFeatureLoginDefaultOrgRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateFeatureLoginDefaultOrgResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMilestonesRequest {
    /// list limitations and ordering
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::v1::ListQuery>,
    /// the field the result is sorted
    #[prost(enumeration="super::super::milestone::v1::MilestoneFieldName", tag="2")]
    pub sorting_column: i32,
    /// criteria the client is looking for
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<super::super::milestone::v1::MilestoneQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMilestonesResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ListDetails>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<super::super::milestone::v1::Milestone>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRestrictionsRequest {
    #[prost(bool, optional, tag="1")]
    pub disallow_public_org_registration: ::core::option::Option<bool>,
    #[prost(message, optional, tag="2")]
    pub allowed_languages: ::core::option::Option<SelectLanguages>,
}
/// We have to wrap the languages list into a message so we can serialize empty lists.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectLanguages {
    #[prost(string, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRestrictionsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRestrictionsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRestrictionsResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(bool, tag="2")]
    pub disallow_public_org_registration: bool,
    #[prost(string, repeated, tag="3")]
    pub allowed_languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
include!("zitadel.admin.v1.tonic.rs");
// @@protoc_insertion_point(module)