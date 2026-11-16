//! Unique-column and entity-name providers.

use cgp_fork::prelude::*;
use cgp_orm::accessors::{UniqueColumnsGetter, UniqueColumnsGetterComponent};
use cgp_orm::types::HasEntityType;

use crate::entity::{favorite, post, tag, user};

/// Unique-column provider (email).
pub struct UserUniqueColumns;

#[cgp_impl(UserUniqueColumns)]
#[use_type(HasEntityType.{Entity = user::Entity})]
impl UniqueColumnsGetter {
    fn unique_columns(&self) -> &'static [user::Column] {
        &[user::Column::Email]
    }
}

/// Unique-column provider (slug).
pub struct PostUniqueColumns;

#[cgp_impl(PostUniqueColumns)]
#[use_type(HasEntityType.{Entity = post::Entity})]
impl UniqueColumnsGetter {
    fn unique_columns(&self) -> &'static [post::Column] {
        &[post::Column::Slug]
    }
}

/// Identity is the composite primary key itself.
pub struct FavoriteUniqueColumns;

#[cgp_impl(FavoriteUniqueColumns)]
#[use_type(HasEntityType.{Entity = favorite::Entity})]
impl UniqueColumnsGetter {
    fn unique_columns(&self) -> &'static [favorite::Column] {
        &[favorite::Column::UserId, favorite::Column::PostId]
    }
}

/// Unique-column provider (name).
pub struct TagUniqueColumns;

#[cgp_impl(TagUniqueColumns)]
#[use_type(HasEntityType.{Entity = tag::Entity})]
impl UniqueColumnsGetter {
    fn unique_columns(&self) -> &'static [tag::Column] {
        &[tag::Column::Name]
    }
}

// Tags only: wire `ConstEntityName<…>` at the context preset.
cgp_api::entity_name!(UserEntityName, "users");
cgp_api::entity_name!(PostEntityName, "posts");
cgp_api::entity_name!(FavoriteEntityName, "favorites");
cgp_api::entity_name!(TagEntityName, "tags");
