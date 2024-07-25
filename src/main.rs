/*
* author: Stefano De Ciechi
* purpose: create a simple gui for zotify, a spotify downloader
* date: 2024-07-25
*/

mod api_structs;

use std::{fs::File, io::BufReader};
use crate::api_structs::playlists::{FollowedPlaylists, debug_print_followed_playlists};

fn main() {
    let file = File::open("./data/followed_playlists.json").expect("could not open the JSON file");
    let reader = BufReader::new(file);

    let data: FollowedPlaylists = serde_json::from_reader(reader).unwrap();

    debug_print_followed_playlists(&data);
}
