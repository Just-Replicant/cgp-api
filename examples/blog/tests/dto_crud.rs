//! The `cgp-api` DTO-envelope blanket impls on a real SQLite context.
//!
//! `TagApp` has no DTO-specific code: `TagDto` implements `HasId` once and
//! the context's create input, update input, and output are all `TagDto`.

use blog_example::entity::tag;
use blog_example::{TagApp, TagDto};
use cgp_api::prelude::*;
use sea_orm::{ConnectionTrait, Database, Schema};

async fn setup() -> TagApp {
    let db = Database::connect("sqlite::memory:")
        .await
        .expect("in-memory sqlite");
    let schema = Schema::new(db.get_database_backend());
    db.execute(&schema.create_table_from_entity(tag::Entity))
        .await
        .expect("create table");
    for index in schema.create_index_from_entity(tag::Entity) {
        db.execute(&index).await.expect("create index");
    }
    TagApp::new(db)
}

fn tag(id: Option<i32>, name: &str) -> TagDto {
    TagDto {
        id,
        name: name.to_owned(),
    }
}

#[tokio::test]
async fn create_by_dto_inserts_then_returns_the_existing_identity() {
    let tags = setup().await;

    let (created, outcome) = tags.create_by_dto(tag(None, "rust")).await.expect("create");
    assert_eq!(outcome, InsertOutcome::Inserted);
    let id = created.id.expect("the stored tag has an id");
    assert_eq!(created, tag(Some(id), "rust"));

    // Same unique name: get-or-create returns the stored row.
    let (again, outcome) = tags
        .create_by_dto(tag(None, "rust"))
        .await
        .expect("get_or_create");
    assert_eq!(outcome, InsertOutcome::Existing);
    assert_eq!(again, created);
}

#[tokio::test]
async fn update_dto_by_id_uses_the_envelope_id() {
    let tags = setup().await;
    let (created, _) = tags.create_by_dto(tag(None, "rust")).await.expect("create");

    let updated = tags
        .update_dto_by_id(tag(created.id, "rustlang"))
        .await
        .expect("update");
    assert_eq!(updated, Some(tag(created.id, "rustlang")));

    let missing = tags
        .update_dto_by_id(tag(Some(9_999), "ghost"))
        .await
        .expect("update of an unknown id");
    assert_eq!(missing, None);
}

#[tokio::test]
async fn update_dto_by_id_without_an_id_is_primary_key_unset() {
    let tags = setup().await;
    tags.create_by_dto(tag(None, "rust")).await.expect("create");

    let error = tags
        .update_dto_by_id(tag(None, "rustlang"))
        .await
        .expect_err("an update needs an id");
    assert!(
        matches!(&error, ApiError::PrimaryKeyUnset { column } if column == "Id"),
        "got {error:?}"
    );

    // Nothing was written.
    let names: Vec<String> = tags
        .list_dto(None, None)
        .await
        .expect("list")
        .into_iter()
        .map(|dto| dto.name)
        .collect();
    assert_eq!(names, ["rust"]);
}

#[tokio::test]
async fn list_dto_pages_with_policy_defaults() {
    let tags = setup().await;
    for name in ["a", "b", "c", "d", "e"] {
        tags.create_by_dto(tag(None, name)).await.expect("create");
    }

    let all = tags.list_dto(None, None).await.expect("first page");
    assert_eq!(all.len(), 5, "the default policy size covers five rows");

    let names = |page: Vec<TagDto>| page.into_iter().map(|dto| dto.name).collect::<Vec<_>>();
    assert_eq!(
        names(tags.list_dto(Some(1), Some(2)).await.expect("page 1")),
        ["a", "b"]
    );
    assert_eq!(
        names(tags.list_dto(Some(3), Some(2)).await.expect("page 3")),
        ["e"]
    );
    // Page 0 is the first page; size 0 is the policy default.
    assert_eq!(
        tags.list_dto(Some(0), Some(0)).await.expect("page 0"),
        tags.list_dto(None, None).await.expect("defaults"),
    );
    assert!(
        tags.list_dto(Some(9), Some(2))
            .await
            .expect("past the end")
            .is_empty()
    );
}

/// Fails to compile when `$ty` implements the trait: two candidate impls
/// make the inferred `_` ambiguous (the `static_assertions`
/// `assert_not_impl_any!` technique, without the dependency).
macro_rules! assert_not_impl {
    ($ty:ty: $($bound:tt)+) => {
        const _: fn() = || {
            trait AmbiguousIfImpl<A> {
                fn some_item() {}
            }
            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            struct Invalid;
            impl<T: ?Sized + $($bound)+> AmbiguousIfImpl<Invalid> for T {}
            let _ = <$ty as AmbiguousIfImpl<_>>::some_item;
        };
    };
}

/// Only an envelope context gets the envelope methods. `UserApp` and
/// `PostApp` share `Output` with nothing else (distinct create / update
/// structs), so `list_dto` — which would drop the page and skip the
/// count — is not available; they list through `CanListResources::list`.
mod envelope_only {
    use blog_example::{PostApp, PostOutput, TagApp, TagDto, UserApp, UserOutput};
    use cgp_api::prelude::*;
    use cgp_orm::components::service::CanListResources;

    const _: fn() = || {
        fn envelope<App>()
        where
            App: CanCreateByDto<TagDto> + CanUpdateDtoById<TagDto> + CanListDto<TagDto>,
        {
        }
        envelope::<TagApp>();

        fn pages<App: CanListResources>() {}
        pages::<UserApp>();
        pages::<PostApp>();
    };

    assert_not_impl!(UserApp: CanListDto<UserOutput>);
    assert_not_impl!(PostApp: CanListDto<PostOutput>);
    assert_not_impl!(UserApp: CanCreateByDto<UserOutput>);
    assert_not_impl!(PostApp: CanUpdateDtoById<PostOutput>);
}

/// The futures cross threads, which `flutter_rust_bridge` requires.
#[tokio::test]
async fn futures_are_send() {
    let tags = setup().await;
    let handle = tokio::spawn(async move {
        let (created, _) = tags.create_by_dto(tag(None, "send")).await?;
        tags.update_dto_by_id(created).await?;
        tags.list_dto(None, None).await
    });
    let listed = handle.await.expect("join").expect("crud");
    assert_eq!(listed.len(), 1);
}

/// The same blanket impls on a REST context: a DTO without an id is a 400.
mod rest_stack {
    use blog_example::TagDto;
    use blog_example::codec::TagCodec;
    use blog_example::entity::tag;
    use blog_example::identity::{TagEntityName, TagUniqueColumns};
    use cgp_api::error::RestError;
    use cgp_api::prelude::*;
    use cgp_api::presets::RestErrorPolicy;
    use cgp_api::providers::ConstEntityName;
    use cgp_orm::presets::{RepositoryCrudStack, ResourceServiceStack};

    cgp_preset! {
        TagRestPreset: RepositoryCrudStack + ResourceServiceStack + RestErrorPolicy {
            cgp_orm::types::EntityTypeProviderComponent:
                UseType<tag::Entity>,
            [
                cgp_orm::components::service::CreateInputTypeProviderComponent,
                cgp_orm::components::service::UpdateInputTypeProviderComponent,
                cgp_orm::components::service::OutputTypeProviderComponent,
            ]:
                UseType<TagDto>,
            [
                cgp_orm::components::DecodeCreateInputComponent,
                cgp_orm::components::ApplyUpdateInputComponent,
                cgp_orm::components::EncodeOutputComponent,
            ]: TagCodec,
            cgp_orm::accessors::UniqueColumnsGetterComponent:
                TagUniqueColumns,
            cgp_api::accessors::EntityNameGetterComponent:
                ConstEntityName<TagEntityName>,
        }
    }

    #[derive(HasField, Clone)]
    struct TagRestApp {
        pub db_conn: sea_orm::DatabaseConnection,
    }

    delegate_components! {
        #[unchecked]
        TagRestApp {
            preset TagRestPreset,
        }
    }

    #[tokio::test]
    async fn missing_id_is_bad_request() {
        let tags = TagRestApp {
            db_conn: super::setup().await.db_conn,
        };
        let error: RestError = tags
            .update_dto_by_id(super::tag(None, "rust"))
            .await
            .expect_err("an update needs an id");
        assert_eq!(error.status.as_u16(), 400);
        assert_eq!(error.message, "primary-key field Id is not set");
    }
}
