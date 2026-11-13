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
//! - Composition of [`cgp_api::ApiResourceStack`] and
//!   [`cgp_api::ApiCrudStack`], with the application only filling
//!   in entity, codecs, and identity. The context error is
//!   [`cgp_api::ApiError`].

pub mod codec;
pub mod contexts;
pub mod dto;
pub mod entity;
pub mod identity;

pub use contexts::{FavoriteApp, PostApp, UserApp};
pub use dto::{CreatePost, CreateUser, PostOutput, UpdatePost, UpdateUser, UserOutput};
