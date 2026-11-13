# cgp-api-macros

Proc-macro attribute `#[entity_api]` that generates a [`cgp-api`](https://crates.io/crates/cgp-api)
context. Callers depend only on `cgp-api` and enable the `macros` feature,
which re-exports this attribute.

The attribute generates a context. It does not generate error tables, stacks,
codecs, or name providers. Error policies and application stacks stay
handwritten.

See the [cgp-api documentation](https://docs.rs/cgp-api) for the attribute
grammar, shorthand (`policy` + `layer`), explicit `stack` paths, and
examples.

Licensed under AGPL-3.0-or-later, the same license as `cgp-api`.
