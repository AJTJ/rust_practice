
https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors

`propagating_errors`
`more_errors`

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

# using the ? to replace the match

- The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code."

```
f.read_to_string(&mut s)?
```
- The above will go through the `from` function in the `From` trait.

# usint the ? in the `main` function
- check `more_errors` file

# To panic! or Not to panic!
- `panic!` is unrecoverable
- generally better to return `Result`
- `panic!` can be good for tests

- below we still need to parse the IP address because the program doesn't know if it is actually an IP address.
- Here it is valid to use the `panic!` since it simply won't happen because we have more information than the compiler.
```
use std::net::IpAddr;
let home: IpAddr = "127.0.0.1".parse().unwrap();
```

- another situation to call `panic!` is when your program is in a "bad state"
- ..."such as when invalid values, contradictory values, or missing values are passed to your code"
- AND ALSO
- "The bad state is not something that’s expected to happen occasionally."
- "Your code after this point needs to rely on not being in this bad state."
- "There’s not a good way to encode this information in the types you use."