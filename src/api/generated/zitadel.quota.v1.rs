// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    /// The percentage relative to the quotas amount on which the call_url should be called.
    #[prost(uint32, tag="1")]
    pub percent: u32,
    /// If true, the call_url is called each time a factor of percentage is reached.
    #[prost(bool, tag="2")]
    pub repeat: bool,
    /// The URL, which is called with HTTP method POST and a JSON payload with the properties "unit", "id" (notification id), "callURL", "periodStart", "threshold" and "usage".
    #[prost(string, tag="3")]
    pub call_url: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Unit {
    Unimplemented = 0,
    /// The sum of all requests to the ZITADEL API with an authorization header,
    /// excluding the following exceptions
    /// - Calls to the System API
    /// - Calls that cause internal server errors
    /// - Failed authorizations
    /// - Requests after the quota already exceeded
    RequestsAllAuthenticated = 1,
    /// The sum of all actions run durations in seconds
    ActionsAllRunSeconds = 2,
}
impl Unit {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Unit::Unimplemented => "UNIT_UNIMPLEMENTED",
            Unit::RequestsAllAuthenticated => "UNIT_REQUESTS_ALL_AUTHENTICATED",
            Unit::ActionsAllRunSeconds => "UNIT_ACTIONS_ALL_RUN_SECONDS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNIT_UNIMPLEMENTED" => Some(Self::Unimplemented),
            "UNIT_REQUESTS_ALL_AUTHENTICATED" => Some(Self::RequestsAllAuthenticated),
            "UNIT_ACTIONS_ALL_RUN_SECONDS" => Some(Self::ActionsAllRunSeconds),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
