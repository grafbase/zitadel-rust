// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="UserState", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub login_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub preferred_login_name: ::prost::alloc::string::String,
    #[prost(oneof="user::Type", tags="7, 8")]
    pub r#type: ::core::option::Option<user::Type>,
}
/// Nested message and enum types in `User`.
pub mod user {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="7")]
        Human(super::Human),
        #[prost(message, tag="8")]
        Machine(super::Machine),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Human {
    #[prost(message, optional, tag="1")]
    pub profile: ::core::option::Option<Profile>,
    #[prost(message, optional, tag="2")]
    pub email: ::core::option::Option<Email>,
    #[prost(message, optional, tag="3")]
    pub phone: ::core::option::Option<Phone>,
    /// The time the user last changed their password.
    #[prost(message, optional, tag="4")]
    pub password_changed: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Machine {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub has_secret: bool,
    #[prost(enumeration="AccessTokenType", tag="4")]
    pub access_token_type: i32,
}
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
    #[prost(enumeration="Gender", tag="6")]
    pub gender: i32,
    #[prost(string, tag="7")]
    pub avatar_url: ::prost::alloc::string::String,
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
pub struct SearchQuery {
    #[prost(oneof="search_query::Query", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14")]
    pub query: ::core::option::Option<search_query::Query>,
}
/// Nested message and enum types in `SearchQuery`.
pub mod search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        UserNameQuery(super::UserNameQuery),
        #[prost(message, tag="2")]
        FirstNameQuery(super::FirstNameQuery),
        #[prost(message, tag="3")]
        LastNameQuery(super::LastNameQuery),
        #[prost(message, tag="4")]
        NickNameQuery(super::NickNameQuery),
        #[prost(message, tag="5")]
        DisplayNameQuery(super::DisplayNameQuery),
        #[prost(message, tag="6")]
        EmailQuery(super::EmailQuery),
        #[prost(message, tag="7")]
        StateQuery(super::StateQuery),
        #[prost(message, tag="8")]
        TypeQuery(super::TypeQuery),
        #[prost(message, tag="9")]
        LoginNameQuery(super::LoginNameQuery),
        #[prost(message, tag="10")]
        InUserIdsQuery(super::InUserIdQuery),
        #[prost(message, tag="11")]
        OrQuery(super::OrQuery),
        #[prost(message, tag="12")]
        AndQuery(super::AndQuery),
        #[prost(message, tag="13")]
        NotQuery(::prost::alloc::boxed::Box<super::NotQuery>),
        #[prost(message, tag="14")]
        InUserEmailsQuery(super::InUserEmailsQuery),
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
pub struct InUserIdQuery {
    #[prost(string, repeated, tag="1")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InUserEmailsQuery {
    #[prost(string, repeated, tag="1")]
    pub user_emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserNameQuery {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirstNameQuery {
    #[prost(string, tag="1")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastNameQuery {
    #[prost(string, tag="1")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NickNameQuery {
    #[prost(string, tag="1")]
    pub nick_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayNameQuery {
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmailQuery {
    #[prost(string, tag="1")]
    pub email_address: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginNameQuery {
    #[prost(string, tag="1")]
    pub login_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
/// UserStateQuery always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateQuery {
    #[prost(enumeration="UserState", tag="1")]
    pub state: i32,
}
/// UserTypeQuery always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeQuery {
    #[prost(enumeration="Type", tag="1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthFactor {
    #[prost(enumeration="AuthFactorState", tag="1")]
    pub state: i32,
    #[prost(oneof="auth_factor::Type", tags="2, 3, 4, 5")]
    pub r#type: ::core::option::Option<auth_factor::Type>,
}
/// Nested message and enum types in `AuthFactor`.
pub mod auth_factor {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="2")]
        Otp(super::AuthFactorOtp),
        #[prost(message, tag="3")]
        U2f(super::AuthFactorU2f),
        #[prost(message, tag="4")]
        OtpSms(super::AuthFactorOtpsms),
        #[prost(message, tag="5")]
        OtpEmail(super::AuthFactorOtpEmail),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthFactorOtp {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthFactorOtpsms {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthFactorOtpEmail {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthFactorU2f {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebAuthNKey {
    #[prost(bytes="vec", tag="1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebAuthNVerification {
    #[prost(bytes="vec", tag="1")]
    pub public_key_credential: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub token_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebAuthNToken {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="AuthFactorState", tag="2")]
    pub state: i32,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Membership {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(oneof="membership::Type", tags="5, 6, 7, 8")]
    pub r#type: ::core::option::Option<membership::Type>,
}
/// Nested message and enum types in `Membership`.
pub mod membership {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(bool, tag="5")]
        Iam(bool),
        #[prost(string, tag="6")]
        OrgId(::prost::alloc::string::String),
        #[prost(string, tag="7")]
        ProjectId(::prost::alloc::string::String),
        #[prost(string, tag="8")]
        ProjectGrantId(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipQuery {
    #[prost(oneof="membership_query::Query", tags="1, 2, 3, 4")]
    pub query: ::core::option::Option<membership_query::Query>,
}
/// Nested message and enum types in `MembershipQuery`.
pub mod membership_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        OrgQuery(super::MembershipOrgQuery),
        #[prost(message, tag="2")]
        ProjectQuery(super::MembershipProjectQuery),
        #[prost(message, tag="3")]
        ProjectGrantQuery(super::MembershipProjectGrantQuery),
        #[prost(message, tag="4")]
        IamQuery(super::MembershipIamQuery),
    }
}
/// this query always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipOrgQuery {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
/// this query always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipProjectQuery {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
/// this query always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipProjectGrantQuery {
    #[prost(string, tag="1")]
    pub project_grant_id: ::prost::alloc::string::String,
}
/// this query always equals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipIamQuery {
    #[prost(bool, tag="1")]
    pub iam: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub agent_id: ::prost::alloc::string::String,
    #[prost(enumeration="SessionState", tag="3")]
    pub auth_state: i32,
    #[prost(string, tag="4")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub login_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="10")]
    pub avatar_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshToken {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub auth_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub idle_expiration: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub expiration: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, repeated, tag="7")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="8")]
    pub audience: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonalAccessToken {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="3")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, repeated, tag="4")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrant {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, repeated, tag="3")]
    pub role_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="UserGrantState", tag="4")]
    pub state: i32,
    #[prost(string, tag="5")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub org_name: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub org_domain: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub project_grant_id: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub avatar_url: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub preferred_login_name: ::prost::alloc::string::String,
    #[prost(enumeration="Type", tag="19")]
    pub user_type: i32,
    #[prost(string, tag="20")]
    pub granted_org_id: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub granted_org_name: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub granted_org_domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantQuery {
    #[prost(oneof="user_grant_query::Query", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14")]
    pub query: ::core::option::Option<user_grant_query::Query>,
}
/// Nested message and enum types in `UserGrantQuery`.
pub mod user_grant_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        ProjectIdQuery(super::UserGrantProjectIdQuery),
        #[prost(message, tag="2")]
        UserIdQuery(super::UserGrantUserIdQuery),
        #[prost(message, tag="3")]
        WithGrantedQuery(super::UserGrantWithGrantedQuery),
        #[prost(message, tag="4")]
        RoleKeyQuery(super::UserGrantRoleKeyQuery),
        #[prost(message, tag="5")]
        ProjectGrantIdQuery(super::UserGrantProjectGrantIdQuery),
        #[prost(message, tag="6")]
        UserNameQuery(super::UserGrantUserNameQuery),
        #[prost(message, tag="7")]
        FirstNameQuery(super::UserGrantFirstNameQuery),
        #[prost(message, tag="8")]
        LastNameQuery(super::UserGrantLastNameQuery),
        #[prost(message, tag="9")]
        EmailQuery(super::UserGrantEmailQuery),
        #[prost(message, tag="10")]
        OrgNameQuery(super::UserGrantOrgNameQuery),
        #[prost(message, tag="11")]
        OrgDomainQuery(super::UserGrantOrgDomainQuery),
        #[prost(message, tag="12")]
        ProjectNameQuery(super::UserGrantProjectNameQuery),
        #[prost(message, tag="13")]
        DisplayNameQuery(super::UserGrantDisplayNameQuery),
        #[prost(message, tag="14")]
        UserTypeQuery(super::UserGrantUserTypeQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantProjectIdQuery {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantUserIdQuery {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantWithGrantedQuery {
    #[prost(bool, tag="1")]
    pub with_granted: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantRoleKeyQuery {
    #[prost(string, tag="1")]
    pub role_key: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantProjectGrantIdQuery {
    #[prost(string, tag="1")]
    pub project_grant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantUserNameQuery {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantFirstNameQuery {
    #[prost(string, tag="1")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantLastNameQuery {
    #[prost(string, tag="1")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantEmailQuery {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantOrgNameQuery {
    #[prost(string, tag="1")]
    pub org_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantOrgDomainQuery {
    #[prost(string, tag="1")]
    pub org_domain: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantProjectNameQuery {
    #[prost(string, tag="1")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantDisplayNameQuery {
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGrantUserTypeQuery {
    #[prost(enumeration="Type", tag="1")]
    pub r#type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
    Deleted = 3,
    Locked = 4,
    Suspend = 5,
    Initial = 6,
}
impl UserState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserState::Unspecified => "USER_STATE_UNSPECIFIED",
            UserState::Active => "USER_STATE_ACTIVE",
            UserState::Inactive => "USER_STATE_INACTIVE",
            UserState::Deleted => "USER_STATE_DELETED",
            UserState::Locked => "USER_STATE_LOCKED",
            UserState::Suspend => "USER_STATE_SUSPEND",
            UserState::Initial => "USER_STATE_INITIAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "USER_STATE_ACTIVE" => Some(Self::Active),
            "USER_STATE_INACTIVE" => Some(Self::Inactive),
            "USER_STATE_DELETED" => Some(Self::Deleted),
            "USER_STATE_LOCKED" => Some(Self::Locked),
            "USER_STATE_SUSPEND" => Some(Self::Suspend),
            "USER_STATE_INITIAL" => Some(Self::Initial),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
    Diverse = 3,
}
impl Gender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Gender::Unspecified => "GENDER_UNSPECIFIED",
            Gender::Female => "GENDER_FEMALE",
            Gender::Male => "GENDER_MALE",
            Gender::Diverse => "GENDER_DIVERSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GENDER_UNSPECIFIED" => Some(Self::Unspecified),
            "GENDER_FEMALE" => Some(Self::Female),
            "GENDER_MALE" => Some(Self::Male),
            "GENDER_DIVERSE" => Some(Self::Diverse),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessTokenType {
    Bearer = 0,
    Jwt = 1,
}
impl AccessTokenType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessTokenType::Bearer => "ACCESS_TOKEN_TYPE_BEARER",
            AccessTokenType::Jwt => "ACCESS_TOKEN_TYPE_JWT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_TOKEN_TYPE_BEARER" => Some(Self::Bearer),
            "ACCESS_TOKEN_TYPE_JWT" => Some(Self::Jwt),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    Unspecified = 0,
    Human = 1,
    Machine = 2,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "TYPE_UNSPECIFIED",
            Type::Human => "TYPE_HUMAN",
            Type::Machine => "TYPE_MACHINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TYPE_HUMAN" => Some(Self::Human),
            "TYPE_MACHINE" => Some(Self::Machine),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserFieldName {
    Unspecified = 0,
    UserName = 1,
    FirstName = 2,
    LastName = 3,
    NickName = 4,
    DisplayName = 5,
    Email = 6,
    State = 7,
    Type = 8,
    CreationDate = 9,
}
impl UserFieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserFieldName::Unspecified => "USER_FIELD_NAME_UNSPECIFIED",
            UserFieldName::UserName => "USER_FIELD_NAME_USER_NAME",
            UserFieldName::FirstName => "USER_FIELD_NAME_FIRST_NAME",
            UserFieldName::LastName => "USER_FIELD_NAME_LAST_NAME",
            UserFieldName::NickName => "USER_FIELD_NAME_NICK_NAME",
            UserFieldName::DisplayName => "USER_FIELD_NAME_DISPLAY_NAME",
            UserFieldName::Email => "USER_FIELD_NAME_EMAIL",
            UserFieldName::State => "USER_FIELD_NAME_STATE",
            UserFieldName::Type => "USER_FIELD_NAME_TYPE",
            UserFieldName::CreationDate => "USER_FIELD_NAME_CREATION_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "USER_FIELD_NAME_USER_NAME" => Some(Self::UserName),
            "USER_FIELD_NAME_FIRST_NAME" => Some(Self::FirstName),
            "USER_FIELD_NAME_LAST_NAME" => Some(Self::LastName),
            "USER_FIELD_NAME_NICK_NAME" => Some(Self::NickName),
            "USER_FIELD_NAME_DISPLAY_NAME" => Some(Self::DisplayName),
            "USER_FIELD_NAME_EMAIL" => Some(Self::Email),
            "USER_FIELD_NAME_STATE" => Some(Self::State),
            "USER_FIELD_NAME_TYPE" => Some(Self::Type),
            "USER_FIELD_NAME_CREATION_DATE" => Some(Self::CreationDate),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthFactorState {
    Unspecified = 0,
    NotReady = 1,
    Ready = 2,
    Removed = 3,
}
impl AuthFactorState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthFactorState::Unspecified => "AUTH_FACTOR_STATE_UNSPECIFIED",
            AuthFactorState::NotReady => "AUTH_FACTOR_STATE_NOT_READY",
            AuthFactorState::Ready => "AUTH_FACTOR_STATE_READY",
            AuthFactorState::Removed => "AUTH_FACTOR_STATE_REMOVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTH_FACTOR_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTH_FACTOR_STATE_NOT_READY" => Some(Self::NotReady),
            "AUTH_FACTOR_STATE_READY" => Some(Self::Ready),
            "AUTH_FACTOR_STATE_REMOVED" => Some(Self::Removed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionState {
    Unspecified = 0,
    Active = 1,
    Terminated = 2,
}
impl SessionState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SessionState::Unspecified => "SESSION_STATE_UNSPECIFIED",
            SessionState::Active => "SESSION_STATE_ACTIVE",
            SessionState::Terminated => "SESSION_STATE_TERMINATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SESSION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "SESSION_STATE_ACTIVE" => Some(Self::Active),
            "SESSION_STATE_TERMINATED" => Some(Self::Terminated),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserGrantState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
}
impl UserGrantState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserGrantState::Unspecified => "USER_GRANT_STATE_UNSPECIFIED",
            UserGrantState::Active => "USER_GRANT_STATE_ACTIVE",
            UserGrantState::Inactive => "USER_GRANT_STATE_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_GRANT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "USER_GRANT_STATE_ACTIVE" => Some(Self::Active),
            "USER_GRANT_STATE_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
