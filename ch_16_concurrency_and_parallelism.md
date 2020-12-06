file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch16-03-shared-state.html

## files
- `spawn`
- `spawn_values`
- `message_passing`

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

## Using the `move` keyword to move values
- in `spawn_values` it is essential to use the `move` keyword to move the value into the new closure since it is impossible for rust to know whether or not the new thread will outlive the main thread.

## Message passing
- *“Do not communicate by sharing memory; instead, share memory by communicating.”*
- Two halves to every channel: a `transmitter` and a `receiver`
- it will be `closed` if either of those two are dropped.

## `message_passing`
- `mpsc`: multiple producer, single consumer
- `mpsc::channel` returns a `tuple`
- `tx` and `rx` are traditionally used for transmitter and receiver.
- the receiving end (`rx`) has two useful methods
  - `recv` and `try_recv`
    - `recv` blocks the main thread and waits, and then returns a `Result<T,E>`
    - `try_recv` does *not* block, but returns a `Result<T,E>` immediately
      - good for trying every so often (and allowing the main thread to do other work)