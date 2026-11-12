//! Unique-column and entity-name providers.

use cgp_api::accessors::{EntityNameGetter, EntityNameGetterComponent};
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

macro_rules! entity_name {
    ($provider:ident, $name:literal) => {
        /// Entity-name provider.
        pub struct $provider;

        #[cgp_impl($provider)]
        impl EntityNameGetter {
            fn entity_name(&self) -> &str {
                $name
            }
        }
    };
}

entity_name!(UserEntityName, "users");
entity_name!(PostEntityName, "posts");
entity_name!(FavoriteEntityName, "favorites");
