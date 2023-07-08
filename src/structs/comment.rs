use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::user::Author;

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: u128,
    pub author: Author,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub has_permission: bool
}