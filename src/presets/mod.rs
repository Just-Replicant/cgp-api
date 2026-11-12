//! Generic error maps and stacked presets.
//!
//! Error maps are `delegate_components!` tables, not `cgp_preset!`s: preset
//! expansion drops the lifetime and type parameters on `@` keys. The
//! stacked presets (`ApiResourceStack`, …) forward `ErrorType`,
//! `ErrorRaiser`, and `ErrorWrapper` to one of these tables so a consumer
//! only has to choose an entity, codecs, and identity.

#![allow(missing_docs)] // generated delegate tables and presets
#![allow(clippy::crate_in_macro_def)]

#[cfg(feature = "thiserror")]
mod api;
#[cfg(feature = "grpc")]
mod grpc;
#[cfg(feature = "rest")]
mod rest;
mod stacks;

#[cfg(feature = "thiserror")]
pub use api::ApiErrors;
#[cfg(feature = "grpc")]
pub use grpc::GrpcErrors;
#[cfg(feature = "rest")]
pub use rest::RestErrors;
#[cfg(feature = "thiserror")]
pub use stacks::{ApiCrudStack, ApiLookupStack, ApiResourceStack};
#[cfg(feature = "grpc")]
pub use stacks::{GrpcCrudStack, GrpcLookupStack, GrpcResourceStack};
#[cfg(feature = "rest")]
pub use stacks::{RestCrudStack, RestLookupStack, RestResourceStack};
