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