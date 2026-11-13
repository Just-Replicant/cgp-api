use cgp_api_macros::entity_api;

#[entity_api(
    policy = api,
    entity = post::Entity,
    create = CreatePost,
    update = UpdatePost,
    output = PostOutput,
    unique = PostUniqueColumns,
)]
pub struct PostApi {
    pub db_conn: u32,
}

fn main() {}
