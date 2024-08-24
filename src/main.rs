/*
* author: Stefano De Ciechi
* purpose: create a simple gui for zotify, a spotify downloader
* date: 2024-07-25
*/

mod api_structs;
mod spotify_api;

use api_structs::{albums, artists, playlists};
use spotify_api::SpotifyAPI;

use dotenvy::dotenv;

fn main() {

    match dotenv() {
        Ok(_) => {},
        Err(err) => {
            eprintln!("error loading .env file: {err}");
            panic!("");
        }
    }

    if false { 
        let mut api_client = SpotifyAPI::new();

        api_client.refresh_token();
        api_client.update_followed_artists();
        api_client.update_followed_playlists();
        api_client.update_saved_albums();
    }

    let artists = artists::get_followed();
    artists::debug_print_followed(&artists);

    let playlists = playlists::get_followed();
    playlists::debug_print_followed(&playlists);

    let albums = albums::get_saved();
    albums::debug_print_saved(&albums);
}
