use crate::api_structs::*;

/*
* ignored fields: href, limit, next, offset, previous
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FollowedPlaylists {
    items: Vec<Playlist>,
    total : u32,
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

pub fn debug_print_followed_playlists(playlists: &Vec<Playlist>) {
    println!("\ntotal: {}", playlists.len());
    println!("{:<100} | {:>10} | {:>11} | {:>12}", "name", "pub.", "coll.", "tracks num.");
    println!("{}", "-".repeat(143));

    for p in playlists {
        println!("{:<100} | {:>10} | {:>11} | {:>12}", p.name, p.public, p.collaborative, p.tracks.total);
    }
}

pub fn get_followed_playlists(file_path: &str) -> Vec<Playlist> {
    let file = File::open(file_path).expect("could not open the JSON file");
    let reader = BufReader::new(file);

    let playlists: FollowedPlaylists = serde_json::from_reader(reader).unwrap();
    playlists.items
}
