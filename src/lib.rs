extern crate rspotify;
extern crate regex;
extern crate rand;

use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::collections::VecDeque;
use regex::Regex;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::{request_token, process_token};
use rspotify::spotify::oauth2::{SpotifyOAuth, TokenInfo};


pub enum MergePattern
{
    Append,
    Alternate,
    Random
}

/// Asks user how may playlist he wants to merge
pub fn get_playlist_count() -> u8
{
    println!("How many playlist do you want to merge (max 10):");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer)
    {
        Ok(_) => {
            let count = buffer.trim().parse::<u8>().unwrap();
            if count > 0 && count <= 10 {
                return count;
            } else {
                return get_playlist_count();
            }
        }
        Err(err) => panic!("ERROR: {}", err),
    }
}

/// collects playlist-links from stdin
pub fn get_playlists(
    count: u8
) -> Vec<String>
{
    let mut out = Vec::new();
    for i in 0..count {
        println!("Playlist [{}]:", i + 1 );
        out.push(parse_link(get_link()));
    }
    out
}

/// ask the user for a merge-pattern
pub fn get_merge_pattern() -> MergePattern
{
    print!("Select your pattern:");
    print!("\t[1]: Append");
    print!("\t[2]: Alternate");
    println!("\t[3]: Random");

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

/// get name for new playlist from user, if nothing entered return default
pub fn get_new_playlist_name() -> String {
    let mut buffer = String::new();

    print!("Name of the new playlist: ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut buffer)
    {
        Ok(_) => {
            let out = buffer.trim();
            if out.is_empty() {
                return String::from("new-playlist");
            } else {
                return String::from(out);
            }
        },
        Err(err) => panic!("ERROR: {}", err),
    }
}

/// return all track-uris in a single Playlist as Vec<String>
pub fn get_playlist_tracks(
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
pub fn merge(
    playlists: &mut VecDeque<Vec<String>>,
    pattern: MergePattern
) -> Vec<Vec<String>>
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

            return prepare_track_list_for_adding(&mut out)
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
            return prepare_track_list_for_adding(&mut out);
        },

        MergePattern::Random => {
            let mut rnd = rand::thread_rng();
            let tmp = merge(playlists, MergePattern::Append);
            
            let mut out = vec![];
            for mut v in tmp{
                v.shuffle(&mut rnd);
                out.push(v);
            }

            return out;
        },
    }
}

// --------------------------------------------------------------------
//                  Helping functions
// --------------------------------------------------------------------

// Read Playlist-Link from stdin
fn get_link() -> String
{
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer)
    {
        Ok(_) => return buffer,
        Err(err) => panic!("ERROR: {}", err),
    }
}

// get playlist uri from link
fn parse_link(
    input: String
) -> String
{   
    let re: Regex = Regex::new(r"[a-z]{5}://[a-z]{4}\.[a-z]{7}\.[a-z]{3}/[a-z]{8}/(.*)")
        .unwrap();
        
    match re.captures(input.as_str()){
        Some(caps) => return String::from(&caps[1]),
        None => panic!("Not a valid link!"),
    }
}

fn prepare_track_list_for_adding(
    vec: &mut Vec<String>
) -> Vec<Vec<String>>
{
    let mut out = vec![];
    
    while vec.len() > 100 {
        let l = vec.split_off(100);
        out.push(l);
    }

    out.push(vec.to_vec());
    out.reverse();
    return out;
}

// --------------------------------------------------------------------
//                  Authorization
// --------------------------------------------------------------------

/// get tokenInfo by Authorization
pub fn get_token(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    request_token(spotify_oauth);
    println!("Enter the URL you were redirected to: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => process_token(spotify_oauth, &mut input),
        Err(_) => None,
    }
}