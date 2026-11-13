//! Compile-pass: `layer = lookup` with `policy = api`.
#![cfg(feature = "thiserror")]

use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    policy = api,
    layer = lookup,
    entity = entity::Entity,
    unique = PostUniqueColumns,
    name = PostEntityName,
)]
pub struct PostLookup {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn layer_lookup_compiles() {
    let _ = core::mem::size_of::<PostLookup>();
}
