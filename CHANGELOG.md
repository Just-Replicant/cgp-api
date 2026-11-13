# Changelog

## [Unreleased]

### Fixed

- *(error)* `RaiseRestInternal` and `RaiseGrpcInternal` no longer put
  `DbErr`'s Display (or unexpected `String` / `&str`) into the client
  body. The client sees a fixed `"internal error"`; the original is
  kept on in-process `ApiError` and on the transport error's `source()`.
- *(error)* `RestError` implements `Display`, `std::error::Error`
  (with `source()`), `PartialEq`, and `From<RestError> for (StatusCode, String)`
  using only the client message. gRPC `Status::message` stays generic.

### Added

- *(error)* A compiling extension test: one application error raised
  through a child table that embeds `ApiError` and uses `RaiseViaApiError`,
  without copying `ApiErrors`.
- *(docs)* How to return 404 via the opt-in `get_or_not_found` helper
  in `cgp-orm`. `CanGetResource::get` stays `Option`.
- *(examples)* A blog test that builds a `RestResourceStack` context
  on SQLite and checks a unique-conflict as `RestError`.
- *(ci)* One CI job runs `fmt --all --check`, `clippy --workspace
  --all-targets --all-features -- -D warnings`, and `test --workspace
  --all-targets --all-features`.
- *(macro)* `entity_name!` invokes `#[::cgp_fork::prelude::cgp_impl]`,
  so the call site does not need the prelude.

### Documentation

- Crate docs point at `include_str!` / absolute GitHub URLs for
  `ARCHITECTURE.md` and `MIGRATION.md`. README matches `cgp-orm = "0.1"`.
- Publish `cgp-orm` first. Semver note: adding a component key to a
  parent preset is a breaking change for anyone who already assigned
  that key. Point at `cgp-orm::transaction::run_in_transaction`.
- Examples are under the parent AGPL; `HasEntityName` is a compile-time
  literal; this crate does not authenticate (no `CanAuthorize`).
- Key list on `ApiResourceStack` rustdoc; fully qualified component
  keys are kept.


## [0.1.1](https://github.com/Just-Replicant/cgp-api/compare/v0.1.0...v0.1.1) - 2026-11-13

### Added

- *(transport)* make PageMeta.total optional

### Documentation

- *(examples)* rename insert_or_get to get_or_create
