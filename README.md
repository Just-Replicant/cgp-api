# cgp-api

Generic application layer on top of [`cgp-orm`](../cgp-orm): error-mapping
policies, composable resource-service presets, and transport-independent
response helpers.

**This crate has no domain model.** An application chooses entities,
codecs, identity policies, and context types, and composes the presets
here.

**Presets compose behavior, contexts choose policies, and macros do
not own architecture.**

Depends on `cgp-orm` via `path = "../cgp-orm"`. Full design:
[ARCHITECTURE.md](ARCHITECTURE.md). Adopting the split from an earlier
mixed layout: [MIGRATION.md](MIGRATION.md).

## Features

| Feature | Default | What it enables |
| --- | --- | --- |
| `thiserror` | yes | `ApiError`, `ApiErrors`, `ApiLookupStack`, `ApiCrudStack`, `ApiResourceStack` |
| `rest` | no | `RestError`, Axum raisers, `RestLookupStack`, `RestCrudStack`, `RestResourceStack` |
| `grpc` | no | tonic `Status` raisers, `GrpcLookupStack`, `GrpcCrudStack`, `GrpcResourceStack` |

Response helpers (`ResourceResponse`, `response_from`,
`response_from_page`) are always available and generic over the
payload. They are not tied to protobuf.

## Quick start

```rust,ignore
use cgp_api::prelude::*;

cgp_preset! {
    PostAppPreset: ApiResourceStack {
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
        EntityNameGetterComponent: PostEntityName,
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

## Examples

This package is a workspace. The examples are separate crates that
depend on `cgp-api` and prove the generic layer:

- [`examples/blog`](examples/blog) — UUID-key users, distinct
  Create/Update/Output DTOs for posts, and a composite-key favorite.

```sh
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

## What lives here, not in `cgp-orm`

- Error maps and stacked presets (`ApiError` / REST / gRPC) — application
  *policy*, still domain-agnostic
- `HasEntityName` and `ConstEntityName` — the name is chosen by the
  consumer
- Transport-independent `ResourceResponse` helpers
- Re-exported codec traits (`CanDecodeCreateInput`, …)

Domain codecs, entities, and context structs belong in the
application (see the examples).

## License

Licensed under the GNU Affero General Public License v3.0 or later
([AGPL-3.0-or-later](LICENSE)).
