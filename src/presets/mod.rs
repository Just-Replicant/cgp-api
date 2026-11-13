//! Generic error maps and error-policy presets.
//!
//! Error maps are `delegate_components!` tables, not `cgp_preset!`s: preset
//! expansion drops the lifetime and type parameters on `@` keys. Each table
//! has a matching policy preset ([`ApiErrorPolicy`], [`RestErrorPolicy`],
//! [`GrpcErrorPolicy`]) that forwards `ErrorType`, `ErrorRaiser`, and
//! `ErrorWrapper` to that table.
//!
//! A context composes a `cgp-orm` stack with the policy. The policy does
//! not wrap an ORM parent:
//!
//! ```ignore
//! cgp_preset! {
//!     PostAppPreset:
//!         cgp_orm::presets::RepositoryCrudStack
//!         + cgp_orm::presets::ResourceServiceStack
//!         + ApiErrorPolicy
//!     {
//!         EntityTypeProviderComponent: UseType<post::Entity>,
//!         CreateInputTypeProviderComponent: UseType<CreatePost>,
//!         UpdateInputTypeProviderComponent: UseType<UpdatePost>,
//!         OutputTypeProviderComponent: UseType<PostOutput>,
//!         // codecs, UniqueColumnsGetter, EntityNameGetter, hooks
//!     }
//! }
//! ```
//!
//! A lookup-only context uses
//! `cgp_orm::presets::RepositoryLookupStack + ApiErrorPolicy`. A CRUD
//! context without resource-service DTOs uses
//! `cgp_orm::presets::RepositoryCrudStack + ApiErrorPolicy`. REST and
//! gRPC contexts use the same parents with [`RestErrorPolicy`] or
//! [`GrpcErrorPolicy`].
//!
//! Component keys use fully qualified paths (`cgp_orm::…`, `cgp_api::…`,
//! and `extern crate self as cgp_api` in this crate) so a consumer crate
//! can replay the preset. Short names in the [prelude](crate::prelude)
//! are for application code; they collide with `Error`, `Page`, and other
//! common types, and are not used as preset keys.
//!
//! # Keys a consumer fills in
//!
//! - [`cgp_orm::types::EntityTypeProviderComponent`]
//! - [`cgp_orm::components::service::CreateInputTypeProviderComponent`]
//! - [`cgp_orm::components::service::UpdateInputTypeProviderComponent`]
//! - [`cgp_orm::components::service::OutputTypeProviderComponent`]
//! - [`cgp_orm::components::DecodeCreateInputComponent`]
//! - [`cgp_orm::components::ApplyUpdateInputComponent`]
//! - [`cgp_orm::components::EncodeOutputComponent`]
//! - [`cgp_orm::accessors::UniqueColumnsGetterComponent`] (or a custom identity)
//! - optionally [`crate::accessors::EntityNameGetterComponent`], write hooks,
//!   and a tighter [`cgp_orm::components::UniqueConflictDetectorComponent`]
//!
//! Adding a component key to a parent preset is a breaking change for
//! anyone who already assigned that key — see the semver note in
//! [`crate::architecture`].

#![allow(missing_docs)] // generated delegate tables and presets
#![allow(clippy::crate_in_macro_def)]

#[cfg(feature = "thiserror")]
mod api;
#[cfg(feature = "grpc")]
mod grpc;
#[cfg(feature = "rest")]
mod rest;

/// In-process error policy: forwards `ErrorType`, `ErrorRaiser`, and
/// `ErrorWrapper` to [`ApiErrors`].
///
/// Compose with a `cgp-orm` stack; this policy does not wrap one:
/// `RepositoryCrudStack + ResourceServiceStack + ApiErrorPolicy`, or
/// `RepositoryLookupStack + ApiErrorPolicy`.
#[cfg(feature = "thiserror")]
#[doc(inline)]
pub use api::ApiErrorPolicy;
#[cfg(feature = "thiserror")]
pub use api::ApiErrors;

/// gRPC error policy: forwards the three error components to [`GrpcErrors`].
///
/// Compose with a `cgp-orm` stack the same way as [`ApiErrorPolicy`].
#[cfg(feature = "grpc")]
#[doc(inline)]
pub use grpc::GrpcErrorPolicy;
#[cfg(feature = "grpc")]
pub use grpc::GrpcErrors;

/// REST error policy: forwards the three error components to [`RestErrors`].
///
/// Compose with a `cgp-orm` stack the same way as [`ApiErrorPolicy`].
#[cfg(feature = "rest")]
#[doc(inline)]
pub use rest::RestErrorPolicy;
#[cfg(feature = "rest")]
pub use rest::RestErrors;
