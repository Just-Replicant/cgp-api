# cgp-api architecture

`cgp-api` is a **generic** application / service layer on
[`cgp-orm`](https://crates.io/crates/cgp-orm). It owns error-mapping policies, stacked
presets, a routing-name accessor, and transport-independent response
helpers. It does **not** own a domain model: no entities, no fixed
DTO shape, and no transport-specific payload types.

The design principle, shared with `cgp-orm`: **presets compose
behavior, contexts choose policies, and macros do not own
architecture.**

# Layers

```mermaid
flowchart TB
    subgraph app ["application crate — examples/blog or yours"]
        ctx["App context: HasField + db_conn"]
        codecs["Codecs for the app's Create / Update / Output types"]
        ident["Identity: UniqueColumns or a custom condition"]
        names["HasEntityName — a name the app chooses"]
        domain["Entities, domain errors, domain hooks"]
    end

    subgraph api ["cgp-api (this crate)"]
        stacks["Api / Rest / Grpc Lookup, Crud, Resource stacks"]
        errors["ApiErrors, RestErrors, GrpcErrors"]
        resp["ResourceResponse, PageMeta"]
        accessor["HasEntityName, ConstEntityName"]
    end

    subgraph orm ["cgp-orm"]
        core["Repository and resource-service stacks"]
    end

    ctx --> stacks
    codecs --> stacks
    ident --> stacks
    names --> accessor
    stacks --> errors
    stacks --> core
    ctx --> resp
    domain --> ctx
```

# Stacked presets

Each transport has three presets, all forwarding `ErrorType`,
`ErrorRaiser`, and `ErrorWrapper` to the corresponding error table:

| Preset | `cgp-orm` parents | Error map |
| --- | --- | --- |
| `ApiLookupStack` | `RepositoryLookupStack` | `ApiErrors` (`ApiError`) |
| `ApiCrudStack` | `RepositoryCrudStack` | `ApiErrors` (`ApiError`) |
| `ApiResourceStack` | `RepositoryCrudStack + ResourceServiceStack` | `ApiErrors` (`ApiError`) |
| `RestLookupStack` / `RestCrudStack` / `RestResourceStack` | same | `RestErrors` (`rest`) |
| `GrpcLookupStack` / `GrpcCrudStack` / `GrpcResourceStack` | same | `GrpcErrors` (`grpc`) |

A consumer writes a child preset that fills in:

- `EntityTypeProviderComponent`
- For a resource stack: `CreateInput` / `UpdateInput` / `Output` and
  the three codecs
- `UniqueColumnsGetterComponent` (or a custom identity)
- optionally `EntityNameGetterComponent`, write hooks, a tighter
  `UniqueConflictDetectorComponent`

Component keys use `cgp_orm::…` and `cgp_api::…` paths (`extern crate
self as cgp_api`) so they resolve when a consumer crate replays a
preset.

# Error maps

The generic tables map `cgp-orm` source errors and `sea_orm::DbErr`
(and `String` / `&str`). `ApiError` is a `thiserror` enum.
Applications with extra sources define their own enum, embed
`ApiError` with `#[from]`, and raise library sources with
`RaiseViaApiError`. Raiser keys are `open`. Extension is that
open raiser plus the embedded `ApiError`, not an `anyhow` table
and not a copy of the `ApiErrors` source list. A compiling
example is the `child_table_raises_an_app_error` test.

At the REST and gRPC boundary, `RaiseRestInternal` and
`RaiseGrpcInternal` send a fixed `"internal error"` string for
`DbErr` and for unexpected `String` / `&str`. The full error stays
on in-process `ApiError` (callers there need the SQL) and on the
transport error's `source()`. This crate does not add a logger or
a debug-errors feature: there is no `tracing` dependency, and an
off-by-default leak switch gets left on. An application that wants
the raw text on a private network overrides that one open raiser
key.

Error maps are `delegate_components!` tables, not `cgp_preset!`s:
preset expansion drops lifetime and type parameters on `@` keys.

# Service types and codecs

`cgp-orm` exposes associated types `CreateInput`, `UpdateInput`, and
`Output`. This crate re-exports the codec traits and does not dictate
the types. `examples/blog` uses three distinct structs per resource;
an application may equally map all three to one shared type.

# Transport helpers

`ResourceResponse<T, M = ()>` and `ResourceResponseList<T, M>` are
generic wrappers with optional metadata. `response_from_page` copies
a `cgp-orm::Page` into `PageMeta`. Transports own HTTP / RPC mapping;
these helpers do not depend on Axum, tonic, or protobuf.

# Examples as proof

- `examples/blog` — UUID users, distinct post DTOs, composite-key
  favorites, composed from `ApiResourceStack` / `ApiCrudStack`.

# Tests

- `tests/helpers.rs` — response wrappers and REST error conversion.
- `src/error/*.rs` — in-process, REST, and gRPC raisers, including the
  opaque-internal-error split and a child error-table extension.
- `examples/blog/tests/sqlite.rs` — in-memory SQLite, run with
  `cargo test --workspace`. Includes a `RestResourceStack` conflict
  check as `RestError`. `cargo test -p cgp-api` stays a helper test;
  `--workspace` is the real suite.

# Transactions

Multi-resource work that must share one SeaORM transaction should use
[`cgp_orm::transaction::run_in_transaction`](https://docs.rs/cgp-orm/latest/cgp_orm/transaction/fn.run_in_transaction.html)
(documented in cgp-orm's
[ARCHITECTURE.md](https://github.com/Just-Replicant/cgp-orm/blob/main/ARCHITECTURE.md)
and that function's rustdoc). This crate does not add a helper that
builds several application contexts around one `DatabaseTransaction`;
that would start owning application wiring.

# How to return 404

`CanGetResource::get` returns `Result<Option<Output>, _>`. A missing
row is success. `ErrEntityNotFound` is mapped to 404 by `RestErrors`
(and to `NOT_FOUND` by `GrpcErrors`) but is never raised by the
default provider, so a handler that only matches `RestError` answers
200 with an empty body. Leave `get` as `Option`. An opt-in helper
next to the trait in `cgp-orm` (`get_or_not_found`) raises
`ErrEntityNotFound`. The REST map already has the raiser.

# Semver

Adding a component key to a parent preset is a breaking change for
anyone who already assigned that key. A child preset that filled
`SomeComponent: TheirProvider` will fail to compile (a conflict, or
an explicit `override` becomes required) if a parent starts wiring
that same key. New keys on `ApiResourceStack` and the other stacks
are breaking, including during 0.x.

# Security and names

This crate does not authenticate, and a live `db_conn` behind a route
is full CRUD. There is no `CanAuthorize`. `HasEntityName` is a
compile-time `&'static str` literal. Examples are under the same
parent AGPL-3.0-or-later license as this crate.
