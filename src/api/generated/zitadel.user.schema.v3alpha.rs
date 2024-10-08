// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSchema {
    /// ID is the read-only unique identifier of the schema.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Details provide some base information (such as the last change date) of the schema.
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::super::object::v2::Details>,
    /// Type is a human readable text describing the schema.
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    /// Current state of the schema.
    #[prost(enumeration="State", tag="4")]
    pub state: i32,
    /// Revision is a read only version of the schema, each update of the `schema`-field increases the revision.
    #[prost(uint32, tag="5")]
    pub revision: u32,
    /// JSON schema representation defining the user.
    #[prost(message, optional, tag="6")]
    pub schema: ::core::option::Option<::pbjson_types::Struct>,
    /// Defines the possible types of authenticators.
    /// This allows creating different user types like human/machine without usage of actions to validate possible authenticators.
    /// Removal of an authenticator does not remove the authenticator on a user.
    #[prost(enumeration="AuthenticatorType", repeated, packed="false", tag="7")]
    pub possible_authenticators: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    #[prost(oneof="search_query::Query", tags="1, 2, 3, 5, 6, 7")]
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
        /// Limit the result to a specific schema type.
        #[prost(message, tag="5")]
        TypeQuery(super::TypeQuery),
        /// Limit the result to a specific state of the schema.
        #[prost(message, tag="6")]
        StateQuery(super::StateQuery),
        /// Limit the result to a specific schema ID.
        #[prost(message, tag="7")]
        IdQuery(super::IdQuery),
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
pub struct IdQuery {
    /// Defines the ID of the user schema to query for.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the id query.
    #[prost(enumeration="super::super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeQuery {
    /// Defines which type to query for.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Defines which text comparison method used for the type query.
    #[prost(enumeration="super::super::super::object::v2::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateQuery {
    /// Defines the state to query for.
    #[prost(enumeration="State", tag="1")]
    pub state: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldName {
    Unspecified = 0,
    Type = 1,
    State = 2,
    Revision = 3,
    ChangeDate = 4,
}
impl FieldName {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FieldName::Unspecified => "FIELD_NAME_UNSPECIFIED",
            FieldName::Type => "FIELD_NAME_TYPE",
            FieldName::State => "FIELD_NAME_STATE",
            FieldName::Revision => "FIELD_NAME_REVISION",
            FieldName::ChangeDate => "FIELD_NAME_CHANGE_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIELD_NAME_UNSPECIFIED" => Some(Self::Unspecified),
            "FIELD_NAME_TYPE" => Some(Self::Type),
            "FIELD_NAME_STATE" => Some(Self::State),
            "FIELD_NAME_REVISION" => Some(Self::Revision),
            "FIELD_NAME_CHANGE_DATE" => Some(Self::ChangeDate),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::Unspecified => "STATE_UNSPECIFIED",
            State::Active => "STATE_ACTIVE",
            State::Inactive => "STATE_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "STATE_ACTIVE" => Some(Self::Active),
            "STATE_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthenticatorType {
    Unspecified = 0,
    Username = 1,
    Password = 2,
    Webauthn = 3,
    Totp = 4,
    OtpEmail = 5,
    OtpSms = 6,
    AuthenticationKey = 7,
    IdentityProvider = 8,
}
impl AuthenticatorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthenticatorType::Unspecified => "AUTHENTICATOR_TYPE_UNSPECIFIED",
            AuthenticatorType::Username => "AUTHENTICATOR_TYPE_USERNAME",
            AuthenticatorType::Password => "AUTHENTICATOR_TYPE_PASSWORD",
            AuthenticatorType::Webauthn => "AUTHENTICATOR_TYPE_WEBAUTHN",
            AuthenticatorType::Totp => "AUTHENTICATOR_TYPE_TOTP",
            AuthenticatorType::OtpEmail => "AUTHENTICATOR_TYPE_OTP_EMAIL",
            AuthenticatorType::OtpSms => "AUTHENTICATOR_TYPE_OTP_SMS",
            AuthenticatorType::AuthenticationKey => "AUTHENTICATOR_TYPE_AUTHENTICATION_KEY",
            AuthenticatorType::IdentityProvider => "AUTHENTICATOR_TYPE_IDENTITY_PROVIDER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTHENTICATOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTHENTICATOR_TYPE_USERNAME" => Some(Self::Username),
            "AUTHENTICATOR_TYPE_PASSWORD" => Some(Self::Password),
            "AUTHENTICATOR_TYPE_WEBAUTHN" => Some(Self::Webauthn),
            "AUTHENTICATOR_TYPE_TOTP" => Some(Self::Totp),
            "AUTHENTICATOR_TYPE_OTP_EMAIL" => Some(Self::OtpEmail),
            "AUTHENTICATOR_TYPE_OTP_SMS" => Some(Self::OtpSms),
            "AUTHENTICATOR_TYPE_AUTHENTICATION_KEY" => Some(Self::AuthenticationKey),
            "AUTHENTICATOR_TYPE_IDENTITY_PROVIDER" => Some(Self::IdentityProvider),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserSchemasRequest {
    /// list limitations and ordering.
    #[prost(message, optional, tag="1")]
    pub query: ::core::option::Option<super::super::super::object::v2::ListQuery>,
    /// the field the result is sorted.
    #[prost(enumeration="FieldName", tag="2")]
    pub sorting_column: i32,
    /// Define the criteria to query for.
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<SearchQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserSchemasResponse {
    /// Details provides information about the returned result including total amount found.
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::super::object::v2::ListDetails>,
    /// States by which field the results are sorted.
    #[prost(enumeration="FieldName", tag="2")]
    pub sorting_column: i32,
    /// The result contains the user schemas, which matched the queries.
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<UserSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserSchemaByIdRequest {
    /// unique identifier of the schema.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserSchemaByIdResponse {
    #[prost(message, optional, tag="1")]
    pub schema: ::core::option::Option<UserSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserSchemaRequest {
    /// Type is a human readable word describing the schema.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Defines the possible types of authenticators.
    #[prost(enumeration="AuthenticatorType", repeated, packed="false", tag="3")]
    pub possible_authenticators: ::prost::alloc::vec::Vec<i32>,
    #[prost(oneof="create_user_schema_request::DataType", tags="2")]
    pub data_type: ::core::option::Option<create_user_schema_request::DataType>,
}
/// Nested message and enum types in `CreateUserSchemaRequest`.
pub mod create_user_schema_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataType {
        /// JSON schema representation defining the user.
        #[prost(message, tag="2")]
        Schema(::pbjson_types::Struct),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserSchemaResponse {
    /// ID is the read-only unique identifier of the schema.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Details provide some base information (such as the last change date) of the schema.
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserSchemaRequest {
    /// unique identifier of the schema.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Type is a human readable word describing the schema.
    #[prost(string, optional, tag="2")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Defines the possible types of authenticators.
    ///
    /// Removal of an authenticator does not remove the authenticator on a user.
    #[prost(enumeration="AuthenticatorType", repeated, packed="false", tag="4")]
    pub possible_authenticators: ::prost::alloc::vec::Vec<i32>,
    #[prost(oneof="update_user_schema_request::DataType", tags="3")]
    pub data_type: ::core::option::Option<update_user_schema_request::DataType>,
}
/// Nested message and enum types in `UpdateUserSchemaRequest`.
pub mod update_user_schema_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataType {
        /// JSON schema representation defining the user.
        #[prost(message, tag="3")]
        Schema(::pbjson_types::Struct),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserSchemaResponse {
    /// Details provide some base information (such as the last change date) of the schema.
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserSchemaRequest {
    /// unique identifier of the schema.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserSchemaResponse {
    /// Details provide some base information (such as the last change date) of the schema.
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserSchemaRequest {
    /// unique identifier of the schema.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateUserSchemaResponse {
    /// Details provide some base information (such as the last change date) of the schema.
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::super::object::v2::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserSchemaRequest {
    /// unique identifier of the schema.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserSchemaResponse {
    /// Details provide some base information (such as the last change date) of the schema.
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::super::object::v2::Details>,
}
include!("zitadel.user.schema.v3alpha.tonic.rs");
// @@protoc_insertion_point(module)