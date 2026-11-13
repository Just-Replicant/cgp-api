# Migrating to `cgp-orm` + `cgp-api`

Crate names: the generic persistence layer is `cgp-orm`; the generic
application layer is `cgp-api`. Domain entities, codecs, and context
types belong in application crates (see `examples/blog`).

## What lives in each crate

| Concern | Crate |
| --- | --- |
| Repository, identity, transactions, pagination, resource service | `cgp-orm` |
| Error maps, error-policy presets, `HasEntityName`, `ResourceResponse` | `cgp-api` |
| Entities, codecs, identity policies, App contexts | application crate |

The `cgp-api` library itself is only: error maps, error-policy
presets, `HasEntityName`, codec trait re-exports, and
`ResourceResponse`.

## Suggested adoption order

1. Depend on `cgp-orm` for generic PKs, repository, identity,
   transactions, pagination, resource service, and pluggable conflict
   detection.
2. Compose a `cgp-orm` stack with a `cgp-api` error policy
   (`RepositoryCrudStack + ResourceServiceStack + ApiErrorPolicy`,
   or the REST / gRPC policy) for error mapping and transport helpers.
3. Implement entities, codecs, and contexts in your application crate.
   Contexts may be written explicitly, or generated with
   `#[cgp_api::entity_api]` (feature `macros`). The attribute generates
   a context only; error policies and application stacks stay
   handwritten.

## API mapping

| Earlier mixed layout | Rewrite |
| --- | --- |
| One crate with persistence + app policy + domain mixed | `cgp-orm` (persistence) + `cgp-api` (generic app layer) + application crates |
| Macro-generated entity apps | Explicit presets, structs, and `delegate_components!`, or optional `#[entity_api]` which emits the same shape |
| `fn unique_columns` as the identity API | `CanBuildIdentityCondition`; `UniqueColumnsIdentity` |
| `find_or_insert` | `insert`, `find_by_identity`, `insert_or_get`, `upsert` |
| Create-via-`find_or_insert` | `CanCreateResource` (`insert`) and `CanGetOrCreateResource` (`insert_or_get`) |
| A single implicit DTO | Associated `CreateInput` / `UpdateInput` / `Output`; application codecs |
| `From<i32>` identifiers; `primary_key_column()` | `PrimaryKeyOf<C>` = complete `ValueType` |
| Hardcoded page size 5 | `PageRequest`, policy default 20 / max 100 |
| Transaction `?` on rollback | `conclude_transaction` preserves the original error |
| Error maps mixed with domain sources in one place | Generic maps in `cgp-api`; domain sources in the application |
| `HasEntityName` with route names baked into the library | Generic accessor; the name is provided by the application |
| Message-only unique-conflict detection | Pluggable `CanDetectUniqueConflict` in `cgp-orm` |

## Error policies, not stacked presets

Earlier `cgp-api` versions published nine stacked presets
(`ApiLookupStack`, `ApiCrudStack`, `ApiResourceStack`, and the REST
and gRPC copies). Those differed only by which `cgp-orm` parent they
wrapped. They are replaced by one policy preset per error table:

| Former preset | Compose instead |
| --- | --- |
| `ApiLookupStack` | `RepositoryLookupStack + ApiErrorPolicy` |
| `ApiCrudStack` | `RepositoryCrudStack + ApiErrorPolicy` |
| `ApiResourceStack` | `RepositoryCrudStack + ResourceServiceStack + ApiErrorPolicy` |
| `RestLookupStack` / `RestCrudStack` / `RestResourceStack` | the same `cgp-orm` parents, with `RestErrorPolicy` |
| `GrpcLookupStack` / `GrpcCrudStack` / `GrpcResourceStack` | the same `cgp-orm` parents, with `GrpcErrorPolicy` |

Wiring of the three error components is unchanged. An application
whose error type is `ApiError` can inherit `ApiErrorPolicy` directly.
An application error that adds sources (`RfError` and similar) needs
its own policy preset; it can still embed `ApiError` and raise
library sources with `RaiseViaApiError`.

`entity_name!` expands to a tag that implements `EntityNameValue`.
Wire `ConstEntityName<Tag>` as the `EntityNameGetterComponent`
provider. The macro no longer implements `EntityNameGetter` itself.
`#[entity_api]` with `name = Tag` wires `ConstEntityName<Tag>` the same way.

`#[entity_api]` shorthand (`policy` + `layer`) expands to the
`cgp-orm` parents plus the error policy, not to the former nine stack
types. `stack =` accepts a path or a `+`-sum, with `error` required.
`cfg` is opt-in; a keyword policy does not hide the context behind a
consumer feature. The generated struct needs a `db_conn` field.

## Write-operation semantics

- `insert` — unique conflicts are errors.
- `insert_or_get` — try insert; on a unique-constraint violation, roll
  the transaction back and `find_by_identity`. Requires a real unique
  index matching the identity. Detection is a **policy**
  (`StandardUniqueConflict` by default).
- `upsert` — find by identity; insert or merge-update; handle a
  concurrent unique conflict.

## Other notes

- **`cgp-fork` 1.1.0** (published) is used. The unpublished git/path
  fork was not reachable; official `cgp` 0.8.0-alpha was not used.
- **SeaORM 2** `ValueTuple` is `One` / `Two` / `Three` / `Many`.
  Structured unique-conflict detection via `DbErr::sql_err()` works
  when a sqlx backend feature is unified from the application; the
  library also keeps a tighter message fallback (no FK/CHECK matches)
  and pluggable backend-specific policies.
- **Optional `#[entity_api]`.** Contexts can still be written by hand.
  The `macros` feature generates the same shape (base, preset, struct,
  delegate, checks). Error policies and application stacks stay
  handwritten.
- Workspace layout: `cgp-api` is the workspace root; run
  `cargo test --workspace --all-features` to include the examples.

## Publishing and semver

Publish `cgp-orm` to crates.io first, then this crate (`cgp-orm = "0.1"`
in `Cargo.toml`). Point crate docs at `include_str!` or an absolute
URL so docs.rs is not a dead pointer to a repository-relative
markdown file.

Adding a component key to a parent preset is a breaking change for
anyone who already assigned that key. A child that already filled
that key will not compile against the new parent without an
`override` (or a redesign). Treat new keys on `ApiErrorPolicy`,
`RestErrorPolicy`, and `GrpcErrorPolicy` as breaking. New keys on
the `cgp-orm` stacks are governed by that crate.
