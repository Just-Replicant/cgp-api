# Changelog

## [Unreleased]

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
  `ARCHITECTURE.md` and `MIGRATION.md`. README matches `cgp-orm = "0.1"`.
- Publish `cgp-orm` first. Semver note: adding a component key to a
  parent preset is a breaking change for anyone who already assigned
  that key. Point at `cgp-orm::transaction::run_in_transaction`.
- Examples are under the parent AGPL; `HasEntityName` is a compile-time
  literal; this crate does not authenticate (no `CanAuthorize`).
- Key list on the presets module rustdoc; fully qualified component
  keys are kept.


## [0.1.1](https://github.com/Just-Replicant/cgp-api/compare/v0.1.0...v0.1.1) - 2026-11-13

### Added

- *(transport)* make PageMeta.total optional

### Documentation

- *(examples)* rename insert_or_get to get_or_create
