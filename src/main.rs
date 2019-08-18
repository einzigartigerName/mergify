// mod random_vector;

#[macro_use] extern crate lazy_static;
extern crate rspotify;
extern crate regex;
extern crate rand;

use rand::seq::SliceRandom;
use std::io;
use std::collections::VecDeque;
use regex::Regex;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

/// your client-id from spotify and the redirect-url after the OAuth
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
            println!("Playlist Number One: ");
            let first_pl = parse_playlist(get_playlist_link());

            println!("Playlist Number Two: ");
            let second_pl = parse_playlist(get_playlist_link());
            
            // Vector with track-ids
            let mut new_pl_tracks = VecDeque::new();
            new_pl_tracks.push_back(get_all_tracks(&spotify, first_pl.as_str()));
            new_pl_tracks.push_back(get_all_tracks(&spotify, second_pl.as_str()));

            // merge to one vector
            let tracks_add = merge(&mut new_pl_tracks, get_merge_pattern());

            // create new playlist
            let npl_name = "new-playlist-name";
            let npl_id = spotify.user_playlist_create(&me_id, npl_name, false, None)
                .unwrap()
                .id;

            // add all tracks to new playlist
            let result = spotify.user_playlist_add_tracks(&me_id, &npl_id, &tracks_add, None);
            match result
            {
                Ok(_) => println!("Succesfully merged both Playlists and saved in: {}", npl_name),
                Err(err) => println!("ERROR: {}", err),
            }

        }
        None => println!("auth failed"),
    };
}


/// Read Playlist-Link from stdin
fn get_playlist_link() -> String
{
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer)
    {
        Ok(_) => return buffer,
        Err(err) => panic!("ERROR: {}", err),
    }
}


/// get playlist uri from link
fn parse_playlist(
    input: String
) -> String
{   
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-z]{5}://[a-z]{4}\.[a-z]{7}\.[a-z]{3}/[a-z]{8}/(.*)")
            .unwrap();
    }
    
    match RE.captures(input.as_str()){
        Some(caps) => return String::from(&caps[1]),
        None => panic!("Not a valid link!"),
    }
}


/// ask the user for a merge-pattern
fn get_merge_pattern() -> MergePattern
{
    println!("Select your pattern:");
    println!("\t1: Append");
    println!("\t2: Alternate");
    println!("\t3: Random");

    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer)
    {
        Ok(_) => {
            match buffer.trim().parse::<u8>()
            {
                Ok(1) => return MergePattern::Append,
                Ok(2) => return MergePattern::Alternate,
                Ok(3) => return MergePattern::Random,
                _ => {
                    println!("Please choose valid option:");
                    return get_merge_pattern();
                },
            }
        },
        Err(err) => panic!("ERROR: {}", err),
    }

}

/// return all track-uris in a single Playlist as Vec<String>
fn get_all_tracks(
    spotify: &Spotify,
    playlist: &str
) -> Vec<String> 
{
    match spotify.playlist(playlist, None, None)
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
    playlists: &mut VecDeque<Vec<String>>,
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
                out = out.iter().cloned().chain(v.iter().cloned()).collect();
            }

            return out;
        },

        MergePattern::Alternate => {
            let mut out = vec![];

            while ! playlists.is_empty() {
                let mut front = playlists.pop_front().unwrap();
                if front.is_empty() {
                    continue;
                }   else{
                    out.push(front.pop().unwrap());
                    playlists.push_back(front);
                }
            }
            out.reverse();
            return out;
        },

        MergePattern::Random => {
            // let mut out = random_vector::RandomVector::new();
            // for v in playlists{
            //     for t in v{
            //         out.push(t);
            //     }
            // }
            // return out.vector();
            let mut rnd = rand::thread_rng();
            let mut out = merge(playlists, MergePattern::Append);
            out.shuffle(&mut rnd);
            return out;
        },
    }
}