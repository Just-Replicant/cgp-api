//! In-memory SQLite tests for the blog example: UUID users, distinct
//! post DTOs, and a composite-key favorite.

use blog_example::entity::{favorite, post, user};
use blog_example::{CreatePost, CreateUser, FavoriteApp, PostApp, UpdatePost, UpdateUser, UserApp};
use cgp_api::prelude::*;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, EntityTrait, Schema, Set};
use uuid::Uuid;

async fn connect() -> DatabaseConnection {
    Database::connect("sqlite::memory:")
        .await
        .expect("in-memory sqlite")
}

async fn create_table<E: EntityTrait>(db: &DatabaseConnection, entity: E) {
    let schema = Schema::new(db.get_database_backend());
    let table = schema.create_table_from_entity(entity);
    db.execute(&table).await.expect("create table");
    for index in schema.create_index_from_entity(entity) {
        db.execute(&index).await.expect("create index");
    }
}

async fn setup() -> (UserApp, PostApp, FavoriteApp, DatabaseConnection) {
    let db = connect().await;
    create_table(&db, user::Entity).await;
    create_table(&db, post::Entity).await;
    create_table(&db, favorite::Entity).await;
    (
        UserApp::new(db.clone()),
        PostApp::new(db.clone()),
        FavoriteApp::new(db.clone()),
        db,
    )
}

#[tokio::test]
async fn user_uuid_create_get_update_and_get_or_create() {
    let (users, _, _, _) = setup().await;
    assert_eq!(users.entity_name(), "users");

    let (created, outcome) = users
        .create(CreateUser {
            email: "ada@example.com".into(),
            name: "Ada".into(),
        })
        .await
        .expect("create");
    assert_eq!(outcome, InsertOutcome::Inserted);
    assert_ne!(created.id, Uuid::nil());
    assert_eq!(created.email, "ada@example.com");
    assert_eq!(created.name, "Ada");

    let fetched = users.get(created.id).await.expect("get").expect("present");
    assert_eq!(fetched.id, created.id);

    let updated = users
        .update(
            created.id,
            UpdateUser {
                name: "Ada Lovelace".into(),
            },
        )
        .await
        .expect("update")
        .expect("present");
    assert_eq!(updated.name, "Ada Lovelace");
    // Email is NotSet on update and must be preserved.
    assert_eq!(updated.email, "ada@example.com");
    assert_eq!(updated.id, created.id);

    let (again, outcome) = users
        .create(CreateUser {
            email: "ada@example.com".into(),
            name: "Someone else".into(),
        })
        .await
        .expect("get_or_create");
    assert_eq!(outcome, InsertOutcome::Existing);
    assert_eq!(again.id, created.id);
    assert_eq!(again.name, "Ada Lovelace");
}

#[tokio::test]
async fn post_distinct_dtos_and_list() {
    let (users, posts, _, _) = setup().await;
    assert_eq!(posts.entity_name(), "posts");

    let (author, _) = users
        .create(CreateUser {
            email: "grace@example.com".into(),
            name: "Grace".into(),
        })
        .await
        .unwrap();

    let (first, _) = posts
        .create(CreatePost {
            author_id: author.id,
            slug: "hello".into(),
            title: "Hello".into(),
            body: "v1".into(),
        })
        .await
        .expect("create");
    assert_eq!(first.id, 1);
    assert_eq!(first.slug, "hello");
    assert_eq!(first.author_id, author.id);

    let updated = posts
        .update(
            first.id,
            UpdatePost {
                title: "Hello, world".into(),
                body: "v2".into(),
            },
        )
        .await
        .expect("update")
        .unwrap();
    assert_eq!(updated.title, "Hello, world");
    assert_eq!(updated.body, "v2");
    // slug and author are NotSet on update.
    assert_eq!(updated.slug, "hello");
    assert_eq!(updated.author_id, author.id);

    let (_second, _) = posts
        .create(CreatePost {
            author_id: author.id,
            slug: "second".into(),
            title: "Second".into(),
            body: "".into(),
        })
        .await
        .unwrap();

    let page = posts.list(PageRequest::new(1, 1)).await.expect("list");
    assert_eq!(page.page, 1);
    assert_eq!(page.size, 1);
    assert_eq!(page.total, Some(2));
    assert_eq!(page.items[0].id, first.id);

    // Wrap through the generic transport helper.
    let response = response_from_page(page);
    assert_eq!(response.meta.unwrap().total, Some(2));
    assert_eq!(response.data.len(), 1);
}

#[tokio::test]
async fn favorite_composite_key_get_or_create() {
    let (users, posts, favorites, _) = setup().await;
    assert_eq!(favorites.entity_name(), "favorites");

    let (user, _) = users
        .create(CreateUser {
            email: "alan@example.com".into(),
            name: "Alan".into(),
        })
        .await
        .unwrap();
    let (post, _) = posts
        .create(CreatePost {
            author_id: user.id,
            slug: "notes".into(),
            title: "Notes".into(),
            body: "".into(),
        })
        .await
        .unwrap();

    let (first, outcome) = favorites
        .get_or_create(favorite::ActiveModel {
            user_id: Set(user.id),
            post_id: Set(post.id),
            note: Set("keep".into()),
        })
        .await
        .expect("insert");
    assert_eq!(outcome, InsertOutcome::Inserted);
    assert_eq!(first.note, "keep");

    let (again, outcome) = favorites
        .get_or_create(favorite::ActiveModel {
            user_id: Set(user.id),
            post_id: Set(post.id),
            note: Set("changed".into()),
        })
        .await
        .expect("conflict");
    assert_eq!(outcome, InsertOutcome::Existing);
    assert_eq!(again.user_id, user.id);
    assert_eq!(again.post_id, post.id);
    // Existing row is unchanged.
    assert_eq!(again.note, "keep");

    let found = favorites
        .get_by_id((user.id, post.id))
        .await
        .expect("get_by_id")
        .expect("present");
    assert_eq!(found.note, "keep");
}

#[tokio::test]
async fn post_create_duplicate_slug_returns_existing() {
    let (users, posts, _, _) = setup().await;
    let (author, _) = users
        .create(CreateUser {
            email: "x@example.com".into(),
            name: "X".into(),
        })
        .await
        .unwrap();
    let input = CreatePost {
        author_id: author.id,
        slug: "same".into(),
        title: "A".into(),
        body: "".into(),
    };
    let (first, outcome) = posts.create(input.clone()).await.expect("first");
    assert_eq!(outcome, InsertOutcome::Inserted);
    let (again, outcome) = posts
        .create(CreatePost {
            title: "B".into(),
            ..input
        })
        .await
        .expect("duplicate slug");
    assert_eq!(outcome, InsertOutcome::Existing);
    assert_eq!(again.id, first.id);
    assert_eq!(again.title, "A");
}

/// Genericity proof: a `RepositoryCrudStack + ResourceServiceStack +
/// RestErrorPolicy` context on SQLite, with a unique conflict surfaced
/// as [`RestError`]. SeaORM sqlite and entity macros stay in this
/// example's dev-dependencies, not in `cgp-api`.
mod rest_stack {
    use core::error::Error;

    use blog_example::codec::UserCodec;
    use blog_example::dto::{CreateUser, UpdateUser, UserOutput};
    use blog_example::entity::user;
    use blog_example::identity::{UserEntityName, UserUniqueColumns};
    use cgp_api::error::{INTERNAL_ERROR_MESSAGE, RestError};
    use cgp_api::prelude::*;
    use cgp_api::presets::RestErrorPolicy;
    use cgp_api::providers::ConstEntityName;
    use cgp_orm::presets::{RepositoryCrudStack, ResourceServiceStack};
    use sea_orm::Set;
    use uuid::Uuid;

    use super::{connect, create_table};

    cgp_preset! {
        UserRestPreset: RepositoryCrudStack + ResourceServiceStack + RestErrorPolicy {
            cgp_orm::types::EntityTypeProviderComponent:
                UseType<user::Entity>,
            cgp_orm::components::service::CreateInputTypeProviderComponent:
                UseType<CreateUser>,
            cgp_orm::components::service::UpdateInputTypeProviderComponent:
                UseType<UpdateUser>,
            cgp_orm::components::service::OutputTypeProviderComponent:
                UseType<UserOutput>,
            [
                cgp_orm::components::DecodeCreateInputComponent,
                cgp_orm::components::ApplyUpdateInputComponent,
                cgp_orm::components::EncodeOutputComponent,
            ]: UserCodec,
            cgp_orm::accessors::UniqueColumnsGetterComponent:
                UserUniqueColumns,
            cgp_api::accessors::EntityNameGetterComponent:
                ConstEntityName<UserEntityName>,
        }
    }

    #[derive(HasField, Clone)]
    struct UserRestApp {
        pub db_conn: sea_orm::DatabaseConnection,
    }

    delegate_components! {
        #[unchecked]
        UserRestApp {
            preset UserRestPreset,
        }
    }

    #[tokio::test]
    async fn rest_error_policy_unique_conflict_is_rest_error() {
        let db = connect().await;
        create_table(&db, user::Entity).await;
        let users = UserRestApp { db_conn: db };

        let first = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set("ada@example.com".into()),
            name: Set("Ada".into()),
        };
        users.insert(first).await.expect("first insert");

        let conflict = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set("ada@example.com".into()),
            name: Set("Someone else".into()),
        };
        let error: RestError = users
            .insert(conflict)
            .await
            .expect_err("duplicate email is a unique conflict");

        assert_eq!(error.status.as_u16(), 500);
        assert_eq!(error.message, INTERNAL_ERROR_MESSAGE);
        assert_eq!(error.to_string(), INTERNAL_ERROR_MESSAGE);
        let message = error.message.clone();
        let pair: (_, String) = error.clone().into();
        assert_eq!(pair.1, message);
        let source = error.source().expect("original DbErr");
        let text = source.to_string();
        assert!(
            text.to_lowercase().contains("unique") || text.to_lowercase().contains("constraint"),
            "in-process source should still describe the unique conflict, got {text}"
        );
        assert!(source.downcast_ref::<sea_orm::DbErr>().is_some());
    }
}
