fn main() {
    let my_string = String::from("Hundred Millions");
    let word = first_word(&my_string);
    println!("The length of {} is {} ", my_string, word);

    //SLICES
    let slice = &word[2..5];
    println!("the slice: {}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("the string as bytes:{:?}", bytes);

    // looping through a tuple created by enumerate
    for (i, &item) in bytes.iter().enumerate() {
        // searching for space with byte literal
        // the b signifies that we are looking for the byte/byte array representation
        if item == b' ' {
            return &s[0..i];
            // previous iteration only returned the index
            // return i;
        }
    }
    // return the entire length of the slice, which is the same as the whole string
    &s[..]
    // previously returned the length of the string
    // s.len()
}
