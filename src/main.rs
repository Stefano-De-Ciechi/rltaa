/*
* author: Stefano De Ciechi
* purpose: create a simple gui for zotify, a spotify downloader
* date: 2024-07-25
*/

mod api_structs;
mod spotify_api;

//use api_structs::{albums::{debug_print_saved_albums, get_saved_albums}, artists::{debug_print_followed_artists, get_followed_artists}, playlists::{debug_print_followed_playlists, get_followed_playlists}};

use api_structs::artists::{debug_print_followed_artists, get_followed_artists};
use spotify_api::SpotifyAPI;

use dotenvy::dotenv;

fn main() {

    dotenv().expect("could not load a .env file");

    /*let playlists_path = "./data/followed_playlists.json";
    let playlists = get_followed_playlists(playlists_path);
    debug_print_followed_playlists(&playlists);

    let artists_path = "./data/followed_artists.json";
    let artists = get_followed_artists(artists_path);
    debug_print_followed_artists(&artists);

    let albums_path = "./data/saved_albums.json";
    let albums = get_saved_albums(albums_path);
    debug_print_saved_albums(&albums);*/

    let mut api_client = SpotifyAPI::new();
    /*let artists = api_client.get_followed_artists();
    println!("{:?}", artists);*/

    api_client.refresh_token();

    api_client.update_followed_artists();

    let artists_path = "./data/followed_artists.json";
    let artists = get_followed_artists(artists_path);
    debug_print_followed_artists(&artists);
}
