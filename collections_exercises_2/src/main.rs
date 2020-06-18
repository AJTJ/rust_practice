/*
 TODO
 - Take punctuation into account
*/

fn main() {
    fn switch_casing(word: &str) -> String {
        let (big, end) = word.split_at(1);
        [&big.to_uppercase(), end].join("")
    };

    fn alter_word(word: &str) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        if word.chars().nth(0).map_or(false, |c| vowels.contains(&c)) {
            [word, "hay", " "].join("")
        } else {
            let (first, mut last) = word.split_at(1);
            let big_last;
            if first.chars().all(char::is_uppercase) {
                big_last = switch_casing(last);
                last = &big_last
            }
            [last, &first.to_lowercase(), "ay", " "].join("")
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
