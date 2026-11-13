//! Compile-pass: the README shape, `policy = api` plus keyword `also`.
#![cfg(all(feature = "thiserror", feature = "rest", feature = "grpc"))]

use cgp_api_macros_tests::codec::PostCodec;
use cgp_api_macros_tests::dto::{CreatePost, PostOutput, UpdatePost};
use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    policy = api,
    layer = resource,
    entity = entity::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    codec = PostCodec,
    unique = PostUniqueColumns,
    name = PostEntityName,
    also(PostRestApi: rest, PostGrpcApi: grpc),
)]
pub struct PostApi {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn also_transports_compile() {
    let _ = core::mem::size_of::<PostApi>();
    let _ = core::mem::size_of::<PostRestApi>();
    let _ = core::mem::size_of::<PostGrpcApi>();
}
