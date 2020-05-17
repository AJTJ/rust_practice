https://doc.rust-lang.org/book/ch06-02-match.html

## Enums
- Enums allow you to define a type by enumerating its possible variants.
- the variants can also have types
- enums can also take `impl` just like structs


```
enum ENUMTITLE {
  e1,
  e2(value(s)),
  e3(u8, u8, String, i32)
}

let thing = ENUMTITLE::e3(5, 5, String::from("apples"), 75);
```

### Option Enum
- another enum defined by the standard library.
- encodes the scenario where a value could be something or it could be nothing.
- rust does NOT include a `null` value, thus this exists.
```
enum Option<T> {
    Some(T),
    None,
}
```
