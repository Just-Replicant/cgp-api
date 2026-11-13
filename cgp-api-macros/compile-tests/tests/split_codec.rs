//! Compile-pass: `decode` / `apply` / `encode` instead of one `codec`.
#![cfg(feature = "thiserror")]

use cgp_api_macros_tests::codec::PostCodec;
use cgp_api_macros_tests::dto::{CreatePost, PostOutput, UpdatePost};
use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::PostUniqueColumns;

#[cgp_api::entity_api(
    policy = api,
    entity = entity::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    decode = PostCodec,
    apply = PostCodec,
    encode = PostCodec,
    unique = PostUniqueColumns,
)]
pub struct PostSplit {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn split_codec_compiles() {
    let _ = core::mem::size_of::<PostSplit>();
}
