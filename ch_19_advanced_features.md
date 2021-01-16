file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch19-04-advanced-types.html

## files
`unsafe_rust`
`advanced_traits`
`advanced_types`
`advanced_functions_closures`
`macros`

## `unsafe_rust`
- Allows you to:
  - Dereference a raw pointer
  - Call an unsafe function or method
  - Access or modify a mutable static variable
  - Implement an unsafe trait
  - Access fields of `union` S
- Unsafe does NOT
  - turn off the borrow checker
  - or other rust features

## `advanced_traits`
- Associated Types
- Default Generic Type Parameters & Operator Overloading

## `advanced_types`
- the "newtype" pattern/idiom (using tuple struct)
  - `struct Title(i64)`
- using newtype to hide internal implementations
- `Sized` trait

## `advanced_functions_closures`

## `macros`
- used for metaprogramming
  - i.e. writing code that writes code
- declarative macros
  - i.e. `macro_rules!`
- Custom `#[derive]` macros
- attribute-like macros
- function-like macros
- "metaprogramming"
  - writing code that writes code

## macros vs functions
- function signatures must declare the number and type of the parameters in the function.
- Macros can take a variable number of param.