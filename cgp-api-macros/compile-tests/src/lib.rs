//! Shared fixtures for `#[entity_api]` compile-pass tests.

extern crate self as cgp_api_macros_tests;

pub mod entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        #[sea_orm(unique)]
        pub slug: String,
        pub title: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod dto {
    #[derive(Clone, Debug)]
    pub struct CreatePost {
        pub slug: String,
        pub title: String,
    }

    #[derive(Clone, Debug)]
    pub struct UpdatePost {
        pub title: String,
    }

    #[derive(Clone, Debug)]
    pub struct PostOutput {
        pub id: i32,
        pub slug: String,
        pub title: String,
    }
}

pub mod codec {
    use cgp_fork::prelude::*;
    use cgp_orm::components::service::{
        ApplyUpdateInput, ApplyUpdateInputComponent, DecodeCreateInput, DecodeCreateInputComponent,
        EncodeOutput, EncodeOutputComponent, HasCreateInputType, HasOutputType, HasUpdateInputType,
    };
    use cgp_orm::types::HasEntityType;
    use sea_orm::{NotSet, Set};

    use crate::dto::{CreatePost, PostOutput, UpdatePost};
    use crate::entity;

    pub struct PostCodec;

    #[cgp_impl(PostCodec)]
    #[use_type(
        HasEntityType.{Entity = entity::Entity},
        HasErrorType.Error,
        HasCreateInputType.{CreateInput = CreatePost}
    )]
    impl DecodeCreateInput {
        fn decode_create_input(&self, input: CreatePost) -> Result<entity::ActiveModel, Error> {
            Ok(entity::ActiveModel {
                id: NotSet,
                slug: Set(input.slug),
                title: Set(input.title),
            })
        }
    }

    #[cgp_impl(PostCodec)]
    #[use_type(
        HasEntityType.{Entity = entity::Entity},
        HasErrorType.Error,
        HasUpdateInputType.{UpdateInput = UpdatePost}
    )]
    impl ApplyUpdateInput {
        fn apply_update_input(
            &self,
            id: i32,
            input: UpdatePost,
        ) -> Result<entity::ActiveModel, Error> {
            Ok(entity::ActiveModel {
                id: Set(id),
                slug: NotSet,
                title: Set(input.title),
            })
        }
    }

    #[cgp_impl(PostCodec)]
    #[use_type(
        HasEntityType.{Entity = entity::Entity},
        HasErrorType.Error,
        HasOutputType.{Output = PostOutput}
    )]
    impl EncodeOutput {
        fn encode_output(&self, model: entity::Model) -> Result<PostOutput, Error> {
            Ok(PostOutput {
                id: model.id,
                slug: model.slug,
                title: model.title,
            })
        }
    }
}

pub mod identity {
    use cgp_fork::prelude::*;
    use cgp_orm::accessors::{UniqueColumnsGetter, UniqueColumnsGetterComponent};
    use cgp_orm::types::HasEntityType;

    use crate::entity;

    pub struct PostUniqueColumns;

    #[cgp_impl(PostUniqueColumns)]
    #[use_type(HasEntityType.{Entity = entity::Entity})]
    impl UniqueColumnsGetter {
        fn unique_columns(&self) -> Vec<entity::Column> {
            vec![entity::Column::Slug]
        }
    }

    cgp_api::entity_name!(PostEntityName, "posts");
}

/// Custom error, table, and policy that stand in for an application `RfError`.
pub mod app_error {
    use cgp_fork::core::error::{
        ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
    };
    use cgp_fork::extra::error::{RaiseFrom, RaiseInfallible, ReturnError};
    use cgp_fork::prelude::*;
    use cgp_orm::error::{
        ErrConflictFetchFailed, ErrEntityNotFound, ErrIdentityEmpty, ErrIdentityFieldUnset,
        ErrMissingOrdering, ErrMissingPrimaryKey, ErrPrimaryKeyUnset, ErrUniqueConflict,
        ErrUnsupportedKeyArity,
    };
    use thiserror::Error;

    use cgp_api::error::{
        ApiError, RaiseApiMessage, RaiseIdentityFieldUnset, RaisePrimaryKeyUnset, RaiseViaApiError,
        WrapApiError, WrapDetail,
    };

    #[derive(Debug, Error, PartialEq, Eq)]
    #[error("app extra")]
    pub struct AppExtra;

    #[derive(Debug, Error)]
    pub enum AppError {
        #[error(transparent)]
        Api(#[from] ApiError),
        #[error(transparent)]
        Extra(#[from] AppExtra),
    }

    impl WrapDetail for AppError {
        fn wrap_detail(self, detail: impl core::fmt::Display) -> Self {
            match self {
                AppError::Api(api) => AppError::Api(api.wrap_detail(detail)),
                other => AppError::Api(ApiError::Message(other.to_string()).wrap_detail(detail)),
            }
        }
    }

    /// An extra wrap-detail type, used by `wrap(AppDetail)`.
    #[derive(Debug)]
    pub struct AppDetail;

    impl core::fmt::Display for AppDetail {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("app-detail")
        }
    }

    pub struct AppErrors;

    delegate_components! {
        #[unchecked]
        AppErrors {
            open ErrorRaiserComponent;

            ErrorTypeProviderComponent: UseType<AppError>,
            ErrorWrapperComponent: WrapApiError,
            @ErrorRaiserComponent.AppError: ReturnError,
            @ErrorRaiserComponent.core::convert::Infallible: RaiseInfallible,
            @ErrorRaiserComponent.[
                ::sea_orm::DbErr,
                ErrIdentityEmpty,
                ErrMissingPrimaryKey,
                ErrUnsupportedKeyArity,
                ErrMissingOrdering,
                ErrConflictFetchFailed,
                ErrEntityNotFound,
                ErrUniqueConflict,
            ]: RaiseViaApiError,
            @ErrorRaiserComponent.String: RaiseApiMessage,
            @ErrorRaiserComponent.<'a> &'a str: RaiseApiMessage,
            @ErrorRaiserComponent.<Column> ErrIdentityFieldUnset<Column>: RaiseIdentityFieldUnset,
            @ErrorRaiserComponent.<Column> ErrPrimaryKeyUnset<Column>: RaisePrimaryKeyUnset,
            @ErrorRaiserComponent.AppExtra: RaiseFrom,
        }
    }

    // Absolute `::cgp_api_macros_tests::…` paths so a consumer crate can
    // replay these presets (relative `AppErrors` / `AppErrorPolicy` would
    // resolve at the expansion site, not here).
    cgp_preset! {
        AppErrorPolicy {
            ::cgp_fork::core::error::ErrorTypeProviderComponent:
                <::cgp_api_macros_tests::app_error::AppErrors as DelegateComponent<
                    ::cgp_fork::core::error::ErrorTypeProviderComponent,
                >>::Delegate,
            ::cgp_fork::core::error::ErrorRaiserComponent:
                <::cgp_api_macros_tests::app_error::AppErrors as DelegateComponent<
                    ::cgp_fork::core::error::ErrorRaiserComponent,
                >>::Delegate,
            ::cgp_fork::core::error::ErrorWrapperComponent:
                <::cgp_api_macros_tests::app_error::AppErrors as DelegateComponent<
                    ::cgp_fork::core::error::ErrorWrapperComponent,
                >>::Delegate,
        }
    }

    cgp_preset! {
        AppResourceStack:
            ::cgp_orm::presets::RepositoryCrudStack
            + ::cgp_orm::presets::ResourceServiceStack
            + ::cgp_api_macros_tests::app_error::AppErrorPolicy
        {}
    }
}
