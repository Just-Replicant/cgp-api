use cgp_api_macros::entity_api;

#[entity_api(
    policy = api,
    layer = lookup,
    entity = post::Entity,
    unique = PostUniqueColumns,
    also(PostResource: rest, layer = resource),
)]
pub struct PostApi {
    pub db_conn: u32,
}

fn main() {}
