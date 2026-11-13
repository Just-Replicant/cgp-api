use cgp_api_macros::entity_api;

#[entity_api(
    policy = api,
    layer = lookup,
    entity = post::Entity,
    unique = PostUniqueColumns,
    also(PostApi: rest),
)]
pub struct PostApi {
    pub db_conn: u32,
}

fn main() {}
