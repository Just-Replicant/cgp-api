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
/// In-process resource stack: [`cgp_orm::presets::RepositoryCrudStack`] plus
/// [`cgp_orm::presets::ResourceServiceStack`], with [`ApiErrors`].
///
/// Component keys are fully qualified (`cgp_orm::…`, `cgp_api::…`, and
/// `extern crate self as cgp_api` in this crate) so a consumer crate can
/// replay the preset. Short names in the [prelude](crate::prelude) are for
/// application code; they collide with `Error`, `Page`, and other common
/// types, and are not used as preset keys.
///
/// # Keys a consumer fills in
///
/// - [`cgp_orm::types::EntityTypeProviderComponent`]
/// - [`cgp_orm::components::service::CreateInputTypeProviderComponent`]
/// - [`cgp_orm::components::service::UpdateInputTypeProviderComponent`]
/// - [`cgp_orm::components::service::OutputTypeProviderComponent`]
/// - [`cgp_orm::components::DecodeCreateInputComponent`]
/// - [`cgp_orm::components::ApplyUpdateInputComponent`]
/// - [`cgp_orm::components::EncodeOutputComponent`]
/// - [`cgp_orm::accessors::UniqueColumnsGetterComponent`] (or a custom identity)
/// - optionally [`crate::accessors::EntityNameGetterComponent`], write hooks,
///   and a tighter [`cgp_orm::components::UniqueConflictDetectorComponent`]
///
/// Adding a component key to a parent preset is a breaking change for
/// anyone who already assigned that key — see the semver note in
/// [`crate::architecture`].
#[cfg(feature = "thiserror")]
#[doc(inline)]
pub use stacks::ApiResourceStack;
#[cfg(feature = "thiserror")]
pub use stacks::{ApiCrudStack, ApiLookupStack};
#[cfg(feature = "grpc")]
pub use stacks::{GrpcCrudStack, GrpcLookupStack, GrpcResourceStack};
#[cfg(feature = "rest")]
pub use stacks::{RestCrudStack, RestLookupStack, RestResourceStack};
