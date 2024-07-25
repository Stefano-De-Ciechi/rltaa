pub mod playlists;

use serde::Deserialize;     // by declaring it in there, every submodule that uses crate::api_structs::* will have access to Deserialize

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Owner {
    pub display_name: String,
    pub href: String,
    pub id: String,

    #[serde(alias = "type")]
    pub obj_type: String,
}
