// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authenticators {
    /// All of the user's usernames, which will be used for identification during authentication.
    #[prost(message, repeated, tag="1")]
    pub usernames: ::prost::alloc::vec::Vec<Username>,
    /// If the user has set a password, the time it was last changed will be returned.
    #[prost(message, optional, tag="2")]
    pub password: ::core::option::Option<Password>,
    /// Meta information about the user's WebAuthN authenticators.
    #[prost(message, repeated, tag="3")]
    pub web_auth_n: ::prost::alloc::vec::Vec<WebAuthN>,
    /// A list of the user's time-based one-time-password (TOTP) authenticators,
    /// incl. the name for identification.
    #[prost(message, repeated, tag="4")]
    pub totps: ::prost::alloc::vec::Vec<Totp>,
    /// A list of the user's one-time-password (OTP) SMS authenticators.
    #[prost(message, repeated, tag="5")]
    pub otp_sms: ::prost::alloc::vec::Vec<Otpsms>,
    /// A list of the user's one-time-password (OTP) Email authenticators.
    #[prost(message, repeated, tag="6")]
    pub otp_email: ::prost::alloc::vec::Vec<OtpEmail>,
    /// A list of the user's authentication keys. They can be used to authenticate e.g. by JWT Profile.
    #[prost(message, repeated, tag="7")]
    pub authentication_keys: ::prost::alloc::vec::Vec<AuthenticationKey>,
    /// A list of the user's linked identity providers (IDPs).
    #[prost(message, repeated, tag="8")]
    pub identity_providers: ::prost::alloc::vec::Vec<IdentityProvider>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Username {
    /// unique identifier of the username.
    #[prost(string, tag="1")]
    pub username_id: ::prost::alloc::string::String,
    /// The user's unique username. It is used for identification during authentication.
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    /// By default usernames must be unique across all organizations in an instance.
    /// This option allow to restrict the uniqueness to the user's own organization.
    /// As a result, this username can only be used if the authentication is limited
    /// to the corresponding organization.
    ///
    /// This can be useful if you provide multiple usernames for a single user, where one
    /// if specific to your organization, e.g.:
    /// - gigi-giraffe@zitadel.com (unique across organizations)
    /// - gigi-giraffe (unique only inside the ZITADEL organization)
    #[prost(bool, tag="3")]
    pub is_organization_specific: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUsername {
    /// Set the user's username. This will be used for identification during authentication.
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
    /// By default username must be unique across all organizations in an instance.
    /// This option allow to restrict the uniqueness to the user's own organization.
    /// As a result, this username can only be used if the authentication is limited
    /// to the corresponding organization.
    ///
    /// This can be useful if you provide multiple usernames for a single user, where one
    /// if specific to your organization, e.g.:
    /// - gigi-giraffe@zitadel.com (unique across organizations)
    /// - gigi-giraffe (unique only inside the ZITADEL organization)
    #[prost(bool, tag="2")]
    pub is_organization_specific: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Password {
    /// States the time the password was last changed.
    #[prost(message, optional, tag="1")]
    pub last_changed: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebAuthN {
    /// unique identifier of the WebAuthN authenticator.
    #[prost(string, tag="1")]
    pub web_auth_n_id: ::prost::alloc::string::String,
    /// Name of the WebAuthN authenticator. This is used for easier identification.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// State whether the WebAuthN registration has been completed.
    #[prost(bool, tag="3")]
    pub is_verified: bool,
    /// States if the user has been verified during the registration. Authentication with this device
    /// will be considered as multi factor authentication (MFA) without the need to check a password
    /// (typically known as Passkeys).
    /// Without user verification it will be a second factor authentication (2FA), typically done
    /// after a password check.
    ///
    /// More on WebAuthN User Verification: <https://www.w3.org/TR/webauthn/#user-verification>
    #[prost(bool, tag="4")]
    pub user_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Otpsms {
    /// unique identifier of the one-time-password (OTP) SMS authenticator.
    #[prost(string, tag="1")]
    pub otp_sms_id: ::prost::alloc::string::String,
    /// The phone number used for the OTP SMS authenticator.
    #[prost(string, tag="2")]
    pub phone: ::prost::alloc::string::String,
    /// State whether the OTP SMS registration has been completed.
    #[prost(bool, tag="3")]
    pub is_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OtpEmail {
    /// unique identifier of the one-time-password (OTP) Email authenticator.
    #[prost(string, tag="1")]
    pub otp_email_id: ::prost::alloc::string::String,
    /// The email address used for the OTP Email authenticator.
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    /// State whether the OTP Email registration has been completed.
    #[prost(bool, tag="3")]
    pub is_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Totp {
    /// unique identifier of the time-based one-time-password (TOTP) authenticator.
    #[prost(string, tag="1")]
    pub totp_id: ::prost::alloc::string::String,
    /// The name provided during registration. This is used for easier identification.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// State whether the TOTP registration has been completed.
    #[prost(bool, tag="3")]
    pub is_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationKey {
    /// ID is the read-only unique identifier of the authentication key.
    #[prost(string, tag="1")]
    pub authentication_key_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// the file type of the key
    #[prost(enumeration="AuthNKeyType", tag="3")]
    pub r#type: i32,
    /// After the expiration date, the key will no longer be usable for authentication.
    #[prost(message, optional, tag="4")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityProvider {
    /// IDP ID is the read-only unique identifier of the identity provider in ZITADEL.
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    /// IDP name is the name of the identity provider in ZITADEL.
    #[prost(string, tag="3")]
    pub idp_name: ::prost::alloc::string::String,
    /// The user ID represents the ID provided by the identity provider.
    /// This ID is used to link the user in ZITADEL with the identity provider.
    #[prost(string, tag="4")]
    pub user_id: ::prost::alloc::string::String,
    /// The username represents the username provided by the identity provider.
    #[prost(string, tag="5")]
    pub username: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAuthenticators {
    #[prost(message, repeated, tag="1")]
    pub usernames: ::prost::alloc::vec::Vec<SetUsername>,
    #[prost(message, optional, tag="2")]
    pub password: ::core::option::Option<SetPassword>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPassword {
    /// Provide if the user needs to change the password on the next use.
    #[prost(bool, tag="3")]
    pub change_required: bool,
    #[prost(oneof="set_password::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<set_password::Type>,
}
/// Nested message and enum types in `SetPassword`.
pub mod set_password {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Provide the plain text password. ZITADEL will take care to store it in a secure way (hash).
        #[prost(string, tag="1")]
        Password(::prost::alloc::string::String),
        /// Encoded hash of a password in Modular Crypt Format:
        /// <https://zitadel.com/docs/concepts/architecture/secrets#hashed-secrets.>
        #[prost(string, tag="2")]
        Hash(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPasswordResetEmail {
    /// Optionally set a url_template, which will be used in the password reset mail
    /// sent by ZITADEL to guide the user to your password change page.
    /// If no template is set, the default ZITADEL url will be used.
    #[prost(string, optional, tag="2")]
    pub url_template: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPasswordResetSms {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnPasswordResetCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticatorRegistrationCode {
    /// ID to the one time code generated by ZITADEL.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// one time code generated by ZITADEL.
    #[prost(string, tag="2")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendWebAuthNRegistrationLink {
    /// Optionally set a url_template, which will be used in the mail sent by ZITADEL
    /// to guide the user to your passkey registration page.
    /// If no template is set, the default ZITADEL url will be used.
    #[prost(string, optional, tag="1")]
    pub url_template: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnWebAuthNRegistrationCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedirectUrLs {
    /// URL to which the user will be redirected after a successful login.
    #[prost(string, tag="1")]
    pub success_url: ::prost::alloc::string::String,
    /// URL to which the user will be redirected after a failed login.
    #[prost(string, tag="2")]
    pub failure_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LdapCredentials {
    /// Username used to login through LDAP.
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
    /// Password used to login through LDAP.
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityProviderIntent {
    /// ID of the identity provider (IDP) intent.
    #[prost(string, tag="1")]
    pub idp_intent_id: ::prost::alloc::string::String,
    /// Token of the identity provider (IDP) intent.
    #[prost(string, tag="2")]
    pub idp_intent_token: ::prost::alloc::string::String,
    /// If the user was already federated and linked to a ZITADEL user, it's id will be returned.
    #[prost(string, optional, tag="3")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpInformation {
    /// ID of the identity provider.
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    /// ID of the user provided by the identity provider.
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    /// Username of the user provided by the identity provider.
    #[prost(string, tag="3")]
    pub user_name: ::prost::alloc::string::String,
    /// Complete information returned by the identity provider.
    #[prost(message, optional, tag="4")]
    pub raw_information: ::core::option::Option<::pbjson_types::Struct>,
    /// additional access information returned by the identity provider.
    #[prost(oneof="idp_information::Access", tags="5, 6, 7")]
    pub access: ::core::option::Option<idp_information::Access>,
}
/// Nested message and enum types in `IDPInformation`.
pub mod idp_information {
    /// additional access information returned by the identity provider.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Access {
        /// OAuth/OIDC access (and id_token) returned by the identity provider.
        #[prost(message, tag="5")]
        Oauth(super::IdpoAuthAccessInformation),
        /// LDAP entity attributes returned by the identity provider
        #[prost(message, tag="6")]
        Ldap(super::IdpldapAccessInformation),
        /// SAMLResponse returned by the identity provider
        #[prost(message, tag="7")]
        Saml(super::IdpsamlAccessInformation),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpoAuthAccessInformation {
    /// The access_token returned by the identity provider.
    #[prost(string, tag="1")]
    pub access_token: ::prost::alloc::string::String,
    /// In case the provider returned an id_token.
    #[prost(string, optional, tag="2")]
    pub id_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpldapAccessInformation {
    /// The attributes of the user returned by the identity provider.
    #[prost(message, optional, tag="1")]
    pub attributes: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpsamlAccessInformation {
    /// The SAML assertion returned by the identity provider.
    #[prost(bytes="vec", tag="1")]
    pub assertion: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpAuthenticator {
    /// ID of the identity provider
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    /// ID of the user provided by the identity provider
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    /// Username of the user provided by the identity provider.
    #[prost(string, tag="3")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthNKeyType {
    AuthnKeyTypeUnspecified = 0,
    AuthnKeyTypeJson = 1,
}
impl AuthNKeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthNKeyType::AuthnKeyTypeUnspecified => "AUTHN_KEY_TYPE_UNSPECIFIED",
            AuthNKeyType::AuthnKeyTypeJson => "AUTHN_KEY_TYPE_JSON",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTHN_KEY_TYPE_UNSPECIFIED" => Some(Self::AuthnKeyTypeUnspecified),
            "AUTHN_KEY_TYPE_JSON" => Some(Self::AuthnKeyTypeJson),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WebAuthNAuthenticatorType {
    WebAuthNAuthenticatorUnspecified = 0,
    WebAuthNAuthenticatorPlatform = 1,
    WebAuthNAuthenticatorCrossPlatform = 2,
}
impl WebAuthNAuthenticatorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WebAuthNAuthenticatorType::WebAuthNAuthenticatorUnspecified => "WEB_AUTH_N_AUTHENTICATOR_UNSPECIFIED",
            WebAuthNAuthenticatorType::WebAuthNAuthenticatorPlatform => "WEB_AUTH_N_AUTHENTICATOR_PLATFORM",
            WebAuthNAuthenticatorType::WebAuthNAuthenticatorCrossPlatform => "WEB_AUTH_N_AUTHENTICATOR_CROSS_PLATFORM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WEB_AUTH_N_AUTHENTICATOR_UNSPECIFIED" => Some(Self::WebAuthNAuthenticatorUnspecified),
            "WEB_AUTH_N_AUTHENTICATOR_PLATFORM" => Some(Self::WebAuthNAuthenticatorPlatform),
            "WEB_AUTH_N_AUTHENTICATOR_CROSS_PLATFORM" => Some(Self::WebAuthNAuthenticatorCrossPlatform),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    /// Email contact information of the user.
    #[prost(message, optional, tag="1")]
    pub email: ::core::option::Option<Email>,
    /// Phone contact information of the user.
    #[prost(message, optional, tag="2")]
    pub phone: ::core::option::Option<Phone>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Email {
    /// Email address of the user.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// IsVerified states if the email address has been verified to belong to the user.
    #[prost(bool, tag="2")]
    pub is_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Phone {
    /// Phone number of the user.
    #[prost(string, tag="1")]
    pub number: ::prost::alloc::string::String,
    /// IsVerified states if the phone number has been verified to belong to the user.
    #[prost(bool, tag="2")]
    pub is_verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContact {
    #[prost(message, optional, tag="1")]
    pub email: ::core::option::Option<SetEmail>,
    #[prost(message, optional, tag="2")]
    pub phone: ::core::option::Option<SetPhone>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetEmail {
    /// Set the email address.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// if no verification is specified, an email is sent with the default url
    #[prost(oneof="set_email::Verification", tags="2, 3, 4")]
    pub verification: ::core::option::Option<set_email::Verification>,
}
/// Nested message and enum types in `SetEmail`.
pub mod set_email {
    /// if no verification is specified, an email is sent with the default url
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        /// Let ZITADEL send the link to the user via email.
        #[prost(message, tag="2")]
        SendCode(super::SendEmailVerificationCode),
        /// Get the code back to provide it to the user in your preferred mechanism.
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnEmailVerificationCode),
        /// Set the email as already verified.
        #[prost(bool, tag="4")]
        IsVerified(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEmailVerificationCode {
    /// Optionally set a url_template, which will be used in the verification mail sent by ZITADEL
    /// to guide the user to your verification page.
    /// If no template is set, the default ZITADEL url will be used.
    #[prost(string, optional, tag="1")]
    pub url_template: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnEmailVerificationCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPhone {
    /// Set the user's phone number.
    #[prost(string, tag="1")]
    pub number: ::prost::alloc::string::String,
    /// if no verification is specified, a SMS is sent
    #[prost(oneof="set_phone::Verification", tags="2, 3, 4")]
    pub verification: ::core::option::Option<set_phone::Verification>,
}
/// Nested message and enum types in `SetPhone`.
pub mod set_phone {
    /// if no verification is specified, a SMS is sent
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        /// Let ZITADEL send the link to the user via SMS.
        #[prost(message, tag="2")]
        SendCode(super::SendPhoneVerificationCode),
        /// Get the code back to provide it to the user in your preferred mechanism.
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnPhoneVerificationCode),
        /// Set the phone as already verified.
        #[prost(bool, tag="4")]
        IsVerified(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPhoneVerificationCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnPhoneVerificationCode {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// ID is the read-only unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Details provide some base information (such as the last change date) of the user.
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// The user's authenticators. They are used to identify and authenticate the user
    /// during the authentication process.
    #[prost(message, optional, tag="3")]
    pub authenticators: ::core::option::Option<Authenticators>,
    /// Contact information for the user. ZITADEL will use this in case of internal notifications.
    #[prost(message, optional, tag="4")]
    pub contact: ::core::option::Option<Contact>,
    /// State of the user.
    #[prost(enumeration="State", tag="5")]
    pub state: i32,
    /// The schema the user and it's data is based on.
    #[prost(message, optional, tag="6")]
    pub schema: ::core::option::Option<Schema>,
    /// The user's data based on the provided schema.
    #[prost(message, optional, tag="7")]
    pub data: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// The unique identifier of the user schema.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The human readable name of the user schema.
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// The revision the user's data is based on of the revision.
    #[prost(uint32, tag="3")]
    pub revision: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    UserStateUnspecified = 0,
    UserStateActive = 1,
    UserStateInactive = 2,
    UserStateDeleted = 3,
    UserStateLocked = 4,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::UserStateUnspecified => "USER_STATE_UNSPECIFIED",
            State::UserStateActive => "USER_STATE_ACTIVE",
            State::UserStateInactive => "USER_STATE_INACTIVE",
            State::UserStateDeleted => "USER_STATE_DELETED",
            State::UserStateLocked => "USER_STATE_LOCKED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_STATE_UNSPECIFIED" => Some(Self::UserStateUnspecified),
            "USER_STATE_ACTIVE" => Some(Self::UserStateActive),
            "USER_STATE_INACTIVE" => Some(Self::UserStateInactive),
            "USER_STATE_DELETED" => Some(Self::UserStateDeleted),
            "USER_STATE_LOCKED" => Some(Self::UserStateLocked),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    #[prost(oneof="search_query::Query", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub query: ::core::option::Option<search_query::Query>,
}
/// Nested message and enum types in `SearchQuery`.
pub mod search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// Union the results of each sub query ('OR').
        #[prost(message, tag="1")]
        OrQuery(super::OrQuery),
        /// Limit the result to match all sub queries ('AND').
        /// Note that if you specify multiple queries, they will be implicitly used as andQueries.
        /// Use the andQuery in combination with orQuery and notQuery.
        #[prost(message, tag="2")]
        AndQuery(super::AndQuery),
        /// Exclude / Negate the result of the sub query ('NOT').
        #[prost(message, tag="3")]
        NotQuery(::prost::alloc::boxed::Box<super::NotQuery>),
        /// Limit the result to a specific user ID.
        #[prost(message, tag="4")]
        UserIdQuery(super::UserIdQuery),
        /// Limit the result to a specific organization.
        #[prost(message, tag="5")]
        OrganizationIdQuery(super::OrganizationIdQuery),
        /// Limit the result to a specific username.
        #[prost(message, tag="6")]
        UsernameQuery(super::UsernameQuery),
        /// Limit the result to a specific contact email.
        #[prost(message, tag="7")]
        EmailQuery(super::EmailQuery),
        /// Limit the result to a specific contact phone.
        #[prost(message, tag="8")]
        PhoneQuery(super::PhoneQuery),
        /// Limit the result to a specific state of the user.
        #[prost(message, tag="9")]
        StateQuery(super::StateQuery),
        /// Limit the result to a specific schema ID.
        #[prost(message, tag="10")]
        SchemaIdQuery(super::SchemaIdQuery),
        /// Limit the result to a specific schema type.
        #[prost(message, tag="11")]
        SchemaTypeQuery(super::SchemaTypeQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrQuery {
    #[prost(message, repeated, tag="1")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndQuery {
    #[prost(message, repeated, tag="1")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotQuery {
    #[prost(message, optional, boxed, tag="1")]
    pub query: ::core::option::Option<::prost::alloc::boxed::Box<SearchQuery>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdQuery {
    /// Defines the ID of the user to query for.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the id query.
    #[prost(enumeration="super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationIdQuery {
    /// Defines the ID of the organization to query for.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the id query.
    #[prost(enumeration="super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsernameQuery {
    /// Defines the username to query for.
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the username query.
    #[prost(enumeration="super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
    /// Defines that the username must only be unique in the organisation.
    #[prost(bool, tag="3")]
    pub is_organization_specific: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmailQuery {
    /// Defines the email of the user to query for.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the email query.
    #[prost(enumeration="super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoneQuery {
    /// Defines the phone of the user to query for.
    #[prost(string, tag="1")]
    pub number: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the phone query.
    #[prost(enumeration="super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateQuery {
    /// Defines the state to query for.
    #[prost(enumeration="State", tag="1")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaIdQuery {
    /// Defines the ID of the schema to query for.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaTypeQuery {
    /// Defines which type to query for.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the type query.
    #[prost(enumeration="super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldName {
    Unspecified = 0,
    Id = 1,
    CreationDate = 2,
    ChangeDate = 3,
    Email = 4,
    Phone = 5,
    State = 6,
    SchemaId = 7,
    SchemaType = 8,
}
impl FieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FieldName::Unspecified => "FIELD_NAME_UNSPECIFIED",
            FieldName::Id => "FIELD_NAME_ID",
            FieldName::CreationDate => "FIELD_NAME_CREATION_DATE",
            FieldName::ChangeDate => "FIELD_NAME_CHANGE_DATE",
            FieldName::Email => "FIELD_NAME_EMAIL",
            FieldName::Phone => "FIELD_NAME_PHONE",
            FieldName::State => "FIELD_NAME_STATE",
            FieldName::SchemaId => "FIELD_NAME_SCHEMA_ID",
            FieldName::SchemaType => "FIELD_NAME_SCHEMA_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "FIELD_NAME_ID" => Some(Self::Id),
            "FIELD_NAME_CREATION_DATE" => Some(Self::CreationDate),
            "FIELD_NAME_CHANGE_DATE" => Some(Self::ChangeDate),
            "FIELD_NAME_EMAIL" => Some(Self::Email),
            "FIELD_NAME_PHONE" => Some(Self::Phone),
            "FIELD_NAME_STATE" => Some(Self::State),
            "FIELD_NAME_SCHEMA_ID" => Some(Self::SchemaId),
            "FIELD_NAME_SCHEMA_TYPE" => Some(Self::SchemaType),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersRequest {
    /// list limitations and ordering.
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::object::v2::ListQuery>,
    /// the field the result is sorted.
    #[prost(enumeration="FieldName", tag="2")]
    pub sorting_column: i32,
    /// Define the criteria to query for.
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersResponse {
    /// Details provides information about the returned result including total amount found.
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::ListDetails>,
    /// States by which field the results are sorted.
    #[prost(enumeration="FieldName", tag="2")]
    pub sorting_column: i32,
    /// The result contains the user schemas, which matched the queries.
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByIdRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByIdResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserRequest {
    /// Optionally set a unique identifier of the user. If unset, ZITADEL will take care of it.
    #[prost(string, optional, tag="1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Set the organization the user belongs to.
    #[prost(message, optional, tag="2")]
    pub organization: ::core::option::Option<super::super::object::v2::Organization>,
    /// Set the initial authenticators of the user.
    #[prost(message, optional, tag="3")]
    pub authenticators: ::core::option::Option<SetAuthenticators>,
    /// Set the contact information (email, phone) for the user.
    #[prost(message, optional, tag="4")]
    pub contact: ::core::option::Option<SetContact>,
    /// Define the schema the user's data schema by providing it's ID.
    #[prost(string, tag="5")]
    pub schema_id: ::prost::alloc::string::String,
    /// Provide data about the user. It will be validated based on the specified schema.
    #[prost(message, optional, tag="6")]
    pub data: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResponse {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// The email code will be set if a contact email was set with a return_code verification option.
    #[prost(string, optional, tag="3")]
    pub email_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The phone code will be set if a contact phone was set with a return_code verification option.
    #[prost(string, optional, tag="4")]
    pub phone_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserRequest {
    ///   unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Add or update the contact information (email, phone) for the user if needed.
    #[prost(message, optional, tag="4")]
    pub contact: ::core::option::Option<SetContact>,
    /// Change the schema the user's data schema by providing it's ID if needed.
    #[prost(string, optional, tag="5")]
    pub schema_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Update the user data if needed. It will be validated based on the specified schema.
    #[prost(message, optional, tag="6")]
    pub data: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// The email code will be set if a contact email was set with a return_code verification option.
    #[prost(string, optional, tag="3")]
    pub email_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The phone code will be set if a contact phone was set with a return_code verification option.
    #[prost(string, optional, tag="4")]
    pub phone_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockUserRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockUserRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContactEmailRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Set the user's contact email and it's verification state.
    #[prost(message, optional, tag="2")]
    pub email: ::core::option::Option<SetEmail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContactEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// The verification code will be set if a contact email was set with a return_code verification option.
    #[prost(string, optional, tag="3")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyContactEmailRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Set the verification code generated during the set contact email request.
    #[prost(string, tag="2")]
    pub verification_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyContactEmailResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendContactEmailCodeRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// if no verification is specified, an email is sent
    #[prost(oneof="resend_contact_email_code_request::Verification", tags="2, 3")]
    pub verification: ::core::option::Option<resend_contact_email_code_request::Verification>,
}
/// Nested message and enum types in `ResendContactEmailCodeRequest`.
pub mod resend_contact_email_code_request {
    /// if no verification is specified, an email is sent
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        /// Let ZITADEL send the link to the user via email.
        #[prost(message, tag="2")]
        SendCode(super::SendEmailVerificationCode),
        /// Get the code back to provide it to the user in your preferred mechanism.
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnEmailVerificationCode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendContactEmailCodeResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// in case the verification was set to return_code, the code will be returned.
    #[prost(string, optional, tag="2")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContactPhoneRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Set the user's contact phone and it's verification state.
    #[prost(message, optional, tag="2")]
    pub phone: ::core::option::Option<SetPhone>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContactPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// The phone verification code will be set if a contact phone was set with a return_code verification option.
    #[prost(string, optional, tag="3")]
    pub email_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyContactPhoneRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Set the verification code generated during the set contact phone request.
    #[prost(string, tag="2")]
    pub verification_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyContactPhoneResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendContactPhoneCodeRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// if no verification is specified, a SMS is sent
    #[prost(oneof="resend_contact_phone_code_request::Verification", tags="2, 3")]
    pub verification: ::core::option::Option<resend_contact_phone_code_request::Verification>,
}
/// Nested message and enum types in `ResendContactPhoneCodeRequest`.
pub mod resend_contact_phone_code_request {
    /// if no verification is specified, a SMS is sent
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        /// Let ZITADEL send the link to the user via SMS.
        #[prost(message, tag="2")]
        SendCode(super::SendPhoneVerificationCode),
        /// Get the code back to provide it to the user in your preferred mechanism.
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnPhoneVerificationCode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendContactPhoneCodeResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// in case the verification was set to return_code, the code will be returned.
    #[prost(string, optional, tag="2")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUsernameRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Set the user's new username.
    #[prost(message, optional, tag="2")]
    pub username: ::core::option::Option<SetUsername>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUsernameResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// unique identifier of the username.
    #[prost(string, tag="2")]
    pub username_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUsernameRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the username.
    #[prost(string, tag="2")]
    pub username_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUsernameResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPasswordRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Provide the new password (in plain text or as hash).
    #[prost(message, optional, tag="2")]
    pub new_password: ::core::option::Option<SetPassword>,
    /// If neither, the current password nor a verification code generated by the PasswordReset is provided,
    /// the user must be granted permission to set a password.
    #[prost(oneof="set_password_request::Verification", tags="3, 4")]
    pub verification: ::core::option::Option<set_password_request::Verification>,
}
/// Nested message and enum types in `SetPasswordRequest`.
pub mod set_password_request {
    /// If neither, the current password nor a verification code generated by the PasswordReset is provided,
    /// the user must be granted permission to set a password.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Verification {
        /// Provide the current password to verify you're allowed to change the password.
        #[prost(string, tag="3")]
        CurrentPassword(::prost::alloc::string::String),
        /// Or provider the verification code generated during password reset request.
        #[prost(string, tag="4")]
        VerificationCode(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPasswordResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPasswordResetRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// If no medium is specified, an email is sent with the default url.
    #[prost(oneof="request_password_reset_request::Medium", tags="2, 3, 4")]
    pub medium: ::core::option::Option<request_password_reset_request::Medium>,
}
/// Nested message and enum types in `RequestPasswordResetRequest`.
pub mod request_password_reset_request {
    /// If no medium is specified, an email is sent with the default url.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Medium {
        /// Let ZITADEL send the link to the user via email.
        #[prost(message, tag="2")]
        SendEmail(super::SendPasswordResetEmail),
        /// Let ZITADEL send the link to the user via SMS.
        #[prost(message, tag="3")]
        SendSms(super::SendPasswordResetSms),
        /// Get the code back to provide it to the user in your preferred mechanism.
        #[prost(message, tag="4")]
        ReturnCode(super::ReturnPasswordResetCode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPasswordResetResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// In case the medium was set to return_code, the code will be returned.
    #[prost(string, optional, tag="2")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartWebAuthNRegistrationRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Domain on which the user currently is or will be authenticated.
    #[prost(string, tag="4")]
    pub domain: ::prost::alloc::string::String,
    /// Optionally specify the authenticator type of the passkey device (platform or cross-platform).
    /// If none is provided, both values are allowed.
    #[prost(enumeration="WebAuthNAuthenticatorType", tag="3")]
    pub authenticator_type: i32,
    /// Optionally provide a one time code generated by ZITADEL.
    /// This is required to start the passkey registration without user authentication.
    #[prost(message, optional, tag="2")]
    pub code: ::core::option::Option<AuthenticatorRegistrationCode>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartWebAuthNRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// unique identifier of the WebAuthN registration.
    #[prost(string, tag="2")]
    pub web_auth_n_id: ::prost::alloc::string::String,
    /// Options for Credential Creation (dictionary PublicKeyCredentialCreationOptions).
    /// Generated helper methods transform the field to JSON, for use in a WebauthN client.
    /// See also:  <https://www.w3.org/TR/webauthn/#dictdef-publickeycredentialcreationoptions>
    #[prost(message, optional, tag="3")]
    pub public_key_credential_creation_options: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyWebAuthNRegistrationRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the WebAuthN registration, which was returned in the start webauthn registration.
    #[prost(string, tag="2")]
    pub web_auth_n_id: ::prost::alloc::string::String,
    /// PublicKeyCredential Interface.
    /// Generated helper methods populate the field from JSON created by a WebAuthN client.
    /// See also:  <https://www.w3.org/TR/webauthn/#publickeycredential>
    #[prost(message, optional, tag="3")]
    pub public_key_credential: ::core::option::Option<::pbjson_types::Struct>,
    /// Provide a name for the WebAuthN device. This will help identify it in the future.
    #[prost(string, tag="4")]
    pub web_auth_n_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyWebAuthNRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebAuthNRegistrationLinkRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// if no medium is specified, an email is sent with the default url.
    #[prost(oneof="create_web_auth_n_registration_link_request::Medium", tags="2, 3")]
    pub medium: ::core::option::Option<create_web_auth_n_registration_link_request::Medium>,
}
/// Nested message and enum types in `CreateWebAuthNRegistrationLinkRequest`.
pub mod create_web_auth_n_registration_link_request {
    /// if no medium is specified, an email is sent with the default url.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Medium {
        /// Let ZITADEL send the link to the user via email.
        #[prost(message, tag="2")]
        SendLink(super::SendWebAuthNRegistrationLink),
        /// Get the code back to provide it to the user in your preferred mechanism.
        #[prost(message, tag="3")]
        ReturnCode(super::ReturnWebAuthNRegistrationCode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebAuthNRegistrationLinkResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// In case the medium was set to return_code, the code will be returned.
    #[prost(message, optional, tag="2")]
    pub code: ::core::option::Option<AuthenticatorRegistrationCode>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWebAuthNAuthenticatorRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the WebAuthN authenticator.
    #[prost(string, tag="2")]
    pub web_auth_n_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWebAuthNAuthenticatorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartTotpRegistrationRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartTotpRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// unique identifier of the TOTP registration.
    #[prost(string, tag="2")]
    pub totp_id: ::prost::alloc::string::String,
    /// The TOTP URI, which can be used to create a QR Code for scanning with an authenticator app.
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    /// The TOTP secret, which can be used for manually adding in an authenticator app.
    #[prost(string, tag="4")]
    pub secret: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyTotpRegistrationRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the TOTP registration, which was returned in the start TOTP registration.
    #[prost(string, tag="2")]
    pub totp_id: ::prost::alloc::string::String,
    /// Code generated by TOTP app or device.
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyTotpRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTotpAuthenticatorRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the TOTP authenticator.
    #[prost(string, tag="2")]
    pub totp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTotpAuthenticatorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOtpsmsAuthenticatorRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Set the user's phone for the OTP SMS authenticator and it's verification state.
    #[prost(message, optional, tag="2")]
    pub phone: ::core::option::Option<SetPhone>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOtpsmsAuthenticatorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// unique identifier of the OTP SMS registration.
    #[prost(string, tag="2")]
    pub otp_sms_id: ::prost::alloc::string::String,
    /// The OTP verification code will be set if a phone was set with a return_code verification option.
    #[prost(string, optional, tag="3")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyOtpsmsRegistrationRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the OTP SMS registration, which was returned in the add OTP SMS.
    #[prost(string, tag="2")]
    pub otp_sms_id: ::prost::alloc::string::String,
    /// Set the verification code generated during the add OTP SMS request.
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyOtpsmsRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOtpsmsAuthenticatorRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the OTP SMS authenticator.
    #[prost(string, tag="2")]
    pub otp_sms_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOtpsmsAuthenticatorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOtpEmailAuthenticatorRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// Set the user's email for the OTP Email authenticator and it's verification state.
    #[prost(message, optional, tag="2")]
    pub email: ::core::option::Option<SetEmail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOtpEmailAuthenticatorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// unique identifier of the OTP Email registration.
    #[prost(string, tag="2")]
    pub otp_email_id: ::prost::alloc::string::String,
    /// The OTP verification code will be set if a email was set with a return_code verification option.
    #[prost(string, optional, tag="3")]
    pub verification_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyOtpEmailRegistrationRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the OTP Email registration, which was returned in the add OTP Email.
    #[prost(string, tag="2")]
    pub otp_email_id: ::prost::alloc::string::String,
    /// Set the verification code generated during the add OTP Email request.
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyOtpEmailRegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOtpEmailAuthenticatorRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the OTP Email authenticator.
    #[prost(string, tag="2")]
    pub otp_email_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveOtpEmailAuthenticatorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartIdentityProviderIntentRequest {
    /// ID of an existing identity provider (IDP).
    #[prost(string, tag="1")]
    pub idp_id: ::prost::alloc::string::String,
    #[prost(oneof="start_identity_provider_intent_request::Content", tags="2, 3")]
    pub content: ::core::option::Option<start_identity_provider_intent_request::Content>,
}
/// Nested message and enum types in `StartIdentityProviderIntentRequest`.
pub mod start_identity_provider_intent_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag="2")]
        Urls(super::RedirectUrLs),
        #[prost(message, tag="3")]
        Ldap(super::LdapCredentials),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartIdentityProviderIntentResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// the next step to take in the idp intent flow.
    #[prost(oneof="start_identity_provider_intent_response::NextStep", tags="2, 3, 4")]
    pub next_step: ::core::option::Option<start_identity_provider_intent_response::NextStep>,
}
/// Nested message and enum types in `StartIdentityProviderIntentResponse`.
pub mod start_identity_provider_intent_response {
    /// the next step to take in the idp intent flow.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NextStep {
        /// The authentication URL to which the client should redirect.
        #[prost(string, tag="2")]
        AuthUrl(::prost::alloc::string::String),
        /// The Start Intent directly succeeded and returned the IDP Intent.
        /// Further information can be retrieved by using the retrieve identity provider intent request.
        #[prost(message, tag="3")]
        IdpIntent(super::IdentityProviderIntent),
        /// The HTML form with the embedded POST call information to render and execute.
        #[prost(bytes, tag="4")]
        PostForm(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveIdentityProviderIntentRequest {
    /// ID of the identity provider (IDP) intent, previously returned on the success response of the start identity provider intent.
    #[prost(string, tag="1")]
    pub idp_intent_id: ::prost::alloc::string::String,
    /// Token of the identity provider (IDP) intent, previously returned on the success response of the start identity provider intent.
    #[prost(string, tag="2")]
    pub idp_intent_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveIdentityProviderIntentResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// Information returned by the identity provider (IDP) such as the identification of the user
    /// and detailed / profile information.
    #[prost(message, optional, tag="2")]
    pub idp_information: ::core::option::Option<IdpInformation>,
    /// If the user was already federated and linked to a ZITADEL user, it's id will be returned.
    #[prost(string, optional, tag="3")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddIdpAuthenticatorRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub idp_authenticator: ::core::option::Option<IdpAuthenticator>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddIdpAuthenticatorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIdpAuthenticatorRequest {
    /// unique identifier of the user.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// unique identifier of the identity provider (IDP) authenticator.
    #[prost(string, tag="2")]
    pub idp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIdpAuthenticatorResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
}
include!("zitadel.user.v3alpha.tonic.rs");
// @@protoc_insertion_point(module)