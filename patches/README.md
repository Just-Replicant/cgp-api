# Companion patches

## `cgp-orm-get-or-not-found.patch`

Issue 71 lives in `cgp-orm`, which is a separate crate. This patch is
against published `cgp-orm` **v0.1.2** (`8d1e4f65`, tag `v0.1.2`).

It adds the opt-in `CanGetResourceOrNotFound::get_or_not_found` next
to `CanGetResource`. The default `get` is unchanged and still returns
`Ok(None)` for a missing row. The helper raises `ErrEntityNotFound`,
which `cgp-api`'s REST map already turns into 404.

Apply it on a `cgp-orm` checkout at v0.1.2:

```sh
git apply path/to/cgp-orm-get-or-not-found.patch
```

The change is additive. `cgp-api` documents the helper and does not
depend on it (so `cgp-api` still builds against crates.io `cgp-orm`
0.1.2). Publish a `cgp-orm` release that includes this helper, then
depend on that version from `cgp-api` when you want the rustdoc links
to resolve.

Verified on the patched tree: `cargo fmt --all --check`,
`cargo clippy --all-targets -- -D warnings`, and `cargo test --all-targets`
all passed.
