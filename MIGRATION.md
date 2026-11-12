# Migrating to `cgp-orm` + `cgp-api`

Crate names: the generic persistence layer is `cgp-orm`; the generic
application layer is `cgp-api`. Domain entities, codecs, and context
types belong in application crates (see `examples/blog`).

## What lives in each crate

| Concern | Crate |
| --- | --- |
| Repository, identity, transactions, pagination, resource service | `cgp-orm` |
| Error maps, stacked presets, `HasEntityName`, `ResourceResponse` | `cgp-api` |
| Entities, codecs, identity policies, App contexts | application crate |

The `cgp-api` library itself is only: error maps, stacked presets,
`HasEntityName`, codec trait re-exports, and `ResourceResponse`.

## Suggested adoption order

1. Depend on `cgp-orm` for generic PKs, repository, identity,
   transactions, pagination, resource service, and pluggable conflict
   detection.
2. Compose `cgp-api` stacks (`ApiResourceStack` / REST / gRPC) for
   error policy and transport helpers.
3. Implement entities, codecs, and contexts in your application crate.
   Contexts are written explicitly — there is no app-local macro.

## API mapping

| Earlier mixed layout | Rewrite |
| --- | --- |
| One crate with persistence + app policy + domain mixed | `cgp-orm` (persistence) + `cgp-api` (generic app layer) + application crates |
| Macro-generated entity apps | Explicit presets, structs, and `delegate_components!` |
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
- **No app-local macro.** Contexts are explicit.
- Workspace layout: `cgp-api` is the workspace root; run
  `cargo test --workspace --all-features` to include the examples.
