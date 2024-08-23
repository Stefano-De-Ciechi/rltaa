use crate::api_structs::{Serialize, Deserialize, File, BufReader, ExternalUrls};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowedArtists {
    pub artists: FollowedArtistsItems,
}

/*
* ignored fields: next, cursors, limit, href
*/
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowedArtistsItems {
    pub items: Vec<Artist>,
    pub total: u32,
}

/*
* ignored fields: images, popularity, uri
*/
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrls, 
    pub genres: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(alias = "type")] pub obj_type: String, 
}

pub fn debug_print_followed_artists(artists: &Vec<Artist>) {
    println!("\ntotal: {}", artists.len());
    println!("{:<50} | {:<150}", "name", "genres");
    println!("{}", "-".repeat(200));

    for a in artists {
        let genres = match &a.genres {
            Some(g) => g,
            None => &vec![],
        };
        println!("{:<50} | {:<150?}", a.name, genres);
    }
}

pub fn get_followed_artists(file_path: &str) -> Vec<Artist> {
    let file = File::open(file_path).expect("could not open the JSON file");
    let reader = BufReader::new(file);

    let artists: FollowedArtists = serde_json::from_reader(reader).unwrap();
    artists.artists.items 
}

