use reqwest::{self};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, env};
use crate::api_structs::artists::FollowedArtists;
use base64::{engine::general_purpose, Engine};

// #[derive(Debug)]
// pub struct Token {
//     pub access_token: String,
//     pub token_type: String,
//     pub expires: u32,
// }

pub struct SpotifyAPI { 
    // email: String,
    // username: String,
    client_id: String,
    client_secret: String,
    token: String,
    refresh_token: String,
    http_client: reqwest::blocking::Client,
}

impl SpotifyAPI {

    pub fn new() -> Self {
        // let email = env::var("EMAIL").unwrap();
        // let username = env::var("USERNAME").unwrap();
        let client_id = env::var("CLIENT_ID").unwrap();
        let client_secret = env::var("CLIENT_SECRET").unwrap();

        let token = env::var("TOKEN").unwrap();
        let refresh_token = env::var("REFRESH_TOKEN").unwrap();

        let http_client = reqwest::blocking::Client::new();

        Self {
            // email,
            // username,
            client_id,
            client_secret,
            token,
            refresh_token,
            http_client,
        }
    }

    // TODO implement generic function where you just need to pass a struct, the request url and a path and it
    // automatically does the request and data serialization to json file
    pub fn update_data<T>(&self, url: &str, path: &str) where T: Serialize + DeserializeOwned {
        let token_header = format!("Bearer {}", self.token);

        let res = self.http_client
            .get(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", token_header)
            .send();

        let Ok(res) = res else {
            eprintln!("could not receive response");
            // TODO add a Result return type
            return;
        };

        if !res.status().is_success() {
            eprintln!("unsuccessful request, status: {}", res.status());
            return;
        }

        let body = res.json::<T>();

        let Ok(data) = body else {
            eprintln!("coult not deserialize json body");
            return;
        };

        save_to_file(&data, path);

    }

    // "https://api.spotify.com/v1/me/following?type=artist"
    pub fn update_followed_artists(&self) {

        /*let token_header = format!("Bearer {}", self.token);

        let res = self.http_client
            .get("https://api.spotify.com/v1/me/following?type=artist")  
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", token_header)
            .send();

        let res = match res {
            Ok(r) => r,
            Err(_) => {
                eprintln!("could not receive response");
                return;
            },
        };

        if !res.status().is_success() {
            eprintln!("unsuccessful request, status: {}", res.status());
            return;
        }

        let body = res.json::<FollowedArtists>();
        let followed_artists = match body {
            Ok(a) => a,
            Err(_) => {
                eprintln!("could not deserialize json body");
                return;
            }
        };*/

        /*let file_path = "./data/followed_artists.json";
        let str = serde_json::to_string_pretty(&followed_artists).unwrap();
        std::fs::write(file_path, str).unwrap();*/

        //save_to_file(&followed_artists, "./data/followed_artists.json");

        self.update_data::<FollowedArtists>("https://api.spotify.com/v1/me/following?type=artist", "./data/followed_artists.json");
    }

    // TODO add a way to call refresh_token only if the previous one has expired
    // for example save the timestamp in the .env file and read it before requesting a refresh
    pub fn refresh_token(&mut self) {
        let auth_value = format!("{}:{}", self.client_id, self.client_secret); 
        let encoded_auth = general_purpose::STANDARD.encode(auth_value);

        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "refresh_token");
        form_data.insert("refresh_token", &self.refresh_token);

        let res = self.http_client
            .post("https://accounts.spotify.com/api/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Basic {}", encoded_auth))
            .form(&form_data)
            .send();

        let Ok(res) = res else {
            eprintln!("could not send request");
            return;
        };

        if !res.status().is_success() {
            eprintln!("unsuccessful request, status: {}", res.status());
            return;
        }

        let res_json = res.json::<RefreshTokenResponse>();

        let Ok(res_json) = res_json else {
            eprintln!("could not deserialize json body");
            return;
        };

        eprintln!("previous access token: {}", self.token);

        self.token = res_json.access_token;
        env::set_var("TOKEN", &self.token);
        eprintln!("new access token updated and saved to .env file");

        match res_json.refresh_token {
            Some(token) => {
                self.refresh_token = token;
                env::set_var("REFRESH_TOKEN", &self.refresh_token);
            },
            None => eprintln!("no refresh token received, skipping .env update"),
        };

        eprintln!("access token: {}", self.token);
        eprintln!("refresh token: {}", self.refresh_token);

    }

}

#[derive(Debug, Deserialize)]
struct RefreshTokenResponse {
    access_token: String,
    #[serde(alias = "token_type")] _token_type: String,
    #[serde(alias = "expires_in")] _expires_in: u32,
    refresh_token: Option<String>, 
    #[serde(alias = "scope")] _scope: String, 
}

// fn save_to_file<T: serde::Serialize>(data: T, path: &str) {
//     let str = serde_json::to_string_pretty(&data).unwrap();
//     std::fs::write(path, str).unwrap();
// }

fn save_to_file<T>(data: T, path: &str) where T: Serialize {
    let str = serde_json::to_string_pretty(&data).unwrap();
    std::fs::write(path, str).unwrap();
}
