# cgp-api

Generic application layer on top of [`cgp-orm`](https://crates.io/crates/cgp-orm): error-mapping
policies, error-policy presets composed with `cgp-orm` stacks, and transport-independent
response helpers.

**This crate has no domain model.** An application chooses entities,
codecs, identity policies, and context types, and composes the presets
here.

**Presets compose behavior, contexts choose policies, and macros do
not own architecture.**

Depends on [`cgp-orm`](https://crates.io/crates/cgp-orm) `0.1`. Publish
`cgp-orm` to crates.io first, then this crate. Full design:
[ARCHITECTURE.md](https://github.com/Just-Replicant/cgp-api/blob/main/ARCHITECTURE.md).
Adopting the split from an earlier mixed layout:
[MIGRATION.md](https://github.com/Just-Replicant/cgp-api/blob/main/MIGRATION.md).

## Features

| Feature | Default | What it enables |
| --- | --- | --- |
| `thiserror` | yes | `ApiError`, `ApiErrors`, `ApiErrorPolicy` |
| `rest` | no | `RestError`, Axum raisers, `RestErrorPolicy` |
| `grpc` | no | tonic `Status` raisers, `GrpcErrorPolicy` |
| `macros` | no | `#[entity_api]` (re-exported from `cgp-api-macros`) |

Response helpers (`ResourceResponse`, `response_from`,
`response_from_page`) are always available and generic over the
payload. They are not tied to protobuf.

## Quick start

```rust,ignore
use cgp_api::prelude::*;

cgp_preset! {
    PostAppPreset: RepositoryCrudStack + ResourceServiceStack + ApiErrorPolicy {
        EntityTypeProviderComponent: UseType<post::Entity>,
        CreateInputTypeProviderComponent: UseType<CreatePost>,
        UpdateInputTypeProviderComponent: UseType<UpdatePost>,
        OutputTypeProviderComponent: UseType<PostOutput>,
        [
            DecodeCreateInputComponent,
            ApplyUpdateInputComponent,
            EncodeOutputComponent,
        ]: PostCodec,
        UniqueColumnsGetterComponent: PostUniqueColumns,
        EntityNameGetterComponent: ConstEntityName<PostEntityName>,
    }
}

#[derive(HasField, Clone)]
pub struct PostApp {
    pub db_conn: sea_orm::DatabaseConnection,
}

delegate_components! {
    #[unchecked]
    PostApp { preset PostAppPreset }
}
```

`create` uses `insert` (unique conflicts are errors). `get_or_create`
uses `insert_or_get` and requires a real unique constraint matching
the identity policy. Input and output types are whatever the
application supplies — they do not have to be the same type.

With the `macros` feature, the same context can be generated. The
attribute produces a context only; error policies and application
stacks stay handwritten. `policy` + `layer` expand to the `cgp-orm`
parents plus the matching error policy (the nine stacked types are
gone):

```rust,ignore
#[cgp_api::entity_api(
    policy = api,                 // or rest, grpc, or a custom policy path
    layer = resource,             // lookup | crud | resource (default)
    entity = post::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    codec = PostCodec,
    unique = PostUniqueColumns,
    name = PostEntityName,        // wires ConstEntityName<PostEntityName>
    also(PostRestApi: rest, PostGrpcApi: grpc),
)]
pub struct PostApi {
    pub db_conn: sea_orm::DatabaseConnection,
}
```

`stack =` takes a single path or a `+`-sum, and then `error` is
required: `stack = RepositoryCrudStack + ResourceServiceStack +
RfErrorPolicy`. A path `policy` is combined with `layer`; `error` is
required. `cfg` is never implied — set `cfg = "rest"` only when the
consumer crate should compile that context behind a feature of its
own. `policy = api` / `rest` / `grpc` still needs the matching feature
on the `cgp-api` dependency. The struct must have a `db_conn` field.
`name = Tag` wires `ConstEntityName<Tag>`.

## Examples

This package is a workspace. The examples are separate crates that
depend on `cgp-api` and prove the generic layer:

- [`examples/blog`](examples/blog) — UUID-key users, distinct
  Create/Update/Output DTOs for posts, and a composite-key favorite.
  Examples are under the same AGPL-3.0-or-later license as this crate.

```sh
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

## What lives here, not in `cgp-orm`

- Error maps and error-policy presets (`ApiErrorPolicy` / `RestErrorPolicy`
  / `GrpcErrorPolicy`) — application *policy*, still domain-agnostic.
  Compose them with the `cgp-orm` stacks (`RepositoryCrudStack +
  ResourceServiceStack + ApiErrorPolicy`, or
  `RepositoryLookupStack + ApiErrorPolicy`).
- `HasEntityName` and `ConstEntityName` — the name is chosen by the
  consumer. `entity_name!` declares a tag; wire `ConstEntityName<Tag>`.
- `#[entity_api]` (feature `macros`) — generates a context. Error
  policies and application stacks stay handwritten.
- Transport-independent `ResourceResponse` helpers
- Re-exported codec traits (`CanDecodeCreateInput`, …)

Domain codecs, entities, and context structs belong in the
application (see the examples).

This crate does not authenticate, and a live `db_conn` behind a route
is full CRUD. There is no `CanAuthorize`.

## License

Licensed under the GNU Affero General Public License v3.0 or later
([AGPL-3.0-or-later](LICENSE)).
