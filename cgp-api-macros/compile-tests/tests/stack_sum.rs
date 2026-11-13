//! Compile-pass: `stack` as a `+`-sum of ORM parents and an error policy.
use cgp_api_macros_tests::codec::PostCodec;
use cgp_api_macros_tests::dto::{CreatePost, PostOutput, UpdatePost};
use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    stack = ::cgp_orm::presets::RepositoryCrudStack
        + ::cgp_orm::presets::ResourceServiceStack
        + ::cgp_api::presets::ApiErrorPolicy,
    error = ::cgp_api::error::ApiError,
    entity = entity::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    codec = PostCodec,
    unique = PostUniqueColumns,
    name = PostEntityName,
)]
pub struct PostSum {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn stack_sum_compiles() {
    let _ = core::mem::size_of::<PostSum>();
}
