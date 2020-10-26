file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch13-03-improving-our-io-project.html#using-the-returned-iterator-directly

currently building `Cacher`
- memoization tool

files
- `adder`
- `closures_2`
- `shoe_size`
- `iterator_trait`

# closures
## notes
- similar to functions
- does NOT require type annotations if it is being called, since the types can be inferred from their usage.
- closures are able to capture values from their environment (unlike functions)
- `let example_closure = |x| x;`
```
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

## types of closures
`FnOnce`
- consumes the captured vars. Taking ownership. Only does it once i.e. can only be called once.
`FnMut`
- borrows, but can mutate
`Fn`
- borrows values immutably

## the `move` keyword
- forces the closure to take ownership of the value
- allows the closure to outlive the life of the captured value

## closure errors

- example of closure error
```
let s = example_closure(String::from("hello"));
let n = example_closure(5);
```
- This will display an error when the closure attempts to infor the type the second time. Closure definitions will have one concrete type inferred for each of their paramaters and for their return value.

# Iterators

## Notes
- Iterators are lazy, no effect until you call methods that consume the iterator
- consuming an iter, such as with a `for` does *not* require it be `mut` since it takes ownership and makes it `mut` behind the scenes.

## Next
- All iterators have the `next` method
- calling `next` on an iter requires that the iter be `mut`

## Creating iterators
- calling `iter`, `into_iter`, and `iter_mut` on a vector are all valuable ways.
- a custom iterator can be created using the `Iterator` trait.
  - see `iterator_trait` file