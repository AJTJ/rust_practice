STATUS

- Chapter 3 "Data types"
  https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

# some cargo **commands**

`cargo new FILE_NAME` = start new cargo
`cargo check` = check your code
`cargo build` = produces an executable
\*\* `cargo run` = cargo build + run the executable
`cargo build --release` = compile with optimizations
`cargo run --release` = cargo build --release + run
`cargo doc --open` = build documentation in the browser

# statements

`use`

- generally used to bring librairies/crates into scope

# expressions

`match`

- made up of arms. It will check each arm until it finds a valid arm. Then it will end.
  - Arm: "arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm's pattern."

# variables

`let (with mut) = something`

"shadowing" = the ability to reuse a variable from before (using `let` again), instead of creating a new one.

`const` - value must be annotated, and is always immutable.
`const MAX_POINTS: u32 = 100_000;`

# keywords

`loop`

- "break" ends the loop.
- "continue" starts a new iteration of the loop.

`parse`

- returns "Ok" or "Err"

`_`

- the underscore is a catchall value

# data types

## scalar

- Represents a single value
  - integers `1`
  - floating-point-numbers `1.3`
  - Booleans `true`

#### integers

- a number without a fractional component.
- `u32` is "unsigned"
  - meaning, that it is always positive, since the negative "sign" would be required to make it negative.
- `i32` is "signed"
- Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

### floating point

- also `f32` and `f64` (the default) for floating point numbers

### boolean type

`let d: f32 = 4.0` to name the type

- boolean type
- `let t = true`
- `let f: bool = false`

### character type

    - characters is a primitive datatype
    - always single quotes `'d'`
    - string literals are double quotes

## Compound Types

### Tuple

- `let biggle: (i32, f32, i8) = (400, 6.2, 5);`
- `let (uu, yy, tt) = biggle;`
- `let four_hundred = biggle.0;`
- a way of grouping multiple types together.
- Fixed length: cannot grow or shrink.

### Array

- `let my_first_rust_array: [i32; 4] = [1, 2, 3, 4];`
- `println!("2nd element of array {}", my_first_rust_array[1]);`
- every element must have the same type
- fixed length (like tuple)
- data allocated to _stack (not heap)_
- shortcut to declare 10 4s
  - `let 10_4s = [4; 10];`

# Functions

## NOTES

- each parameter _must_ have its dype declared.
- it doesn't matter where functions are declared.
- functions have _scope_ like JS
- Function bodies are made up of statements and optionally ending in an expression
  - `STATEMENTS` do not return value
  - `EXPRESSIONS` return value
- we can declare reture _type_ with arrow
  - `fn name() -> i32 {}`

# Control flow (expressions and loops)

## IF

- `if something {} else if {}`
- the potential values MUST be the same type.

## LOOP

```
let result = loop {
    println!("current counter {}", counter);
    counter += 1;

    if counter == 10 {
        // return value AFTER break
        break counter;
    }
};
```

## WHILE

```
while ii < 8 {
  println!("the value is {}", a[ii]);
  ii += 1;
}
```

## FOR

```
for num in (1..4).rev() {
    println!("{}!", num)
}
```

# METHODS

- `.rev()`
- reverse the range

# Other tools

- `(1..4)`
  - includes all numbes between.
