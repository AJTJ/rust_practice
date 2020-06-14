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

    let x = String::from("tic");
    let y = String::from("tac");
    let z = String::from("toe");

    //Using the FORMAT! macro
    // does NOT take ownership
    let full_tic = format!("{}-{}-{}", x, y, z);

    println!("the full tic: {}", full_tic);
    println!("just a tic: {}", x);

    //Indexing into Strings
    let h = String::from("abcdefg");

    //The following will NOT work, since each unicode scalar value will take different amounts of memory (bytes) depending on the character.
    // indexing a string is not allowed
    // println!("1st index of h: {}", h[0])

    // the way to grab chars
    for i in h.chars() {
        println!("another char in h: {}", i)
    }

    // the way to grab bytes
    for i in h.bytes() {
        println!("another byte in h: {}", i)
    }

    //Grapheme clusters from strings is another thing, but is not in the stlib.
}
