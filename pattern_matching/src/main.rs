#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Using match to match the variant of a given enum
fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Using MATCH with an OPTION
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!(
        "The value of a {:#?} is {} cents",
        Coin::Penny,
        value_in_cents(Coin::Penny)
    );
    let some_number = Some(8);
    println!(
        "Adding one to {:?} is equal to {:?}",
        some_number,
        plus_one(some_number)
    );

    //Using IF LET
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("is 3 in regular match"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("is 3 in If LET")
    }

    if let Some(4) = some_u8_value {
        println!("is also 4 in IF LET with ELSE")
    } else {
        println!("Not 4 this time")
    }
}
