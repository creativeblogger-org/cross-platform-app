use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Author {
    id: u128,
    pub username: String,
    pub permission: u8
}

pub enum PermissionLevel {
    Membre,
    Rédacteur,
    Modérateur,
    Administrateur
}