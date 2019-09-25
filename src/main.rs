extern crate rspotify;

mod lib;

use lib::*;
use std::process;
use std::collections::VecDeque;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

/// your client-id from spotify and the redirect-url after the OAuth
const CLIENT_ID: &'static str = "your-client-id";
const CLIENT_SECRET: &'static str = "your-secret-client-id";
const REDIRECT_URI: &'static str = "http://localhost:8888/callback";


fn main() {

    // OAuth
    let mut oauth = SpotifyOAuth::default()
        .client_id(CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .redirect_uri(REDIRECT_URI)
        .scope("playlist-read-private, playlist-read-collaborative, playlist-modify-public, playlist-modify-private")
        .build();

    match get_token(&mut oauth) {
        Some(token_info) => {
            println!("\n");
            let client_credential = SpotifyClientCredentials::default()
                .client_id(CLIENT_ID)
                .client_secret(CLIENT_SECRET)
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            // current users id
            let me_id = spotify.me().unwrap().id;
            
            // ask user how many playlist he want to merge
            let playlist_list = get_playlists(get_playlist_count());
            
            // Vector with track-ids
            let mut new_pl_tracks = VecDeque::new();
            for p in playlist_list {
                new_pl_tracks.push_back(get_playlist_tracks(&spotify, p.as_str()));
            }


            // merge to one vector
            let tracks_add = merge(&mut new_pl_tracks, get_merge_pattern());

            // create new playlist
            let npl_name = get_new_playlist_name();
            let npl_id = spotify.user_playlist_create(&me_id, npl_name.as_str(), false, None)
                .unwrap()
                .id;

            for ta in tracks_add {
                let result = spotify.user_playlist_add_tracks(&me_id, &npl_id, &ta, None);
                match result
                {
                    Ok(_) => (),
                    Err(err) => {
                        println!("ERROR: {}", err);
                        process::exit(1);
                    },
                }
            }
            println!("Succesfully merged both Playlists and saved in: {}", npl_name);
        }
        None => println!("auth failed"),
    };
}
