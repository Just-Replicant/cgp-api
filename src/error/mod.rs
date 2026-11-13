//! Error types and raisers.
//!
//! Repository source errors live in [`cgp_orm::error`]. This module holds
//! the in-process [`ApiError`] (feature `thiserror`) and the REST and gRPC
//! raisers. Error *maps* live in [`crate::presets`]. Application-specific
//! sources belong in the application crate, which embeds [`ApiError`] and
//! extends the generic map — see the test and rustdoc on
//! [`crate::presets::ApiErrors`].
//!
//! At the REST and gRPC boundary, [`sea_orm::DbErr`] and unexpected
//! `String` / `&str` are raised with a fixed [`INTERNAL_ERROR_MESSAGE`].
//! The full original stays on in-process [`ApiError`] (callers there need
//! the SQL) and on the transport error's [`Error::source`](core::error::Error::source).
//! There is no logger and no debug-errors feature: this crate has no
//! `tracing` dependency, and an off-by-default leak switch gets left on.
//! The raiser keys are `open`, so an application that wants the raw text
//! on a private network overrides that one key.

#[cfg(feature = "thiserror")]
mod api;
#[cfg(feature = "grpc")]
mod grpc;
#[cfg(any(feature = "rest", feature = "grpc"))]
mod opaque;
#[cfg(feature = "rest")]
mod rest;

#[cfg(feature = "thiserror")]
pub use api::{
    ApiError, RaiseApiMessage, RaiseIdentityFieldUnset, RaisePrimaryKeyUnset, RaiseViaApiError,
    WrapApiError, WrapDetail,
};
#[cfg(feature = "grpc")]
pub use grpc::{
    RaiseGrpcInternal, RaiseGrpcInternalDisplay, RaiseGrpcInvalidArgument, RaiseGrpcNotFound,
    WrapGrpcStatus,
};
#[cfg(any(feature = "rest", feature = "grpc"))]
pub use opaque::{INTERNAL_ERROR_MESSAGE, UnexpectedMessage};
#[cfg(feature = "rest")]
pub use rest::{
    RaiseRestBadRequest, RaiseRestInternal, RaiseRestInternalDisplay, RaiseRestNotFound, RestError,
    WrapRestError,
};
/// gRPC context error (`tonic::Status`).
///
/// `#[entity_api(policy = grpc)]` names this re-export, so the generated
/// check does not need a direct `tonic` dependency.
#[cfg(feature = "grpc")]
pub use tonic::Status;
