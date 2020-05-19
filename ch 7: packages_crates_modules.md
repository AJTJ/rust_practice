

# Packages
- A cargo feature that lets you build, test and share crates

### Package Rules
- Must contain 0 or 1 library crates, no more. But may contain one or more binary crates.
- Contains a `Cargo.toml` file that describes how to build the included crates.
- `src/main.rs` is the crate root of a binary crate, with the same name as the package.
- if the package dir contains `src/lib.rs` the package contains a library crate with the same name as the package (and `src/lib.rs` is its crate root)
- Cargo passes the crate root files to `rustrc` to build the library or binary.
- A package can have multiple binary crates by placing files in the `src/bin` dir; each file will be a separate binary crate.

### NOTES
- see the `restaurant` file to see examples of modules and path creation/designation.


### Crates
- A tree of modules that produces a library or executable.
- A crate is a BINARY or a LIBRARY
- Will group related functionality together in a scope.

### Modules and `use`
- Let you control the organization, scope and privacy of paths
- Let us organize code within a crate into groups for readability and easy reuse.

### Paths
- A way of naming an item, such as a struct, function, or module.

### `use` keyword
- using `use` to bring `hosting` into scope
- also using the `crate` keyword, to target the current crate.
- it is idiomatic to call the FUNCTION's parent, rather than the child, so as to specify that `add_to_waitlist` isn't locally defined.
- it is the opposite with `structs, enums and other items` with `use`; i.e. you call them direction
```
use crate::front_of_house::hosting;
...
hosting::add_to_waitlist();

...

use std::collections::HashMap;

let mut map = HashMap::new();
```
- as is stated above, the `std` (standard library) is a crate, but does not require any alteration to `Cargo.toml` to access.
- `use` is also how we import external packages.
```
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io}; //same as the two lines above

use std::io::{self, Write}; //brings `std::io` and `std::io::Write` into scope.

use std::collections::*; //glob operator brings all public items into scope
```

### the `as` keyword
```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### `pub use`
- this is re-exporting