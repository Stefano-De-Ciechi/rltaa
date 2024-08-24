use crate::api_structs::{Deserialize, Serialize, ExternalUrls, File, BufReader};
use crate::api_structs::artists::Artist;

/*
* ignored fields: href, limit, next, offset, previous
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Saved {
    total: u32,
    items: Vec<SavedAlbumsItem>,
} 

impl Saved {
    const fn empty() -> Self {
        Self {
            total: 0,
            items: vec![],
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SavedAlbumsItem {
    added_at: String,
    album: Album,
}

/*
* ignored fields: 
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
struct Tracks {
    items: Vec<TracksItem>,
    total: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct TracksItem {
    external_urls: ExternalUrls, 
    name: String,
}

pub fn debug_print_saved(albums: &Vec<SavedAlbumsItem>) {
    println!("\n===== ALBUMS =====\ntotal: {}", albums.len());
    println!("{:<50} | {:<11}", "name", "tot. tracks");
    println!("{}", "-".repeat(200));

    for a in albums {
        let a = &a.album;
        println!("{:<50} | {}", a.name, a.total_tracks);
    }
}

pub fn get_saved() -> Vec<SavedAlbumsItem> {
    get_saved_p("./data/saved_albums.json")
}

pub fn get_saved_p(file_path: &str) -> Vec<SavedAlbumsItem> {
    let Ok(file) = File::open(file_path) else { 
        eprintln!("couldn't open followed_artists.json");
        return vec![];
    };

    let reader = BufReader::new(file);

    let albums: Saved = serde_json::from_reader(reader)
        .unwrap_or_else(|_| {
            eprintln!("couldn't deserialize followed artists from json file");
            Saved::empty()
    });

    albums.items

}
