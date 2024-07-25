use crate::api_structs::*;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FollowedPlaylists {
    pub href: String,
    pub items: Vec<Playlist>,
    pub limit : u32,
    pub next : Option<String>,
    pub offset : u32,
    pub previous : Option<String>,
    pub total : u32,
}

/*
* ignored fields: images, primary-color, snapshot-id, tracks, uri
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub collaborative: bool,
    pub description: String,

    #[serde(alias = "external-urls")]
    pub external_urls: ExternalUrls,

    pub href: String,
    pub id: String,
    pub name: String,
    pub owner: Owner,
    pub public: bool,
    pub tracks: Tracks,

    #[serde(alias = "type")]
    pub obj_type: String,
}

/*
* ignored fields: href
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Tracks {
    pub total: u32,
}

pub fn debug_print_followed_playlists(playlists: &FollowedPlaylists) {
    println!("{:<100} | {:>10} | {:>11} | {:>12}", "name", "pub.", "coll.", "tracks num.");
    println!("{}", "-".repeat(143));

    for p in &playlists.items {
        println!("{:<100} | {:>10} | {:>11} | {:>12}", p.name, p.public, p.collaborative, p.tracks.total);
    }
}
