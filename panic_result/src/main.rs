use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // will cause a panic
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // will cause a panic because the 99th index does not exist
    // v[99];

    // RESULT

    // file does not actually exist
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    println!("it is f: {:?}", f);
}
