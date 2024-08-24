use crate::api_structs::{Serialize, Deserialize, File, BufReader, ExternalUrls};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Followed{
    pub artists: FollowedArtistsItems,
}

impl Followed {
    const fn empty() -> Self {
        Self {
            artists: FollowedArtistsItems { items: vec![], total: 0 }
        } 
    }
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

pub fn debug_print_followed(artists: &Vec<Artist>) {
    println!("\n===== ARTISTS =====\ntotal: {}", artists.len());
    println!("{:<50} | {:<150}", "name", "genres");
    println!("{}", "-".repeat(200));

    for a in artists {
        let genres: &Vec<String> = if let Some(g) = &a.genres { g } else { &vec![] };
        println!("{:<50} | {:<150?}", a.name, genres);
    }
}

pub fn get_followed() -> Vec<Artist> {
    get_followed_p("./data/followed_artists.json")
}

pub fn get_followed_p(file_path: &str) -> Vec<Artist> {
    let Ok(file) = File::open(file_path) else { 
        eprintln!("couldn't open followed_artists.json");
        return vec![];
    };

    let reader = BufReader::new(file);

    let artists: Followed = serde_json::from_reader(reader)
        .unwrap_or_else(|_| {
            eprintln!("couldn't deserialize followed artists from json file");
            Followed::empty()
    });

    artists.artists.items 
}

