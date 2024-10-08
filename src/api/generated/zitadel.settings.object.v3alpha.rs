// @generated
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
    /// resource_owner represents the context an object belongs to
    #[prost(message, optional, tag="3")]
    pub owner: ::core::option::Option<super::super::super::object::v3alpha::Owner>,
}
// @@protoc_insertion_point(module)
