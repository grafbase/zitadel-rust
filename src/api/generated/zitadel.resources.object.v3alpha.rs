// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Details {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// the timestamp of the first event applied to the object.
    #[prost(message, optional, tag="3")]
    pub created: ::core::option::Option<::pbjson_types::Timestamp>,
    /// the timestamp of the last event applied to the object.
    #[prost(message, optional, tag="4")]
    pub changed: ::core::option::Option<::pbjson_types::Timestamp>,
    /// the parent object representing the returned objects context.
    #[prost(message, optional, tag="5")]
    pub owner: ::core::option::Option<super::super::super::object::v3alpha::Owner>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    #[prost(uint64, tag="1")]
    pub offset: u64,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    /// If desc is true, the result is sorted by in descending order. Beware that if desc is true or the sorting column is not the creation date, the pagination results might be inconsistent.
    #[prost(bool, tag="3")]
    pub desc: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDetails {
    #[prost(uint64, tag="1")]
    pub applied_limit: u64,
    #[prost(uint64, tag="2")]
    pub total_result: u64,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TextFilterMethod {
    Equals = 0,
    EqualsIgnoreCase = 1,
    StartsWith = 2,
    StartsWithIgnoreCase = 3,
    Contains = 4,
}
impl TextFilterMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TextFilterMethod::Equals => "TEXT_FILTER_METHOD_EQUALS",
            TextFilterMethod::EqualsIgnoreCase => "TEXT_FILTER_METHOD_EQUALS_IGNORE_CASE",
            TextFilterMethod::StartsWith => "TEXT_FILTER_METHOD_STARTS_WITH",
            TextFilterMethod::StartsWithIgnoreCase => "TEXT_FILTER_METHOD_STARTS_WITH_IGNORE_CASE",
            TextFilterMethod::Contains => "TEXT_FILTER_METHOD_CONTAINS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEXT_FILTER_METHOD_EQUALS" => Some(Self::Equals),
            "TEXT_FILTER_METHOD_EQUALS_IGNORE_CASE" => Some(Self::EqualsIgnoreCase),
            "TEXT_FILTER_METHOD_STARTS_WITH" => Some(Self::StartsWith),
            "TEXT_FILTER_METHOD_STARTS_WITH_IGNORE_CASE" => Some(Self::StartsWithIgnoreCase),
            "TEXT_FILTER_METHOD_CONTAINS" => Some(Self::Contains),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
