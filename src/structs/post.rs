use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::user::Author;

#[derive(Debug, Deserialize)]
pub struct PreviewPost {
    id: u128,
    pub title: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Author,
    pub description: String,
    //changement bientôt en Vec<String>
    tags: String,
    has_permission: bool
}