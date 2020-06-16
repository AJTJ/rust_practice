fn main() {
    fn to_pig(words: &String) -> String {
        fn alter_word(word: &str) -> String {
            // let vowels = ["a", "e", "i", "o", "u", "y"];

            println!("word.chars: {:?}", word.chars());

            // if word.chars().nth(0).map_or(|c| vowels.contains(*c), false) {
            // } else {
            // }

            String::from("return")
        };

        let split = words.split_whitespace().map(|x| alter_word(&x));

        println!("the split: {:?}", split);

        String::from("Return")
    }

    let data = String::from("The big hairy dog walked in a circle");

    to_pig(&data);
}
// 19:03 < mipri> the above wouldn't be too verbose with if let Some(c) = ... {}
// 19:03 < mipri> and that can be an expression that returns false in the else case
// 19:03 < yrp> yeah

// 18:55 < yrp> words.chars().nth(0).map_or(|c| vowels.contains(*c), false)
// 18:56 < yrp> might be a more ergonomic way
// 18:56 < yrp> of doing what you want
