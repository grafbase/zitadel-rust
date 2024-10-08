// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Member {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub preferred_login_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub avatar_url: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::user::v1::Type", tag="10")]
    pub user_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    #[prost(oneof="search_query::Query", tags="1, 2, 3, 4")]
    pub query: ::core::option::Option<search_query::Query>,
}
/// Nested message and enum types in `SearchQuery`.
pub mod search_query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        FirstNameQuery(super::FirstNameQuery),
        #[prost(message, tag="2")]
        LastNameQuery(super::LastNameQuery),
        #[prost(message, tag="3")]
        EmailQuery(super::EmailQuery),
        #[prost(message, tag="4")]
        UserIdQuery(super::UserIdQuery),
    }
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
pub struct EmailQuery {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::v1::TextQueryMethod", tag="2")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdQuery {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
