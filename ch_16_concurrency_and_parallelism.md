file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch16-04-extensible-concurrency-sync-and-send.html

## files
- `spawn`
- `spawn_values`
- `message_passing`
- `mutex`
- `multiple_mutex`

# threads and runtime
- 1:1
  - one OS thread per one language thread
- M:N aka green threads
  - M green threads to N OS threads
- Every non-assembly language has some amount of runtime code.
- thus "no runtime" often means "small runtime"
- Rust only provides 1:1 to maintain being able to call into C

## `spawn`
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

## `mutex` usage
- *mutual exclusion*
  - mutex only allows one thread to access some data at any given time
- challenging because:
  - you must attempt to acquire the lock before using the data
  - you musst unlock the data afterwards

## `multiple_mutex`
- the use of `Arc<T>`
  - `Rc<T>` does *NOT* use concurrency primitives, but is faster.

## NOTES
- `Mutex<T>` provides interior mutability, as the `Cell` family does.
  - `RefCell<T>` allows us to mutate contents of `Rc<T>`
  - `Mutex<T>` allows us to mutate the contents of `Arc<T>`
- `Mutex<T>` can create **deadlocks**

## deadlock
- `deadlock`
  - when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.
- Create a piece of software that creates a deadlock.
- The docs for `Mutex<T>` and `MutexGuard` have useful information for avoiding a deadlock.

## `Send` marker trait
- indicates that the type implementing Send can be transferred between threads.
- `Rc<T>` does not have it, since two clones could update the ref count at the same time.

## `Sync`
- this marker trait indicates that the type is safe to be referenced from multiple threads.