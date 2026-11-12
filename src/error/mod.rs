//! Error types and raisers.
//!
//! Repository source errors live in [`cgp_orm::error`]. This module holds
//! the in-process [`ApiError`] (feature `thiserror`) and the REST and gRPC
//! raisers. Error *maps* live in [`crate::presets`]. Application-specific
//! sources belong in the application crate, which embeds [`ApiError`] and
//! extends the generic map.

#[cfg(feature = "thiserror")]
mod api;
#[cfg(feature = "grpc")]
mod grpc;
#[cfg(feature = "rest")]
mod rest;

#[cfg(feature = "thiserror")]
pub use api::{
    ApiError, RaiseApiMessage, RaiseIdentityFieldUnset, RaisePrimaryKeyUnset, RaiseViaApiError,
    WrapApiError, WrapDetail,
};
#[cfg(feature = "grpc")]
pub use grpc::{RaiseGrpcInternal, RaiseGrpcInvalidArgument, RaiseGrpcNotFound, WrapGrpcStatus};
#[cfg(feature = "rest")]
pub use rest::{
    RaiseRestBadRequest, RaiseRestInternal, RaiseRestNotFound, RestError, WrapRestError,
};
