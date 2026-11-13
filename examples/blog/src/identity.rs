//! Unique-column and entity-name providers.

use cgp_fork::prelude::*;
use cgp_orm::accessors::{UniqueColumnsGetter, UniqueColumnsGetterComponent};
use cgp_orm::types::HasEntityType;

use crate::entity::{favorite, post, user};

/// Unique-column provider (email).
pub struct UserUniqueColumns;

#[cgp_impl(UserUniqueColumns)]
#[use_type(HasEntityType.{Entity = user::Entity})]
impl UniqueColumnsGetter {
    fn unique_columns(&self) -> Vec<user::Column> {
        vec![user::Column::Email]
    }
}

/// Unique-column provider (slug).
pub struct PostUniqueColumns;

#[cgp_impl(PostUniqueColumns)]
#[use_type(HasEntityType.{Entity = post::Entity})]
impl UniqueColumnsGetter {
    fn unique_columns(&self) -> Vec<post::Column> {
        vec![post::Column::Slug]
    }
}

/// Identity is the composite primary key itself.
pub struct FavoriteUniqueColumns;

#[cgp_impl(FavoriteUniqueColumns)]
#[use_type(HasEntityType.{Entity = favorite::Entity})]
impl UniqueColumnsGetter {
    fn unique_columns(&self) -> Vec<favorite::Column> {
        vec![favorite::Column::UserId, favorite::Column::PostId]
    }
}

// `entity_name!` expands with fully qualified CGP paths: no prelude or
// `EntityNameGetterComponent` import is required at this call site.
cgp_api::entity_name!(UserEntityName, "users");
cgp_api::entity_name!(PostEntityName, "posts");
cgp_api::entity_name!(FavoriteEntityName, "favorites");
