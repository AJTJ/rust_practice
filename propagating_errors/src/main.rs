use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    // The verbose way to propagate the results
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // using the ? operator
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // using the ? operator
    f.read_to_string(&mut s)?;
    Ok(s)

    // The verbose way to propagate the results
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

fn main() {
    let result = read_username_from_file();
    println!("the result: {:?}", result)
}
