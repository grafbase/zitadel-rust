// @generated
/// Deprecated: use Organization
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organisation {
    #[prost(oneof="organisation::Org", tags="1, 2")]
    pub org: ::core::option::Option<organisation::Org>,
}
/// Nested message and enum types in `Organisation`.
pub mod organisation {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Org {
        #[prost(string, tag="1")]
        OrgId(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        OrgDomain(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organization {
    #[prost(oneof="organization::Org", tags="1, 2")]
    pub org: ::core::option::Option<organization::Org>,
}
/// Nested message and enum types in `Organization`.
pub mod organization {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Org {
        #[prost(string, tag="1")]
        OrgId(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        OrgDomain(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestContext {
    #[prost(oneof="request_context::ResourceOwner", tags="1, 2")]
    pub resource_owner: ::core::option::Option<request_context::ResourceOwner>,
}
/// Nested message and enum types in `RequestContext`.
pub mod request_context {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceOwner {
        #[prost(string, tag="1")]
        OrgId(::prost::alloc::string::String),
        #[prost(bool, tag="2")]
        Instance(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuery {
    #[prost(uint64, tag="1")]
    pub offset: u64,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    #[prost(bool, tag="3")]
    pub asc: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Details {
    /// sequence represents the order of events. It's always counting
    ///
    /// on read: the sequence of the last event reduced by the projection
    ///
    /// on manipulation: the timestamp of the event(s) added by the manipulation
    #[prost(uint64, tag="1")]
    pub sequence: u64,
    /// change_date is the timestamp when the object was changed
    ///
    /// on read: the timestamp of the last event reduced by the projection
    ///
    /// on manipulation: the timestamp of the event(s) added by the manipulation
    #[prost(message, optional, tag="2")]
    pub change_date: ::core::option::Option<::pbjson_types::Timestamp>,
    /// resource_owner is the organization or instance_id an object belongs to
    #[prost(string, tag="3")]
    pub resource_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDetails {
    #[prost(uint64, tag="1")]
    pub total_result: u64,
    #[prost(uint64, tag="2")]
    pub processed_sequence: u64,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TextQueryMethod {
    Equals = 0,
    EqualsIgnoreCase = 1,
    StartsWith = 2,
    StartsWithIgnoreCase = 3,
    Contains = 4,
    ContainsIgnoreCase = 5,
    EndsWith = 6,
    EndsWithIgnoreCase = 7,
}
impl TextQueryMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TextQueryMethod::Equals => "TEXT_QUERY_METHOD_EQUALS",
            TextQueryMethod::EqualsIgnoreCase => "TEXT_QUERY_METHOD_EQUALS_IGNORE_CASE",
            TextQueryMethod::StartsWith => "TEXT_QUERY_METHOD_STARTS_WITH",
            TextQueryMethod::StartsWithIgnoreCase => "TEXT_QUERY_METHOD_STARTS_WITH_IGNORE_CASE",
            TextQueryMethod::Contains => "TEXT_QUERY_METHOD_CONTAINS",
            TextQueryMethod::ContainsIgnoreCase => "TEXT_QUERY_METHOD_CONTAINS_IGNORE_CASE",
            TextQueryMethod::EndsWith => "TEXT_QUERY_METHOD_ENDS_WITH",
            TextQueryMethod::EndsWithIgnoreCase => "TEXT_QUERY_METHOD_ENDS_WITH_IGNORE_CASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEXT_QUERY_METHOD_EQUALS" => Some(Self::Equals),
            "TEXT_QUERY_METHOD_EQUALS_IGNORE_CASE" => Some(Self::EqualsIgnoreCase),
            "TEXT_QUERY_METHOD_STARTS_WITH" => Some(Self::StartsWith),
            "TEXT_QUERY_METHOD_STARTS_WITH_IGNORE_CASE" => Some(Self::StartsWithIgnoreCase),
            "TEXT_QUERY_METHOD_CONTAINS" => Some(Self::Contains),
            "TEXT_QUERY_METHOD_CONTAINS_IGNORE_CASE" => Some(Self::ContainsIgnoreCase),
            "TEXT_QUERY_METHOD_ENDS_WITH" => Some(Self::EndsWith),
            "TEXT_QUERY_METHOD_ENDS_WITH_IGNORE_CASE" => Some(Self::EndsWithIgnoreCase),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ListQueryMethod {
    In = 0,
}
impl ListQueryMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ListQueryMethod::In => "LIST_QUERY_METHOD_IN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIST_QUERY_METHOD_IN" => Some(Self::In),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimestampQueryMethod {
    Equals = 0,
    Greater = 1,
    GreaterOrEquals = 2,
    Less = 3,
    LessOrEquals = 4,
}
impl TimestampQueryMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TimestampQueryMethod::Equals => "TIMESTAMP_QUERY_METHOD_EQUALS",
            TimestampQueryMethod::Greater => "TIMESTAMP_QUERY_METHOD_GREATER",
            TimestampQueryMethod::GreaterOrEquals => "TIMESTAMP_QUERY_METHOD_GREATER_OR_EQUALS",
            TimestampQueryMethod::Less => "TIMESTAMP_QUERY_METHOD_LESS",
            TimestampQueryMethod::LessOrEquals => "TIMESTAMP_QUERY_METHOD_LESS_OR_EQUALS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TIMESTAMP_QUERY_METHOD_EQUALS" => Some(Self::Equals),
            "TIMESTAMP_QUERY_METHOD_GREATER" => Some(Self::Greater),
            "TIMESTAMP_QUERY_METHOD_GREATER_OR_EQUALS" => Some(Self::GreaterOrEquals),
            "TIMESTAMP_QUERY_METHOD_LESS" => Some(Self::Less),
            "TIMESTAMP_QUERY_METHOD_LESS_OR_EQUALS" => Some(Self::LessOrEquals),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
