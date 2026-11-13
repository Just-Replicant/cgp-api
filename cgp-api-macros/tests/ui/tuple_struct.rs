use cgp_api_macros::entity_api;

#[entity_api(
    policy = api,
    layer = lookup,
    entity = post::Entity,
    unique = PostUniqueColumns,
)]
pub struct PostApi(pub u32);

fn main() {}
