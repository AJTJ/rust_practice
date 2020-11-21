
# files
`ref_borrowing`
`slice_type`

# Stack

- literallly a stack of data that is easy for the CPU to push and pop, since it is all in a stack. Data on the stack _must_ have a known, fixed size.

# Heap

- ...all other data must go on the heap.
- Pointers are created and placed on the stack that point to the heap.

# String Literals

- `let = "Jeremy"`
  - This is a hard-coded string literal and can't be mutated.
- `let mut s = String::from("hello");`
  - the `::` namespaces the from function.
    - will be discussed Ch5...
  - this _can_ be mutated.

# Ownership

## General programming nots on copying

- **Shallow copying** will mean that the pointer is _shared_

- **Deep copying** actually copies the heap data as well.

## How rust works

- Basically, when a variable goes out of scope Rust calls the `drop` function and cleans up that variables heap memory.

- Anything only stored on the _stack_ (anything with known, fixed value at compile time) has no heap data, therefore will never need to have its memory allocation managed.

- `.clone()` can be used to make a full copy of a value.

- _To be seen_ the `Copy` trait is part of certain data types and will _not_ cause older variables to go out of scope.

## In functions

- functions take ownership, unless the data they are taking ownership of has the `Copy` trait.

# References

## How to refer

- The `&` (ampersand) creates a reference, thus using the pointer of previous data.
- `let len = calculate_length(&s1)`

- Dereferencing uses the `*`
  - TO BE SEEN: chapter 15...

## Mutating references

- Steps:

  - The var must be `mut`
    - `let mut mut_s = String::from("memes");`
  - The function call must have a mutable ref
    - `change(&mut mut_s);`
  - The function must call mutable data
    - `fn change(some_string: &mut String) {...`

- Restrictions
  - You can only have one mutable reference to a piece of data _in the same scope._
  - Restiction exists so as to have _very_ controlled mutation.
  - You cannot have a mutable ref if an immutable ref already exists.

### Dangling References

```
fn dangle() -> String {
    let s = String::from("hello");
    &s
}
```
- In the above example, we are trying to return a reference to `s`, but it will have gone out of scope and it is dropped.
- Just returning `s` directly solves this, since it moves ownership out of the function.

### The Slice Type
```
let slice = &var[1..5];
```
- Is a type of reference
  - Does not have ownership

## Notes

- References are `immutable` by default.
- References must always be valid, i.e. no `dangling ref`
### The `.iter()` method
- returns each element in a collection

### The `enumerate` method
  - wraps the result of `iter` and returns each element as part of a tuple instead.
  - the first element of this tuple is the index.

Basically in `slice_type` the `iter().enumerate()` calculates the index for us...
```
for (i, value) in foo.iter().enumerate() {

}
```
### byte literal syntax
`b''`
- represents the syntax for searching for a byle literal and NOT the string variant.