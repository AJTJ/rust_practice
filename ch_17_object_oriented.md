file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch17-03-oo-design-patterns.html#trade-offs-of-the-state-pattern

## files
- `gui`
- `blog_post`

## What makes OOP
- `struct`s and `enum`s have `impl` blocks that create methods. Not called objects, but they provide the same functionality using the `pub` keyword.
- Rust does not have `inheritance`, but uses trait objects instead.

## not objects
- with `struct`s and `enum`s, the data in the `struct` fields and the behavior in the `impl` blocks are separated, whereas in other languages the data and the behavior are combined into one concept often labeled an "object".

## Trait Object Notes
- Like traditional objects, they combine data and behavior.
- Different from traditional "objects" because we can't add data to a trait object.
- The trait object purpose: *To allow abstraction across common behavior*

## Trait Objects
- `Box<dyn Trait>`
  - This is a stand-in for any type inside a Box that implements the `Trait`

## Object safety and Trait Objects
- when all the methods defined in the trait have:
  - The return type isn't `self`
  - There are no generic type parameters.

## OO Notes
- "polymorphism"
  - code that can work with data of multiple types.
  - rust uses generics instead.
- "static dispatch"
  - this is what happens with monomorphization where the compiler knows what method you're calling at compile time.
- "dynamic dispatch"
  - where the compiler emits code that at runtime will figure out what method to call.
  - this has a runtime cost.

## OO Design Patterns
- "state pattern"
  - an object-oriented design pattern.
  - The crux being that a value has internal state.

## `blog_post`


