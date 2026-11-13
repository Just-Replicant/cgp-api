//! Explicit App contexts composed from `cgp-orm` stacks and `ApiErrorPolicy`.

use cgp_api::presets::ApiErrorPolicy;
use cgp_api::providers::ConstEntityName;
use cgp_fork::prelude::*;
use cgp_orm::presets::{RepositoryCrudStack, ResourceServiceStack};

use crate::codec::{PostCodec, UserCodec};
use crate::dto::{CreatePost, CreateUser, PostOutput, UpdatePost, UpdateUser, UserOutput};
use crate::entity::{favorite, post, user};
use crate::identity::{
    FavoriteEntityName, FavoriteUniqueColumns, PostEntityName, PostUniqueColumns, UserEntityName,
    UserUniqueColumns,
};

cgp_preset! {
    UserAppPreset: RepositoryCrudStack + ResourceServiceStack + ApiErrorPolicy {
        cgp_orm::types::EntityTypeProviderComponent:
            UseType<user::Entity>,
        cgp_orm::components::service::CreateInputTypeProviderComponent:
            UseType<CreateUser>,
        cgp_orm::components::service::UpdateInputTypeProviderComponent:
            UseType<UpdateUser>,
        cgp_orm::components::service::OutputTypeProviderComponent:
            UseType<UserOutput>,
        [
            cgp_orm::components::DecodeCreateInputComponent,
            cgp_orm::components::ApplyUpdateInputComponent,
            cgp_orm::components::EncodeOutputComponent,
        ]: UserCodec,
        cgp_orm::accessors::UniqueColumnsGetterComponent:
            UserUniqueColumns,
        cgp_api::accessors::EntityNameGetterComponent:
            ConstEntityName<UserEntityName>,
    }
}

/// Context for users (UUID primary key, [`cgp_api::ApiError`]).
#[derive(HasField, Clone)]
pub struct UserApp {
    /// SeaORM connection.
    pub db_conn: sea_orm::DatabaseConnection,
}

impl UserApp {
    /// Build a context around this connection.
    pub fn new(db_conn: sea_orm::DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

delegate_components! {
    #[unchecked]
    UserApp {
        preset UserAppPreset,
    }
}

cgp_preset! {
    PostAppPreset: RepositoryCrudStack + ResourceServiceStack + ApiErrorPolicy {
        cgp_orm::types::EntityTypeProviderComponent:
            UseType<post::Entity>,
        cgp_orm::components::service::CreateInputTypeProviderComponent:
            UseType<CreatePost>,
        cgp_orm::components::service::UpdateInputTypeProviderComponent:
            UseType<UpdatePost>,
        cgp_orm::components::service::OutputTypeProviderComponent:
            UseType<PostOutput>,
        [
            cgp_orm::components::DecodeCreateInputComponent,
            cgp_orm::components::ApplyUpdateInputComponent,
            cgp_orm::components::EncodeOutputComponent,
        ]: PostCodec,
        cgp_orm::accessors::UniqueColumnsGetterComponent:
            PostUniqueColumns,
        cgp_api::accessors::EntityNameGetterComponent:
            ConstEntityName<PostEntityName>,
    }
}

/// Context for posts (distinct Create / Update / Output DTOs, [`cgp_api::ApiError`]).
#[derive(HasField, Clone)]
pub struct PostApp {
    /// SeaORM connection.
    pub db_conn: sea_orm::DatabaseConnection,
}

impl PostApp {
    /// Build a context around this connection.
    pub fn new(db_conn: sea_orm::DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

delegate_components! {
    #[unchecked]
    PostApp {
        preset PostAppPreset,
    }
}

cgp_preset! {
    FavoriteAppPreset: RepositoryCrudStack + ApiErrorPolicy {
        cgp_orm::types::EntityTypeProviderComponent:
            UseType<favorite::Entity>,
        cgp_orm::accessors::UniqueColumnsGetterComponent:
            FavoriteUniqueColumns,
        cgp_api::accessors::EntityNameGetterComponent:
            ConstEntityName<FavoriteEntityName>,
    }
}

/// CRUD context for composite-key favorites (no resource-service DTOs, [`cgp_api::ApiError`]).
#[derive(HasField, Clone)]
pub struct FavoriteApp {
    /// SeaORM connection.
    pub db_conn: sea_orm::DatabaseConnection,
}

impl FavoriteApp {
    /// Build a context around this connection.
    pub fn new(db_conn: sea_orm::DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

delegate_components! {
    #[unchecked]
    FavoriteApp {
        preset FavoriteAppPreset,
    }
}
