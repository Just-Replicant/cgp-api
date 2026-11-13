//! Generic application layer on top of [`cgp_orm`].
//!
//! `cgp-api` is transport- and domain-agnostic. It has **no** domain
//! entities, **no** fixed DTO shape, and **no** domain route names.
//! An application crate (see `examples/blog`) chooses entities, codecs,
//! identity policies, and context types, and composes the presets in
//! this crate.
//!
//! What this crate provides:
//!
//! - **Error-mapping policies** — in-process [`ApiError`] (feature
//!   `thiserror`), REST (Axum, feature `rest`), and gRPC (tonic, feature
//!   `grpc`). Tables map `cgp-orm` source errors and `sea_orm::DbErr`.
//!   Applications embed [`ApiError`] and extend the map for their own sources.
//! - **Error-policy presets** — [`ApiErrorPolicy`], [`RestErrorPolicy`],
//!   [`GrpcErrorPolicy`] each forward the three error components to the
//!   matching table. A context composes them with a `cgp-orm` stack
//!   (`RepositoryCrudStack + ResourceServiceStack + ApiErrorPolicy`, or
//!   `RepositoryLookupStack + ApiErrorPolicy`) and fills in entity,
//!   codecs, and identity.
//! - **Transport-independent response helpers** — [`ResourceResponse`] and
//!   [`ResourceResponseList`], generic over the payload and optional
//!   metadata. Not tied to protobuf.
//! - **[`HasEntityName`]** — a generic routing/display name accessor.
//!   The name itself is chosen by the application.
//! - **`#[entity_api]`** (feature `macros`) — an attribute that generates
//!   a context from an entity, a policy or stack, and identity. Error
//!   policies and application stacks stay handwritten.
//!
//! Codec *traits* (`CanDecodeCreateInput`, `CanApplyUpdateInput`,
//! `CanEncodeOutput`) live in `cgp-orm` and are re-exported here. Input
//! and output types are associated types on the context, not a fixed DTO.
//!
//! The design principle, shared with `cgp-orm`: **presets compose
//! behavior, contexts choose policies, and macros do not own
//! architecture.**
//!
//! Full design: <https://github.com/Just-Replicant/cgp-api/blob/main/ARCHITECTURE.md>
//! (also included as [`architecture`]). Adopting the split from an earlier
//! mixed layout:
//! <https://github.com/Just-Replicant/cgp-api/blob/main/MIGRATION.md>
//! (also included as [`migration`]).
//!
//! [`HasEntityName`] is a compile-time `&'static str` literal. This crate
//! does not authenticate, and a live `db_conn` behind a route is full
//! CRUD; there is no `CanAuthorize`. Examples are under the same parent
//! AGPL-3.0-or-later license.

#![cfg_attr(docsrs, feature(doc_cfg))]

// So preset keys use `cgp_api::…` and resolve when a consumer
// crate replays a preset (`crate::` would resolve in the consumer).
extern crate self as cgp_api;

pub mod accessors;
pub mod codec;
pub mod error;
pub mod prelude;
pub mod presets;
pub mod providers;
pub mod transport;

pub use prelude::*;

/// Generate a context from an entity and a policy or stack.
///
/// Available behind the `macros` feature. See the
/// [`cgp_api_macros`](https://docs.rs/cgp-api-macros) crate docs for the
/// attribute grammar. The macro generates a context only; error policies
/// and application stacks stay handwritten.
#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[doc(inline)]
pub use cgp_api_macros::entity_api;

/// Architecture of `cgp-api` (the repository `ARCHITECTURE.md`).
#[doc = include_str!("../ARCHITECTURE.md")]
pub mod architecture {}

/// Migrating to `cgp-orm` + `cgp-api` (the repository `MIGRATION.md`).
#[doc = include_str!("../MIGRATION.md")]
pub mod migration {}
