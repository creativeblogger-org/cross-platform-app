use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Author {
    pub username: String,
    pub permission: u8
}

#[derive(Debug, Deserialize, Default)]
pub enum PermissionLevel {
    #[default]
    Membre,
    Rédacteur,
    Modérateur,
    Administrateur
}