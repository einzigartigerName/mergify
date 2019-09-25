# mergify
Command line tool to merge mutliple (up to ten) Spotify playlists. You can choose between three merge pattern:
* Appending the tracks
* Alternating between each playlist
* Random merge

### Build
To correctly use this crate you need a spotify developer account and create a new project. Then enter your `client-id` and `secret-client-id` in `main.rs`. Otherwise the program is not going to work.\
Example:
```Rust
const CLIENT_ID: &'static str = "fbf4964d9dfd419d939068372f4ee";
const CLIENT_SECRET: &'static str = "fa297e5629724ffb80da296b18214";
```

To build, you need Rust v1.37+
```bash
cargo build --release
```
