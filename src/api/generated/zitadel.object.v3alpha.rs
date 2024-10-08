// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    #[prost(enumeration="OwnerType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    #[prost(oneof="instance::Property", tags="1, 2")]
    pub property: ::core::option::Option<instance::Property>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Property {
        #[prost(string, tag="1")]
        Id(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        Domain(::prost::alloc::string::String),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OwnerType {
    Unspecified = 0,
    System = 1,
    Instance = 2,
    Org = 3,
}
impl OwnerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OwnerType::Unspecified => "OWNER_TYPE_UNSPECIFIED",
            OwnerType::System => "OWNER_TYPE_SYSTEM",
            OwnerType::Instance => "OWNER_TYPE_INSTANCE",
            OwnerType::Org => "OWNER_TYPE_ORG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OWNER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OWNER_TYPE_SYSTEM" => Some(Self::System),
            "OWNER_TYPE_INSTANCE" => Some(Self::Instance),
            "OWNER_TYPE_ORG" => Some(Self::Org),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
