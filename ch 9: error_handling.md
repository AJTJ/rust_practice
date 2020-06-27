
https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors

# `panic!`
- for unrecoverable errors
- `RUST_BACKTRACE=1 cargo run`
  - this will display a trace of what happened
  - top: code that my code called.
  - my code
  - bottm: code that called my code


# Recoverable errors with `Result`
```
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```
- In the above `T` and `E` are generics