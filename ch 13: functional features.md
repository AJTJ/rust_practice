file:///Users/aaronjanke/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch13-01-closures.html

currently building `Cacher`
- memoization tool

# closures

```
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```
- This will display an error when the closure attempts to infor the type the second time. Closure definitions will have one concrete type inferred for each of their paramaters and for their return value.
