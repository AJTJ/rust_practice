fn main() {
    // creates an empty string, to load data into
    let mut my_string = String::new();

    // applying the to_string method on a string literal
    my_string = "initial content".to_string();
    // this is equivalent to the following
    // my_string = String::from("initial content");

    println!("my_string is: {}", my_string);

    let mut d = String::from("Dank");

    // PUSH_STR takes a string slice because we don't necessarily want to take ownership of the param
    let e = "Memes";
    d.push_str(e);

    println!("d is: {}", d);
    println!("e is: {}", e);

    // PUSH adds one char
    d.push('z');
    println!("d is: {}", d);

    // CONCATENATION with the +
    // both d and e have been moved here and cannot be used again
    println!("d + e: {}", d + e);
}
