extern crate rspotify;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

const CLIENT_ID: &'static str = "your-client-id";
const CLIENT_SECRET: &'static str = "your-secret-client-id";
const REDIRECT_URI: &'static str = "http://localhost:8888/callback";

enum MergePattern
{
    Append,
    Alternate,
    Random
}

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

            // current users id
            let me_id = spotify.me().unwrap().id;

            // get playlist tracks
            // first playlist
            let mut first_pl = String::from("first-playlist-url");

            // second playlist
            let mut second_pl = String::from("second-playlist-url");

            // create new playlist
            let npl_id = spotify.user_playlist_create(&me_id, "new-playlist-name", false, None)
                .unwrap()
                .id;
            
            // Vector with track-ids
            let mut new_pl_tracks = vec![];
            new_pl_tracks.push(get_all_tracks(&spotify, &me_id, Some(&mut first_pl)));
            new_pl_tracks.push(get_all_tracks(&spotify, &me_id, Some(&mut second_pl)));

            // merge to one vector
            let tracks_add = merge(new_pl_tracks, MergePattern::Append);

            // add all tracks to new playlist
            let result = spotify.user_playlist_add_tracks(&me_id, &npl_id, &tracks_add, None);
            println!("{:?}", result);
        }
        None => println!("auth failed"),
    };
}

// return all track-uris in a single Playlist as Vec<String>
fn get_all_tracks(
    spotify: &Spotify,
    user_id: &str,
    playlist: Option<&mut str>
) -> Vec<String> 
{
    match spotify.user_playlist(user_id, playlist, None, None)
    {
        Ok(result) => {
            let mut out = vec![];
            let tracks = result.tracks.items;
            for t in tracks
            {
                out.push(t.track.uri);
            }
            return out;
        },
        Err(err) => panic!("Could not read Playlist-Informations:\n{}", err),
    }
}


/// Merge a vector of Vec<String> using predefined pattern into one
fn merge(
    playlists: Vec<Vec<String>>,
    pattern: MergePattern
) -> Vec<String>
{
    match pattern
    {
        MergePattern::Append => {
            // return-vector
            let mut out = vec![];
            // add all tracks to return-vector
            for v in playlists{
                // out.append(&mut v);
                out = out.iter().cloned().chain(v.iter().cloned()).collect();
            }

            return out;
        },
        MergePattern::Alternate => {
            println!("Not yet implemented");
            return vec![];
        },
        MergePattern::Random => {
            println!("OK COOL");
            return vec![];
        },
    }
}