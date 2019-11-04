fn main() {
    let word = String::from("Hundred");
    println!("The length of {} is {} ", word, first_word(&word));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
