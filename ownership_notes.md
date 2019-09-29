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
  - this _can_ be mutated.

# Ownership

- Basically, when a variable goes out of scope Rust calls the `drop` function and cleans up that variables heap memory.
- `.clone()` can be used to make a full copy of a value.
- **Shallow copying** will mean that the pointer is _shared_
- **Deep copying** actually copies the heap data as well.
- Anything only stored on the _stack_ (anything with known, fixed value at compile time) has no heap data, therefore will never need to have its memory allocation managed.
- _To be seen_ the `Copy` trait is part of certain data types and will _not_ cause older variables to go out of scope.
