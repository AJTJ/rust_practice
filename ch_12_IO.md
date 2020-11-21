file:///Users/aaronjanke/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch12-04-testing-the-librarys-functionality.html

NOTE: check `minigrep` cargo

# General process for splitting based on separation of concerns

- Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
- As long as your command line parsing logic is small, it can remain in main.rs.
- When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.


The responsibilities that remain in the main function after this process should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error

Basically, main.rs handles running the program and lib.rs handles the logic of the task at hand.

# Notes
- Using primitive values when a complex type would be more appropriate is an anti-pattern known as **primitive obsession**.
- `&'static str` is the type of string literals