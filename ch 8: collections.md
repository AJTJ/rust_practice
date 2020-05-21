https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector

## Vectors
- allow you to store more than one value in a data structure that puts all the values next to each other in memory.
- Can only store values of the same type.
- Useful for lists of items.
```
let v: Vec<i32> = Vec::new();
```
- `vec!` is a macro for a vector
```
let v2 = vec![1, 2, 3];
```

## The `get` method
- returns us an `Option<&T>`
```
match v1.get(2) {
    Some(third) => println!("The third ele is {}", third),
    None => println!("The third ain't nuffin"),
};
```

## iterating over Vectors
```
for i in &veccy {
    println!("the ele is {}", i);
}
for i in &mut veccy2 {
    println!("the ele is {:?}", *i += 50);
}
```
## Using an `enum` within a `vec`
- This allows the use of different types within a vec, since each enum type is the same enum type.
```
enum InputValue {
    Int(i32),
    Text(String),
    Float(f64),
};

let char_input = vec![
    InputValue::Float(33.33),
    InputValue::Text(String::from("Some text")),
    InputValue::Int(66),
];
```

## The `String` and `str` types
- `str` is the string slice, usually seen in its borrowed form `&str`.
- the `String` type is a growable, mutable, owned, UTF-8 encoded string type.
- There is also `OsString`, `OsStr`, `CString` and `CStr`

SEE the `strings` file for all sorts of tools and methods.