//! Blog entities: a UUID-key user, an i32-key post, and a composite-key favorite.

pub mod user {
    use sea_orm::entity::prelude::*;

    /// A user identified by a UUID, with a unique email.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        /// Application-assigned UUID primary key.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Unique identity.
        #[sea_orm(unique)]
        pub email: String,
        /// Display name.
        pub name: String,
    }

    /// Relations (none in this example).
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod post {
    use sea_orm::entity::prelude::*;

    /// A post with an autoincrement id and a unique slug.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        /// Autoincrement primary key.
        #[sea_orm(primary_key)]
        pub id: i32,
        /// Author ([`super::user`] id).
        pub author_id: Uuid,
        /// Unique identity.
        #[sea_orm(unique)]
        pub slug: String,
        /// Title.
        pub title: String,
        /// Body.
        pub body: String,
    }

    /// Relations (none in this example).
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod favorite {
    use sea_orm::entity::prelude::*;

    /// A user's favorite post. Composite primary key `(user_id, post_id)`,
    /// which is also the identity.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "favorites")]
    pub struct Model {
        /// User id (part of the composite key).
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// Post id (part of the composite key).
        #[sea_orm(primary_key, auto_increment = false)]
        pub post_id: i32,
        /// Optional note.
        pub note: String,
    }

    /// Relations (none in this example).
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
