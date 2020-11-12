file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch14-01-release-profiles.html

files
- `my_crate`
- root `Cargo.toml`

published crates
- https://crates.io/crates/speed_server (aka `my_crate`)
- 

# level of optimization
- see the root Cargo.toml for how to set profile optimizations.

# documentation comments
- uses three slashes
- useful for describing *how to use the crate*, as opposed to regular comments which are about *how the crate is implemented*
- see `my_crate`

# Workspaces
- The top-level `Cargo.lock` actually ensures that shared files are only downloaded once.
- To run a specific crate within a workspace, the `-p` flag chooses a specific crate for commands, such as:
  - `cargo run -p specific_crate_name`
  - `cargo test -p specific_crate_name`