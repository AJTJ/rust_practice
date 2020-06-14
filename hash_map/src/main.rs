use std::collections::HashMap;

fn main() {
    // The HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Blue team has {} points", scores["Blue"]);

    //Using Collect to turn VECTOR of TUPLES into HASHMAP

    let other_teams = vec![String::from("Dragons"), String::from("Griffons")];

    let other_scores = vec![33, 55];

    // zipping two VECTORS turned into iterators
    let other_pairs: HashMap<_, _> = other_teams
        .into_iter()
        .zip(other_scores.into_iter())
        .collect();

    println!("the other pairs: {:?}", other_pairs);

    //HashMap ownership
    let a = String::from("fav colour");
    let b = String::from("is blue");

    let mut my_map = HashMap::new();
    my_map.insert(&a, b);

    // strings do not have the `COPY` trait,
    // and thus their values will be moved i32 DOES have the `COPY` trait
    // println!("a and b are: {} {}", a, b);

    // the get method returns an Option
    let the_color = match my_map.get(&a) {
        None => "none",
        Some(i) => i,
    };

    println!("fav colour is: {}", the_color);

    for (key, value) in my_map {
        println!("in for loop: {}, {}", key, value)
    }
    //OVERWRITING
    scores.insert(String::from("Blue"), 300);
    scores.insert(String::from("Blue"), 500);
    println!("new scores: {:?}", scores);

    //Only INSERTING if the key has NO VALUE

    scores.entry(String::from("Blue")).or_insert(10000);
    scores.entry(String::from("Red")).or_insert(10000);

    println!("inserting if empty: {:?}", scores);

    // using a HASHMAP to update a key's value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("the map of text: {:?}", map);
}
