file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch18-03-pattern-syntax.html#destructuring-enums

## files
- `conditonal_expressions`

## `conditional_expressions`
- `if let Ok(age) = age`
  - an example of shadow variables in rust.
- `while let`
  - will run as long as the pattern continues to match.

## refutable vs irrefutable patterns
- `let x = 5` is irrefutable
- `Some(x)` is refutable
  - for example in `if let Some(x) = a_value` if the value of `a_value` is `None` rather than Some, the `Some(x)` pattern will not match.
- `let` statements and `for` loops only accept irrefutable patterns because the program cannot do anything meaningful when values don't match.
- `if let` and `while let` allow refutable patterns, 

## General Notes
- pattern matching is everything in rust.