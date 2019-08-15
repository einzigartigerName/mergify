# mergify

Befor you build, you need to enter your `client-id` and `secret-client-id` in `main.rs`. Otherwise the program is not going to work.
Example:
```Rust
const CLIENT_ID: &'static str = "fbf4964d9dfd419d939068372f4ee";
const CLIENT_SECRET: &'static str = "fa297e5629724ffb80da296b18214";
```

To build, you need Rust v1.37+
```bash
cargo build --release
```
