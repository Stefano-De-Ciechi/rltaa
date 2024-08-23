/*
* author: Stefano De Ciechi
* purpose: create a simple gui for zotify, a spotify downloader
* date: 2024-07-25
*/

mod api_structs;
mod spotify_api;

use api_structs::{albums::{debug_print_saved_albums, get_saved_albums}, artists::{debug_print_followed_artists, get_followed_artists}, playlists::{debug_print_followed_playlists, get_followed_playlists}};
use spotify_api::SpotifyAPI;

use dotenvy::dotenv;

fn main() {

    dotenv().expect("could not load a .env file");

    let mut _api_client = SpotifyAPI::new();

    /*_api_client.refresh_token();
    _api_client.update_followed_artists();
    _api_client.update_followed_playlists();
    _api_client.update_saved_albums();*/

    let artists_path = "./data/followed_artists.json";
    let artists = get_followed_artists(artists_path);
    debug_print_followed_artists(&artists);

    let playlists_path = "./data/followed_playlists.json";
    let playlists = get_followed_playlists(playlists_path);
    debug_print_followed_playlists(&playlists);

    let albums_path = "./data/saved_albums.json";
    let albums = get_saved_albums(albums_path);
    debug_print_saved_albums(&albums);
}
