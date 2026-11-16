# Blog example (`cgp-api-example-blog`)

A small consumer of `cgp-api` / `cgp-orm` that proves the generic
layer is not tied to a single DTO or identifier type.

| Context | Key | Identity | Service types |
| --- | --- | --- | --- |
| `UserApp` | `Uuid` | unique `email` | `CreateUser`, `UpdateUser`, `UserOutput` |
| `PostApp` | `i32` | unique `slug` | `CreatePost`, `UpdatePost`, `PostOutput` |
| `FavoriteApp` | composite `(user_id, post_id)` | the key itself | CRUD, no DTOs (`get_or_create`, `get_by_id`) |
| `TagApp` | `i32` | unique `name` | one `TagDto` envelope; `create_by_dto` / `update_dto_by_id` / `list_dto` come from the `cgp-api` blanket impls (`TagDto: HasId`) |

```sh
cargo test -p cgp-api-example-blog
```

Licensed under the same AGPL-3.0-or-later license as the parent `cgp-api` crate.
