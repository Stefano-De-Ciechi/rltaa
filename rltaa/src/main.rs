/*
* author: Stefano De Ciechi
* purpose: generate ascii art starting from song lyrics retrieved from spotify and genius api
* date: 2024-07-25
*/

use spotify_api_wrapper::api_structs::{albums, artists, playlists};
use spotify_api_wrapper::SpotifyAPI;

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
        //api_client.search_data("Tell+me+I'm+pretty", "album", 5);
    }

    
    let artists = artists::get_followed();
    artists::debug_print_followed(&artists);

    let playlists = playlists::get_followed();
    playlists::debug_print_followed(&playlists);

    let albums = albums::get_saved();
    albums::debug_print_saved(&albums);
}
