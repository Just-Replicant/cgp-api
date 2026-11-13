//! Compile-pass: a resource context plus a lookup `also` with `layer`.
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
    also(PostLookup: api, layer = lookup),
)]
pub struct PostResource {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn also_lookup_layer_compiles() {
    let _ = core::mem::size_of::<PostResource>();
    let _ = core::mem::size_of::<PostLookup>();
}
