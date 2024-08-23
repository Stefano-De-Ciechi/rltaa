pub mod playlists;
pub mod artists;
pub mod albums;

use serde::{Serialize, Deserialize};     // by declaring it in there, every submodule that uses crate::api_structs::* will have access to Deserialize
use std::{fs::File, io::BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,    // this is the url to be passed to zotify
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub display_name: String,
    pub href: String,
    pub id: String,
    #[serde(alias = "type")] pub obj_type: String,
}
