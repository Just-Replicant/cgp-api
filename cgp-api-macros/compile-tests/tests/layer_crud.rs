//! Compile-pass: `layer = crud` (no resource-service DTOs).
//! Also checks that an existing `Clone` / `HasField` derive is not repeated.
#![cfg(feature = "thiserror")]

use cgp_api_macros_tests::entity;
use cgp_api_macros_tests::identity::{PostEntityName, PostUniqueColumns};

#[cgp_api::entity_api(
    policy = api,
    layer = crud,
    entity = entity::Entity,
    unique = PostUniqueColumns,
    name = PostEntityName,
)]
#[derive(Debug, Clone, cgp_fork::prelude::HasField)]
pub struct PostCrud {
    pub db_conn: sea_orm::DatabaseConnection,
}

#[test]
fn layer_crud_compiles() {
    fn _assert_name<T: cgp_api::prelude::HasEntityName>(_: &T) {}
    let _ = core::mem::size_of::<PostCrud>();
}
