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
//! - **Composable presets** — [`ApiResourceStack`], [`RestResourceStack`],
//!   [`GrpcResourceStack`] (and the lookup / CRUD-only variants) wire a
//!   `cgp-orm` stack to an error map so a consumer only has to fill in
//!   entity, codecs, and identity.
//! - **Transport-independent response helpers** — [`ResourceResponse`] and
//!   [`ResourceResponseList`], generic over the payload and optional
//!   metadata. Not tied to protobuf.
//! - **[`HasEntityName`]** — a generic routing/display name accessor.
//!   The name itself is chosen by the application.
//!
//! Codec *traits* (`CanDecodeCreateInput`, `CanApplyUpdateInput`,
//! `CanEncodeOutput`) live in `cgp-orm` and are re-exported here. Input
//! and output types are associated types on the context, not a fixed DTO.
//!
//! The design principle, shared with `cgp-orm`: **presets compose
//! behavior, contexts choose policies, and macros do not own
//! architecture.**
//!
//! Full design: `ARCHITECTURE.md`. Adopting the split from an earlier
//! mixed layout: `MIGRATION.md`.

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
