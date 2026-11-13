use cgp_api_macros::entity_api;

#[entity_api(
    stack = SomeResourceStack,
    entity = post::Entity,
    unique = PostUniqueColumns,
)]
pub struct PostApi {
    pub db_conn: u32,
}

fn main() {}
