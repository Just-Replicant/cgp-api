//! Codecs between the distinct DTOs and SeaORM active models / models.

use cgp_fork::prelude::*;
use cgp_orm::components::service::{
    ApplyUpdateInput, ApplyUpdateInputComponent, DecodeCreateInput, DecodeCreateInputComponent,
    EncodeOutput, EncodeOutputComponent, HasCreateInputType, HasOutputType, HasUpdateInputType,
};
use cgp_orm::types::HasEntityType;
use sea_orm::{NotSet, Set};
use uuid::Uuid;

use crate::dto::{CreatePost, CreateUser, PostOutput, UpdatePost, UpdateUser, UserOutput};
use crate::entity::{post, user};

/// Codec for the user resource.
pub struct UserCodec;

#[cgp_impl(UserCodec)]
#[use_type(
    HasEntityType.{Entity = user::Entity},
    HasErrorType.Error,
    HasCreateInputType.{CreateInput = CreateUser}
)]
impl DecodeCreateInput {
    fn decode_create_input(&self, input: CreateUser) -> Result<user::ActiveModel, Error> {
        Ok(user::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(input.email),
            name: Set(input.name),
        })
    }
}

#[cgp_impl(UserCodec)]
#[use_type(
    HasEntityType.{Entity = user::Entity},
    HasErrorType.Error,
    HasUpdateInputType.{UpdateInput = UpdateUser}
)]
impl ApplyUpdateInput {
    fn apply_update_input(&self, id: Uuid, input: UpdateUser) -> Result<user::ActiveModel, Error> {
        Ok(user::ActiveModel {
            id: Set(id),
            email: NotSet,
            name: Set(input.name),
        })
    }
}

#[cgp_impl(UserCodec)]
#[use_type(
    HasEntityType.{Entity = user::Entity},
    HasErrorType.Error,
    HasOutputType.{Output = UserOutput}
)]
impl EncodeOutput {
    fn encode_output(&self, model: user::Model) -> Result<UserOutput, Error> {
        Ok(UserOutput {
            id: model.id,
            email: model.email,
            name: model.name,
        })
    }
}

/// Codec for the post resource.
pub struct PostCodec;

#[cgp_impl(PostCodec)]
#[use_type(
    HasEntityType.{Entity = post::Entity},
    HasErrorType.Error,
    HasCreateInputType.{CreateInput = CreatePost}
)]
impl DecodeCreateInput {
    fn decode_create_input(&self, input: CreatePost) -> Result<post::ActiveModel, Error> {
        Ok(post::ActiveModel {
            id: NotSet,
            author_id: Set(input.author_id),
            slug: Set(input.slug),
            title: Set(input.title),
            body: Set(input.body),
        })
    }
}

#[cgp_impl(PostCodec)]
#[use_type(
    HasEntityType.{Entity = post::Entity},
    HasErrorType.Error,
    HasUpdateInputType.{UpdateInput = UpdatePost}
)]
impl ApplyUpdateInput {
    fn apply_update_input(&self, id: i32, input: UpdatePost) -> Result<post::ActiveModel, Error> {
        Ok(post::ActiveModel {
            id: Set(id),
            author_id: NotSet,
            slug: NotSet,
            title: Set(input.title),
            body: Set(input.body),
        })
    }
}

#[cgp_impl(PostCodec)]
#[use_type(
    HasEntityType.{Entity = post::Entity},
    HasErrorType.Error,
    HasOutputType.{Output = PostOutput}
)]
impl EncodeOutput {
    fn encode_output(&self, model: post::Model) -> Result<PostOutput, Error> {
        Ok(PostOutput {
            id: model.id,
            author_id: model.author_id,
            slug: model.slug,
            title: model.title,
            body: model.body,
        })
    }
}
