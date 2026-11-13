//! Compile-pass: `policy = api` (default `layer = resource`).
#![cfg(feature = "thiserror")]

use cgp_api_macros_tests::codec::PostCodec;
use cgp_api_macros_tests::dto::{CreatePost, PostOutput, UpdatePost};
use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    policy = api,
    entity = entity::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    codec = PostCodec,
    unique = PostUniqueColumns,
    name = PostEntityName,
)]
pub struct PostApi {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn policy_api_resource_compiles() {
    fn _assert_name<T: cgp_api::prelude::HasEntityName>(_: &T) {}
    let _ = core::mem::size_of::<PostApi>();
}
