//! Example application on top of the generic [`cgp_api`] / [`cgp_orm`]
//! layers.
//!
//! Demonstrates:
//!
//! - **Distinct Create / Update / Output DTOs** — `CreateUser` / `UpdateUser`
//!   / `UserOutput`, and the same for posts.
//! - **A UUID primary key** (`UserApp`) and an **i32** key (`PostApp`).
//! - **A composite primary key** (`FavoriteApp`, CRUD stack, no DTOs) used as
//!   the identity for `get_or_create`.
//! - Composition of `cgp-orm` stacks with [`cgp_api::ApiErrorPolicy`]
//!   (`RepositoryCrudStack + ResourceServiceStack + ApiErrorPolicy`,
//!   or `RepositoryCrudStack + ApiErrorPolicy` for favorites), with
//!   the application only filling in entity, codecs, and identity.
//!   The context error is [`cgp_api::ApiError`].
//! - **One DTO envelope** (`TagApp`, [`TagDto`]) for create, update, and
//!   output. [`TagDto`] implements [`cgp_api::dto_crud::HasId`] once and the
//!   context gets `create_by_dto` / `update_dto_by_id` / `list_dto` from the
//!   `cgp-api` blanket impls, with no per-context impl.

pub mod codec;
pub mod contexts;
pub mod dto;
pub mod entity;
pub mod identity;

pub use contexts::{FavoriteApp, PostApp, TagApp, UserApp};
pub use dto::{CreatePost, CreateUser, PostOutput, TagDto, UpdatePost, UpdateUser, UserOutput};
