file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch16-01-threads.html#using-move-closures-with-threads

# threads and runtime
- 1:1
  - one OS thread per one language thread
- M:N aka green threads
  - M green threads to N OS threads
- Every non-assembly language has some amount of runtime code.
- thus "no runtime" often means "small runtime"
- Rust only provides 1:1 to maintain being able to call into C

# `spawn`
## Notes
- the spawned thread will stop when the main thread ends.
- `thread::sleep` forces a thread to stop executing for a short duration, allowing different threads to run.

## `JoinHandle` 
- this is the return type of `thread::spawn`
- it is an "owned value"