//! Compile-pass: explicit `stack` with `also`, `raise`, `cfg`, and `extra`.
//! A handwritten app stack and custom error stand in for `RfResourceStack` / `RfError`.
#![cfg(feature = "thiserror")]

use cgp_api_macros_tests::app_error::{AppDetail, AppError, AppExtra, AppResourceStack};
use cgp_api_macros_tests::codec::PostCodec;
use cgp_api_macros_tests::dto::{CreatePost, PostOutput, UpdatePost};
use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    stack = AppResourceStack,
    error = AppError,
    cfg = "thiserror",
    entity = entity::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    codec = PostCodec,
    unique = PostUniqueColumns,
    name = PostEntityName,
    raise(AppExtra),
    wrap(AppDetail),
    extra(
        cgp_orm::components::EntityWritePreparerComponent:
            cgp_orm::providers::transaction::NoopPrepareEntityWrite,
    ),
    also(
        FileLookupApp: ::cgp_orm::presets::RepositoryLookupStack
            + ::cgp_api_macros_tests::app_error::AppErrorPolicy,
        error = AppError,
        cfg = "thiserror",
    ),
)]
pub struct FileApp {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn explicit_stack_compiles() {
    let _ = core::mem::size_of::<FileApp>();
    let _ = core::mem::size_of::<FileLookupApp>();
}
