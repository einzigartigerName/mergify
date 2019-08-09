extern crate rspotify;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

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

            let client_credential = SpotifyClientCredentials::default()
                .client_id(CLIENT_ID)
                .client_secret(CLIENT_SECRET)
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            let me_id = spotify.me().unwrap().id;
            // get playlist tracks
            let mut playlist_id = String::from("playlist-id-to-copy");
            let playlist_tracks = spotify.user_playlist(&me_id, Some(&mut playlist_id), None, None)
                .unwrap()
                .tracks
                .items;

            // create new playlist
            let new_pl = spotify.user_playlist_create(&me_id, "new-playlist-name", false, None).unwrap();
            let npl_id = new_pl.id;

            let mut tracks_add = vec![];

            for t in playlist_tracks
            {
                tracks_add.push(t.track.uri);
            }

            let result = spotify.user_playlist_add_tracks(&me_id, &npl_id, &tracks_add, None);
            println!("{:?}", result);
        }
        None => println!("auth failed"),
    };
}