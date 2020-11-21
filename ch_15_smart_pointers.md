file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch15-01-box.html

# Smart Pointers

## Notes
- Both `String` and `Vec<T>` are smart pointers. Because they own some memory and allow you to manipulate it.
- Smart pointers are usually implemented using `structs` but they also implement the `Deref` and `Drop` traits.