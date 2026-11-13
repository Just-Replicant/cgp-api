//! Compile-pass: `policy = grpc` names `cgp_api::error::Status`.
//! This crate does not depend on `tonic` directly.
#![cfg(feature = "grpc")]

use cgp_api_macros_tests::codec::PostCodec;
use cgp_api_macros_tests::dto::{CreatePost, PostOutput, UpdatePost};
use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    policy = grpc,
    entity = entity::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    codec = PostCodec,
    unique = PostUniqueColumns,
    name = PostEntityName,
)]
pub struct PostGrpc {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn policy_grpc_compiles() {
    fn _assert_error(_: &cgp_api::error::Status) {}
    let _ = core::mem::size_of::<PostGrpc>();
}
