use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Author {
    id: u128,
    pub username: String,
    pub permission: u8
}