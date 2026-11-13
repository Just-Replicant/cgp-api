//! Compile-pass: `policy = rest` does not require a direct `axum` dependency.
#![cfg(feature = "rest")]

use cgp_api_macros_tests::codec::PostCodec;
use cgp_api_macros_tests::dto::{CreatePost, PostOutput, UpdatePost};
use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    policy = rest,
    entity = entity::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    codec = PostCodec,
    unique = PostUniqueColumns,
    name = PostEntityName,
)]
pub struct PostRest {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn policy_rest_compiles() {
    fn _assert_error(_: &cgp_api::error::RestError) {}
    let _ = core::mem::size_of::<PostRest>();
}
