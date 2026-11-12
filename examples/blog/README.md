# Blog example (`cgp-api-example-blog`)

A small consumer of `cgp-api` / `cgp-orm` that proves the generic
layer is not tied to a single DTO or identifier type.

| Context | Key | Identity | Service types |
| --- | --- | --- | --- |
| `UserApp` | `Uuid` | unique `email` | `CreateUser`, `UpdateUser`, `UserOutput` |
| `PostApp` | `i32` | unique `slug` | `CreatePost`, `UpdatePost`, `PostOutput` |
| `FavoriteApp` | composite `(user_id, post_id)` | the key itself | CRUD, no DTOs (`insert_or_get`, `find_by_id`) |

```sh
cargo test -p cgp-api-example-blog
```
