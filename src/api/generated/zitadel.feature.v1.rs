// @generated
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstanceFeature {
    Unspecified = 0,
    LoginDefaultOrg = 1,
}
impl InstanceFeature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstanceFeature::Unspecified => "INSTANCE_FEATURE_UNSPECIFIED",
            InstanceFeature::LoginDefaultOrg => "INSTANCE_FEATURE_LOGIN_DEFAULT_ORG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTANCE_FEATURE_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTANCE_FEATURE_LOGIN_DEFAULT_ORG" => Some(Self::LoginDefaultOrg),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
