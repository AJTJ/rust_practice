fn main() {
    fn alter_word(word: &str) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        if word.chars().nth(0).map_or(false, |c| vowels.contains(&c)) {
            [word, "hay", " "].join("")
        } else {
            let (first, last) = word.split_at(1);
            [last, first, "ay", " "].join("")
        }
    };

    fn to_pig(data: &String) -> String {
        data.split_whitespace().map(|x| alter_word(&x)).collect()
    }

    let data = String::from("The big hairy dog walked in a circle");

    println!(
        "
    
    from this: {} 
    
    to this: {}
    
    ",
        data,
        to_pig(&data)
    );
}
