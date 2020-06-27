use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // will cause a panic
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // will cause a panic because the 99th index does not exist
    // v[99];

    // RESULT

    // file does not actually exist therefore it errors
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

    // An example of the same logic but using closures instead of match
    let e = File::open("dankMemes.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("dankMemes.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // since the file does not exist, unwrap will call the panic! macro
    // let d = File::open("notExist.txt").unwrap();
    // this will also panic, but it will return the custom error message
    // let d = File::open("notExist.txt").expect("file no existy");

    println!(
        "the 'f' file is: 
    {:?}",
        f
    );
    println!(
        "the 'e' file is: 
    {:?}",
        e
    );
}
