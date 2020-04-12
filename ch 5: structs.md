# Status
https://doc.rust-lang.org/book/ch05-01-defining-structs.html#defining-and-instantiating-structs

# Files
`structs`

# Struct facts
- The field of a struct can be different types.
- Each field is named.
- Each field has key: value pairs

# Tuple Structs
```
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```
- behave exactly like tuples except that each struct is inherently its own `type`, and cannot take another tuple struct as an argument.

# NOTES
- in order to pretty print a struct, you need to apply the `Debug` trait to it
```
#[derive(Debug)]
struct Name {
  ...
}
```