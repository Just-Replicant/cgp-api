//! Compile-pass: `policy` as a custom error-preset path, with `layer` and `error`.
#![cfg(feature = "thiserror")]

use cgp_api_macros_tests::app_error::{AppError, AppErrorPolicy};
use cgp_api_macros_tests::codec::PostCodec;
use cgp_api_macros_tests::dto::{CreatePost, PostOutput, UpdatePost};
use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    policy = AppErrorPolicy,
    layer = resource,
    error = AppError,
    entity = entity::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    codec = PostCodec,
    unique = PostUniqueColumns,
    name = PostEntityName,
    raise(cgp_api_macros_tests::app_error::AppExtra),
)]
pub struct PostCustom {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn path_policy_compiles() {
    let _ = core::mem::size_of::<PostCustom>();
}
