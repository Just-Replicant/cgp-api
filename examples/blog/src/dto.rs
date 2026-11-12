//! Distinct Create / Update / Output types. Not a single envelope.

use uuid::Uuid;

/// Input for creating a user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUser {
    /// Unique email.
    pub email: String,
    /// Display name.
    pub name: String,
}

/// Input for updating a user (email is not changed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUser {
    /// New display name.
    pub name: String,
}

/// Public user representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserOutput {
    /// User id.
    pub id: Uuid,
    /// Email.
    pub email: String,
    /// Display name.
    pub name: String,
}

/// Input for creating a post.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePost {
    /// Author user id.
    pub author_id: Uuid,
    /// Unique slug.
    pub slug: String,
    /// Title.
    pub title: String,
    /// Body.
    pub body: String,
}

/// Input for updating a post (slug and author are not changed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePost {
    /// New title.
    pub title: String,
    /// New body.
    pub body: String,
}

/// Public post representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostOutput {
    /// Post id.
    pub id: i32,
    /// Author user id.
    pub author_id: Uuid,
    /// Slug.
    pub slug: String,
    /// Title.
    pub title: String,
    /// Body.
    pub body: String,
}
