# Files
`structs`
`rectangle`

# Struct facts
- The fields of a struct can be different types.
- Each field is named.
- Each field has key: value pairs

### Struct Declaration
```
struct Thing {
  key: dataType,
  key: dataType,
  ...
}
```

## Tuple Structs
```
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```
- behave exactly like tuples except that each struct is inherently its own `type`, and cannot take another tuple struct as an argument.

## Unit Struct
- simply an empty struct
```
struct StructName;
```

## Methods

`impl`
- This is for the "implementation block".

`self`
- Self is *ALWAYS* the first param in an `impl` block.

- Methods can take ownership of `self`, borrow `&self` immutably, or borrow `&mut self` mutably, just as they can any other parameter. The first case (just taking ownership of `self` is rare, and would be used to change the identity of the struct).

- One of the purposes of Methods on structs (rather than floating functions) is that it groups all associated methods together.

## Associated Functions
- this is created when a method does NOT call `self`
- often used for constructors that will return a new instance of the struct

`String::from`
- is an *associated function*



# NOTES
- in order to pretty print a struct, you need to apply the `Debug` trait to it
```
#[derive(Debug)]
struct Name {
  ...
}
```