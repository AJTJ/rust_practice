https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters

# Generics

check the `generics` file

- rust performs _Monomorphization_ on generic code.

# Traits

- Like `interfaces` in other languages.

- Here is an example of the `ToString` trait that is conditionally implemented on any type that implements the `Display` trait

```
impl<T: Display> ToString for T {
    // --snip--
}
```

- rust does not allow us to call methods on a type that doesn't define the method.

## coherance
- We cannot implement **external** traits on **external** types.
  - This is known as the `orphan rune`
    - One or the other MUST be local to our crate.
    - This ensures other people's code cannot break our code and vice versa.

# Lifetimes

- lifetime annotations are used to inform the borrow checker of how long a functions values etc should live in the outer scope.

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

- normally (in pre1.0 Rust) this code would have required lifetime annotations:

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
- like so:

```fn first_word<'a>(s: &'a str) -> &'a str {```

- ... but the "lifetime elision rules" were created for the compiler to consider circumstances where you would not need to write lifetimes explicitly.
- 1. The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.
- 2. The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
- 3. The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

- `static` stores the reference right in the program's binary which is always available.