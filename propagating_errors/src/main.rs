use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    // The VERBOSE way to propagate the results
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    // The LESS VERBOSE way.
    // using the ? operator will return and exit the function early with the Err.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // The EVEN LESS VERBOSE way
    // File::open("hello.txt")?.read_to_string(&mut s)?;

    // The EVEN EVEN LESS VERBOSE way
    // ... (this) function opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it...
    // fs::read_to_string("hello.txt")
}

fn main() {
    let result = read_username_from_file();
    println!("the result: {:?}", result)
}
