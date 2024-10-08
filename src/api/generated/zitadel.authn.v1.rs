// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(enumeration="KeyType", tag="3")]
    pub r#type: i32,
    #[prost(message, optional, tag="4")]
    pub expiration_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    Unspecified = 0,
    Json = 1,
}
impl KeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyType::Unspecified => "KEY_TYPE_UNSPECIFIED",
            KeyType::Json => "KEY_TYPE_JSON",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KEY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "KEY_TYPE_JSON" => Some(Self::Json),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
