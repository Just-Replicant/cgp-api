# Changelog

## [Unreleased]

### Added

- *(dto_crud)* `CanCreateByDto<Dto>`, `CanUpdateDtoById<Dto>`, and
  `CanListDto<Dto>`, each with one blanket impl over the `cgp-orm`
  resource service (`CanCreateResource`, `CanUpdateResource`,
  `CanListResources`) for any context whose input and output are the
  same `Dto`. Exported from `cgp_api::prelude`. There is no per-context
  impl to write.
- *(dto_crud)* `HasId`: the one DTO-side trait, so the update blanket can
  read the id. A DTO without an id raises `cgp-orm`'s existing
  `ErrPrimaryKeyUnset` (first key column) — `ApiError::PrimaryKeyUnset`,
  REST 400, gRPC `InvalidArgument` — before any database call.
- *(examples)* `TagApp` / `TagDto` in `examples/blog`: one envelope for
  create, update, and output, served only through the blanket impls, with
  SQLite and REST tests.

### Changed

- *(dto_crud)* `list_dto` sends `PageRequest::without_total()`: the page
  metadata is dropped, so the `COUNT(*)` query is skipped.
- *(dto_crud)* `CanListDto`'s blanket impl also requires
  `HasCreateInputType<CreateInput = Dto>` and
  `HasUpdateInputType<UpdateInput = Dto>`, so `list_dto` appears only on
  envelope contexts, together with `create_by_dto` and
  `update_dto_by_id`. Contexts with distinct create / update types (for
  example `UserApp` / `PostApp`) no longer get a `list_dto` that drops
  the page; they use `CanListResources::list`. Compile-time tests lock
  this in.

### Removed

- `MIGRATION.md` and the `cgp_api::migration` docs module that included
  it. Links from the README and crate docs are gone.


## [0.1.2](https://github.com/Just-Replicant/cgp-api/compare/v0.1.1...v0.1.2) - 2026-11-13

### Added

- *(macros)* add entity_api and optional macros feature

### Changed

- *(examples)* compose cgp-orm stacks with error policies
- *(names)* make entity_name emit tags for ConstEntityName
- *(presets)* replace stacked presets with error-policy presets

### Documentation

- document error policies and entity_api
- document opaque errors, 404 helper, semver, and include_str docs

### Fixed

- *(macro)* invoke cgp_impl via absolute path in entity_name
- *(error)* keep DbErr opaque at REST and gRPC boundaries

### Breaking

- *(deps)* Build against current `cgp-orm` (compatible requirement
  range unchanged). Unique-column providers in the example and the
  macro compile tests return `&'static [Column]` instead of
  `Vec<Column>`.
- *(transport)* `ResourceResponse` and `response_from` are removed: a
  transport response is the payload, not a wrapper. Return `Output` /
  `Option<Output>` and let failures travel on `Err`.
- *(transport)* `ResourceResponseList` is now `data: Vec<T>` plus one
  `meta: Option<M>`; the `errors: Vec<String>` field (a second failure
  channel beside `Result`) is removed. `response_from_page` moves
  `Page::items` in instead of allocating a `Vec` of per-item wrappers,
  and puts `PageMeta` on the list only (no per-row copy).
  `response_list_from` takes an owned `Vec<T>`.
- *(accessors)* `HasEntityName::entity_name` returns `&'static str` and
  is declared with `#[cgp_component(EntityNameGetter)]` instead of
  `#[cgp_getter]`, matching `cgp-orm`'s `HasUniqueColumns`. The
  provider trait and component key keep their names.

- *(presets)* The nine `*LookupStack` / `*CrudStack` / `*ResourceStack`
  types are removed. Publish one policy preset per error table —
  `ApiErrorPolicy`, `RestErrorPolicy`, `GrpcErrorPolicy` — and compose
  it with the `cgp-orm` stacks: `RepositoryCrudStack +
  ResourceServiceStack + ApiErrorPolicy`, or
  `RepositoryLookupStack + ApiErrorPolicy`. The three error-component
  wirings are unchanged.
- *(macro)* `entity_name!` expands to a tag plus an `EntityNameValue`
  impl. It no longer implements `EntityNameGetter` itself. Wire
  `ConstEntityName<Tag>` as the `EntityNameGetterComponent` provider.

### Fixed

- *(error)* `RaiseRestInternal` and `RaiseGrpcInternal` no longer put
  `DbErr`'s Display (or unexpected `String` / `&str`) into the client
  body. The client sees a fixed `"internal error"`; the original is
  kept on in-process `ApiError` and on the transport error's `source()`.
- *(error)* `RestError` implements `Display`, `std::error::Error`
  (with `source()`), `PartialEq`, and `From<RestError> for (StatusCode, String)`
  using only the client message. gRPC `Status::message` stays generic.

### Added

- *(transport)* `ResourceResponseList::{new, with_meta, len, is_empty,
  into_parts}`, `Default`, `From<Vec<T>>`, `From<Page<T>>`, and
  `IntoIterator` (owned and borrowed). `PageMeta::of(&Page<T>)` and
  `From<&Page<T>>`.
- *(tests)* Assert that list helpers reuse the page's buffer, and that
  `ApiError` stays smaller than `sea_orm::DbErr`.

- *(macros)* Optional `macros` feature and the `cgp-api-macros` crate.
  `#[entity_api]` generates a context (base preset, stack preset, struct,
  `new`, `delegate_components!`, and checks) from an entity, a policy or
  stack, and identity. Error policies and application stacks stay
  handwritten. `cfg` is opt-in. `policy = grpc` names
  `cgp_api::error::Status`, a re-export of `tonic::Status`.
- *(error)* A compiling extension test: one application error raised
  through a child table that embeds `ApiError` and uses `RaiseViaApiError`,
  without copying `ApiErrors`.
- *(docs)* How to return 404 via the opt-in `get_or_not_found` helper
  in `cgp-orm`. `CanGetResource::get` stays `Option`.
- *(examples)* A blog test that builds a `RestErrorPolicy` context
  on SQLite and checks a unique-conflict as `RestError`.
- *(ci)* One CI job runs `fmt --all --check`, `clippy --workspace
  --all-targets --all-features -- -D warnings`, and `test --workspace
  --all-targets --all-features`.

### Documentation

- Crate docs point at `include_str!` / absolute GitHub URLs for
  `ARCHITECTURE.md` and `MIGRATION.md`. README matches the `cgp-orm`
  dependency.
- Publish `cgp-orm` first. Semver note: adding a component key to a
  parent preset is a breaking change for anyone who already assigned
  that key. Point at `cgp-orm::transaction::run_in_transaction`.
- Examples are under the parent AGPL; `HasEntityName` is a compile-time
  literal; this crate does not authenticate (no `CanAuthorize`).
- Key list on the presets module rustdoc; fully qualified component
  keys are kept.


## Previous - 2026-11-13

### Added

- *(transport)* make PageMeta.total optional

### Documentation

- *(examples)* rename insert_or_get to get_or_create
