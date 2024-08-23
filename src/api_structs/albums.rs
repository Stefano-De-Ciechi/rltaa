use crate::api_structs::{Deserialize, ExternalUrls, File, BufReader};
use crate::api_structs::artists::Artist;

/*
* ignored fields: href, limit, next, offset, previous
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct SavedAlbums {
    total: u32,
    items: Vec<SavedAlbumsItem>,
} 

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SavedAlbumsItem {
    added_at: String,
    album: Album,
}

/*
* ignored fields: 
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Album {
    #[serde(alias = "album_type")]
    _type: String,   // album or single
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
    name: String,
    total_tracks: u32,
    tracks: Tracks,

    #[serde(alias = "type")]
    obj_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Tracks {
    items: Vec<TracksItem>,
    total: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct TracksItem {
    external_urls: ExternalUrls, 
    name: String,
}

pub fn debug_print_saved_albums(albums: &Vec<SavedAlbumsItem>) {
    println!("\ntotal: {}", albums.len());
    println!("{:<50} | {:<11}", "name", "tot. tracks");
    println!("{}", "-".repeat(200));

    for a in albums {
        let a = &a.album;
        println!("{:<50} | {}", a.name, a.total_tracks);
    }
}

pub fn get_saved_albums(file_path: &str) -> Vec<SavedAlbumsItem> {
    let file = File::open(file_path).expect("could not open the JSON file");
    let reader = BufReader::new(file);

    let saved_albums: SavedAlbums = serde_json::from_reader(reader).unwrap();
    saved_albums.items
}
