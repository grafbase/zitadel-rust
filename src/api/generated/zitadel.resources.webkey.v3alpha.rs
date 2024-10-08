// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebKeyRsaConfig {
    /// bit size of the RSA key
    #[prost(enumeration="web_key_rsa_config::RsaBits", tag="1")]
    pub bits: i32,
    /// signing algrithm used
    #[prost(enumeration="web_key_rsa_config::RsaHasher", tag="2")]
    pub hasher: i32,
}
/// Nested message and enum types in `WebKeyRSAConfig`.
pub mod web_key_rsa_config {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RsaBits {
        Unspecified = 0,
        RsaBits2048 = 1,
        RsaBits3072 = 2,
        RsaBits4096 = 3,
    }
    impl RsaBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RsaBits::Unspecified => "RSA_BITS_UNSPECIFIED",
                RsaBits::RsaBits2048 => "RSA_BITS_2048",
                RsaBits::RsaBits3072 => "RSA_BITS_3072",
                RsaBits::RsaBits4096 => "RSA_BITS_4096",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RSA_BITS_UNSPECIFIED" => Some(Self::Unspecified),
                "RSA_BITS_2048" => Some(Self::RsaBits2048),
                "RSA_BITS_3072" => Some(Self::RsaBits3072),
                "RSA_BITS_4096" => Some(Self::RsaBits4096),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RsaHasher {
        Unspecified = 0,
        Sha256 = 1,
        Sha384 = 2,
        Sha512 = 3,
    }
    impl RsaHasher {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RsaHasher::Unspecified => "RSA_HASHER_UNSPECIFIED",
                RsaHasher::Sha256 => "RSA_HASHER_SHA256",
                RsaHasher::Sha384 => "RSA_HASHER_SHA384",
                RsaHasher::Sha512 => "RSA_HASHER_SHA512",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RSA_HASHER_UNSPECIFIED" => Some(Self::Unspecified),
                "RSA_HASHER_SHA256" => Some(Self::Sha256),
                "RSA_HASHER_SHA384" => Some(Self::Sha384),
                "RSA_HASHER_SHA512" => Some(Self::Sha512),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebKeyEcdsaConfig {
    #[prost(enumeration="web_key_ecdsa_config::EcdsaCurve", tag="1")]
    pub curve: i32,
}
/// Nested message and enum types in `WebKeyECDSAConfig`.
pub mod web_key_ecdsa_config {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EcdsaCurve {
        Unspecified = 0,
        P256 = 1,
        P384 = 2,
        P512 = 3,
    }
    impl EcdsaCurve {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EcdsaCurve::Unspecified => "ECDSA_CURVE_UNSPECIFIED",
                EcdsaCurve::P256 => "ECDSA_CURVE_P256",
                EcdsaCurve::P384 => "ECDSA_CURVE_P384",
                EcdsaCurve::P512 => "ECDSA_CURVE_P512",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ECDSA_CURVE_UNSPECIFIED" => Some(Self::Unspecified),
                "ECDSA_CURVE_P256" => Some(Self::P256),
                "ECDSA_CURVE_P384" => Some(Self::P384),
                "ECDSA_CURVE_P512" => Some(Self::P512),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebKeyEd25519Config {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWebKey {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<WebKey>,
    #[prost(enumeration="WebKeyState", tag="3")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebKey {
    #[prost(oneof="web_key::Config", tags="6, 7, 8")]
    pub config: ::core::option::Option<web_key::Config>,
}
/// Nested message and enum types in `WebKey`.
pub mod web_key {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag="6")]
        Rsa(super::WebKeyRsaConfig),
        #[prost(message, tag="7")]
        Ecdsa(super::WebKeyEcdsaConfig),
        #[prost(message, tag="8")]
        Ed25519(super::WebKeyEd25519Config),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WebKeyState {
    StateUnspecified = 0,
    StateInitial = 1,
    StateActive = 2,
    StateInactive = 3,
    StateRemoved = 4,
}
impl WebKeyState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WebKeyState::StateUnspecified => "STATE_UNSPECIFIED",
            WebKeyState::StateInitial => "STATE_INITIAL",
            WebKeyState::StateActive => "STATE_ACTIVE",
            WebKeyState::StateInactive => "STATE_INACTIVE",
            WebKeyState::StateRemoved => "STATE_REMOVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNSPECIFIED" => Some(Self::StateUnspecified),
            "STATE_INITIAL" => Some(Self::StateInitial),
            "STATE_ACTIVE" => Some(Self::StateActive),
            "STATE_INACTIVE" => Some(Self::StateInactive),
            "STATE_REMOVED" => Some(Self::StateRemoved),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebKeyRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    #[prost(message, optional, tag="2")]
    pub key: ::core::option::Option<WebKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebKeyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateWebKeyRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateWebKeyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebKeyRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebKeyResponse {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::object::v3alpha::Details>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebKeysRequest {
    #[prost(message, optional, tag="1")]
    pub instance: ::core::option::Option<super::super::super::object::v3alpha::Instance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebKeysResponse {
    #[prost(message, repeated, tag="1")]
    pub web_keys: ::prost::alloc::vec::Vec<GetWebKey>,
}
include!("zitadel.resources.webkey.v3alpha.tonic.rs");
// @@protoc_insertion_point(module)