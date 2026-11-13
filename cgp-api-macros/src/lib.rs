//! `#[entity_api]` — generate a `cgp-api` context from an entity and a policy.
//!
//! The attribute generates a context: a `{Name}Base` preset (entity, codecs,
//! identity), a `{Name}Preset` that composes handwritten ORM stacks and error
//! policies with that base, the context struct, `new`, `delegate_components!`,
//! and compile-time checks. It does **not** generate error tables, stacks,
//! codecs, or name providers. Error policies (`ApiErrorPolicy`,
//! `RestErrorPolicy`, `GrpcErrorPolicy`) and application stacks stay
//! handwritten.
//!
//! Re-exported from [`cgp-api`](https://docs.rs/cgp-api) behind the `macros`
//! feature as [`cgp_api::entity_api`](https://docs.rs/cgp-api/latest/cgp_api/attr.entity_api.html).
//! Callers depend only on `cgp-api`.
//!
//! # Shorthand (`policy` + `layer`)
//!
//! `policy = api | rest | grpc` selects an error policy. `layer` (default
//! `resource`) selects the `cgp-orm` parents. The generated preset is
//! those parents plus `{Name}Base`:
//!
//! | layer | parents |
//! | --- | --- |
//! | `lookup` | `RepositoryLookupStack` + `{Api,Rest,Grpc}ErrorPolicy` |
//! | `crud` | `RepositoryCrudStack` + policy |
//! | `resource` | `RepositoryCrudStack` + `ResourceServiceStack` + policy |
//!
//! `policy` may also be a custom error-policy path. Then `error` is required.
//!
//! `cfg` is never implied. Set `cfg = "rest"` to gate one generated context
//! on a feature of the *consumer* crate. `policy = api` still needs the
//! `thiserror` feature of `cgp-api` (`rest` / `grpc` likewise). That is a
//! dependency feature, not a `#[cfg]` on the generated struct: application
//! crates usually do not define features with those names.
//!
//! # Explicit `stack`
//!
//! `stack` is a single path or a `+`-sum of paths, for example
//! `stack = RepositoryCrudStack + ResourceServiceStack + RfErrorPolicy`.
//! `error` is required. No `cfg` is emitted unless `cfg` is set.
//!
//! # Example
//!
//! ```ignore
//! #[cgp_api::entity_api(
//!     policy = api,
//!     entity = post::Entity,
//!     create = CreatePost,
//!     update = UpdatePost,
//!     output = PostOutput,
//!     codec = PostCodec,
//!     unique = PostUniqueColumns,
//!     name = PostEntityName,
//!     also(PostRestApi: rest, PostGrpcApi: grpc),
//! )]
//! pub struct PostApi {
//!     pub db_conn: sea_orm::DatabaseConnection,
//! }
//! ```
//!
//! `name = Tag` wires
//! `EntityNameGetterComponent: ::cgp_api::providers::ConstEntityName<Tag>`,
//! matching [`cgp_api::entity_name!`](https://docs.rs/cgp-api/latest/cgp_api/macro.entity_name.html).
//!
//! `also(PostRestApi: rest)` keeps the same layer and switches the policy.
//! `layer` overrides that (`also(PostLookup: api, layer = lookup)`).
//! Codecs live on the shared base, so `layer = resource` on an `also`
//! entry requires the primary context to already set `create`, `update`,
//! `output`, and a codec.
//! An `also` entry may also use an explicit stack (with `error`) or
//! `policy = CustomPolicy` (with `error`).
//!
//! The struct must have a `db_conn` field (`cgp-orm` derives `HasDbConn`
//! from that name). The expansion also derives `HasField` and `Clone`.
//! A `#[derive]` written on the struct (below `#[entity_api]`) may repeat
//! those; they are not derived twice.

#![allow(missing_docs)]

mod check;
mod expand;
mod parse;

use proc_macro::TokenStream;
use syn::{Error, ItemStruct, parse_macro_input};

use crate::expand::expand;
use crate::parse::EntityApiArgs;

/// Generate a context from an entity, a policy or stack, and identity.
///
/// See the [crate-level docs](crate) for the full attribute grammar and
/// the expansion shape.
#[proc_macro_attribute]
pub fn entity_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EntityApiArgs);
    let item = parse_macro_input!(item as ItemStruct);
    expand(args, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
