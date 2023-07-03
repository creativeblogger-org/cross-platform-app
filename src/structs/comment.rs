use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::user::Author;

#[derive(Debug, Deserialize)]
pub struct Comment {
    id: u128,
    author: Author,
    content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    has_permission: bool
}