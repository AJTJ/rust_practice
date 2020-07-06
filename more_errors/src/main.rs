use std::fs::File;

fn main() {
    // will fail because the ? operator is only allowed in a function that returns a Result or an Option.
    let f = File::open("hello.txt")?;
}
