fn main() {
    let word = String::from("Hundred");
    println!("The length of {} is {} ", word, first_word(&word));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // looping through a tuple created by enumerate
    for (i, &item) in bytes.iter().enumerate() {
        // searching for space with byte literal
        if item == b' ' {
            return i;
        }
    }
    // if we don't return the i above, we return the length
    s.len()
}
