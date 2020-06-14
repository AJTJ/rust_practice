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
    let a = String::from("fav colour44");
    let b = String::from("is blue");

    let mut my_map = HashMap::new();
    my_map.insert(a, b);

    // strings do not have the `COPY` trait, and thus their values will be moved
    //i32 DOES have the `COPY` trait
    // println!("a and b are: {} {}", a, b);
}
