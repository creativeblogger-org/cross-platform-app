use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{user::Author, comment::Comment};

#[derive(Debug, Deserialize)]
pub struct PreviewPost {
    pub title: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Author,
    pub description: String,
    //changement bientôt en Vec<String>
    pub tags: String,
    pub image: String,
    pub has_permission: bool
}

#[derive(Debug, Deserialize, Default)]
pub struct Post {
    pub id: u128,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Author,
    pub description: String,
    pub tags: String,
    pub has_permission: bool,
    pub comments: Vec<Comment>
}