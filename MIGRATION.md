# Migrating to `cgp-orm` + `cgp-api`

Crate names: the generic persistence layer is `cgp-orm`; the generic
application layer is `cgp-api`. Domain entities, codecs, and context
types belong in application crates (see `examples/blog`).

## What lives in each crate

| Concern | Crate |
| --- | --- |
| Repository, identity, transactions, pagination, resource service | `cgp-orm` |
| Error maps, error-policy presets, `HasEntityName`, `ResourceResponseList` | `cgp-api` |
| Entities, codecs, identity policies, App contexts | application crate |

The `cgp-api` library itself is only: error maps, error-policy
presets, `HasEntityName`, codec trait re-exports, and
`ResourceResponseList`.

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
| `find_or_insert` | `insert`, `find_by_identity`, `get_or_create`, `upsert` |
| Create-via-`find_or_insert` | `CanCreateResource::create` (delegates to `get_or_create`, returns `(Output, InsertOutcome)`) |
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
- `get_or_create` — load by identity first; on a miss, insert; on a
  concurrent unique-constraint violation, roll the transaction back and
  refetch the winner. The active model is not cloned.
  Requires a real unique index matching the identity. Detection is a **policy**
  (`StandardUniqueConflict` by default).
- `upsert` — find by identity; insert or merge-update; handle a
  concurrent unique conflict.

## `cgp-orm` unique columns and primary keys

- `HasUniqueColumns::unique_columns` returns `&'static [ColumnOf<Self>]`.
  Change providers from `vec![Column::Email]` to `&[Column::Email]`
  (return type `&'static [entity::Column]`). The provider trait
  `UniqueColumnsGetter` and key `UniqueColumnsGetterComponent` are
  unchanged, so context wiring stays the same.
- `PrimaryKeyReadError::Arity` and `util::value_tuple_*` are gone.
  `ErrUnsupportedKeyArity` still exists (it reports a key *write* with
  the wrong arity), so the `cgp-api` error maps keep it.

## Response helpers (breaking)

- `ResourceResponse` and `response_from` are removed. A single-item
  response is the payload: return `Output` / `Option<Output>` and let
  failures travel on `Err`. Replace `response_from(x)` with `x`.
- `ResourceResponseList` is `data: Vec<T>` plus `meta: Option<M>`.
  `errors` is removed (use the `Result`). Items are payloads, not
  wrappers: `list.data[0].data.unwrap()` becomes `list.data[0]`.
- `response_from_page` moves `Page::items` in and puts `PageMeta` on
  the list only; per-row `meta` is gone.
- `response_list_from` takes an owned `Vec<T>` (was
  `impl IntoIterator`). Call `.collect::<Vec<_>>()` or `Vec::from` on
  other iterables, or use `ResourceResponseList::from(vec)`.
- `HasEntityName::entity_name` returns `&'static str` and is a
  `#[cgp_component(EntityNameGetter)]` instead of a `#[cgp_getter]`.
  The provider trait and `EntityNameGetterComponent` keep their names;
  `ConstEntityName<Tag>` wiring is unchanged. A context that derived
  the name from a struct field must wire `ConstEntityName` (or a
  provider of its own that returns a `'static` literal).

## DTO-envelope CRUD moves into `cgp-api`

`CanCreateByDto`, `CanUpdateDtoById`, and `CanListDto` used to be
defined per application with one impl per context (for example
`impl CanCreateByDto for FolderApp` in `rf-cgp`). They now live in
`cgp_api::dto_crud`, generic over the DTO, with one blanket impl each.
The method names and return shapes are unchanged
(`(Dto, InsertOutcome)`, `Option<Dto>`, `Vec<Dto>`); the error is the
context's `HasErrorType::Error`.

In the application crate (`rf-cgp`):

1. Delete the local trait definitions and **every** per-context impl
   (`impl CanCreateByDto for FolderApp`, `… for DeviceApp`, …), and drop
   their re-exports from the local prelude.
2. Implement `HasId` once for the envelope:

   ```rust,ignore
   use cgp_api::dto_crud::HasId;

   impl HasId for rf_proto::DtoData {
       // `DtoData.id` is `optional int32`; `Into<PrimaryKeyOf<App>>`
       // must hold for every context's key (`i32 -> i32` does).
       type Id = i32;

       fn id(&self) -> Option<i32> {
           self.id
       }
   }
   ```

   **Where it goes (orphan rule).** `HasId` is defined in `cgp-api`, so
   the impl must live in a crate that owns `HasId` or `DtoData`. If
   `rf_proto` is its own crate, put the impl **in `rf-proto`** — ideally
   behind an optional feature (`cgp = ["dep:cgp-api"]`, with
   `cgp-api = { version = "0.1", default-features = false }`) that
   `rf-cgp` enables. It cannot go in `rf-cgp` unless `DtoData` is a type
   `rf-cgp` defines. `proto-dto` is not involved and still does not
   depend on cgp.
3. An update without an id now raises `cgp-orm`'s `ErrPrimaryKeyUnset`
   through the context's error map instead of `ErrDtoIdRequired`. The
   RF error table already registers a raiser for it (the `cgp-orm`
   update requires one); through `ApiErrors` it is
   `ApiError::PrimaryKeyUnset { column: "Id" }`.
   `ErrDtoIdRequired` stays an RF type; keep it if other RF code raises
   it, otherwise delete it.
4. Callers (`proto_dto::frb_crud_service!` ≥ the release that pastes
   `cgp_api::prelude::…`) import the traits from `cgp_api::prelude`. The
   crate that expands the macro must be able to name `cgp_api`: add
   `cgp-api` as a direct dependency (same version as `rf-cgp` uses), or
   `pub use cgp_api;` from `rf-cgp` and `use rf_cgp::cgp_api;` at the
   expansion site.

The blanket impls need, per context: `CreateInput = UpdateInput = Output
= DtoData`, the `ResourceServiceStack` (or any provider of the three
service components), and the `ErrPrimaryKeyUnset` / `ErrMissingPrimaryKey`
raisers — which `RepositoryCrudStack`'s update already requires.

`list_dto` now sends `PageRequest::without_total()`, so the unused
`COUNT(*)` is no longer run.

## Other notes

- **`cgp-fork`** (published) is used. The unpublished git/path
  fork was not reachable; official `cgp` alpha was not used.
- **SeaORM** `ValueTuple` is `One` / `Two` / `Three` / `Many`.
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

Publish `cgp-orm` to crates.io first, then this crate. Point crate docs
at `include_str!` or an absolute URL so docs.rs is not a dead pointer
to a repository-relative markdown file.

Adding a component key to a parent preset is a breaking change for
anyone who already assigned that key. A child that already filled
that key will not compile against the new parent without an
`override` (or a redesign). Treat new keys on `ApiErrorPolicy`,
`RestErrorPolicy`, and `GrpcErrorPolicy` as breaking. New keys on
the `cgp-orm` stacks are governed by that crate.
