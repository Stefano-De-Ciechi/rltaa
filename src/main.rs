/*
* author: Stefano De Ciechi
* purpose: create a simple gui for zotify, a spotify downloader
* date: 2024-07-25
*/

mod api_structs;

use api_structs::{artists::{debug_print_followed_artists, get_followed_artists}, playlists::{debug_print_followed_playlists, get_followed_playlists}};

fn main() {
    let playlists_path = "./data/followed_playlists.json";
    let playlists = get_followed_playlists(playlists_path);
    debug_print_followed_playlists(&playlists);

    let artists_path = "./data/followed_artists.json";
    let artists = get_followed_artists(artists_path);
    debug_print_followed_artists(&artists);
}
