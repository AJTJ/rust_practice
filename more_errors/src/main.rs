use std::error::Error;
use std::fs::File;

// fn main() {
// will fail because the ? operator is only allowed in a function that returns a Result or an Option or something else that implements: std::ops::Try
// let f = File::open("hello.txt")?;
// }

fn main() -> Result<(), Box<dyn Error>> {
    File::open("hellos.txt")?;
    Ok(())
}
