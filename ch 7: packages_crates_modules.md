https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html

# Packages
- A cargo feature that lets you build, test and share crates

### Package Rules
- Must contain 0 or 1 library crates, no more. But may contain one or more binary crates.
- Contains a `Cargo.toml` file that describes how to build the included crates.
- `src/main.rs` is the crate root of a binary crate, with the same name as the package.
- if the package dir contains `src/lib.rs` the package contains a library crate with the same name as the package (and `src/lib.rs` is its crate root)
- Cargo passes the crate root files to `rustrc` to build the library or binary.
- A package can have multiple binary crates by placing files in the `src/bin` dir; each file will be a separate binary crate.


# Crates
- A tree of modules that produces a library or executable.
- A crate is a BINARY or a LIBRARY
- Will group related functionality together in a scope.

# Modules and `use`
- Let you control the organization, scope and privacy of paths
- Let us organize code within a crate into groups for readability and easy reuse.

# Paths
- A way of naming an item, such as a struct, function, or module.

# NOTES
- see the `restaurant` file to see examples of modules and path creation/designation.