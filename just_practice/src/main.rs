// use chrono::prelude::*;
// use std::io;
// #[derive(Debug)]
// struct User {
//     name: Option<String>,
//     id: Option<i32>,
//     date: Option<String>,
// }

// fn main() {
//     let mut current_user = User {
//         name: None,
//         id: None,
//         date: None,
//     };

//     println!("What is your name?");
//     let mut current_input = String::new();

//     io::stdin()
//         .read_line(&mut current_input)
//         .expect("failed to get information");

//     println!("Got your name: {}", current_input);
//     current_user.name = Some(current_input.trim().clone().parse::<String>().unwrap());

//     current_input.clear();

//     println!("ID?");

//     io::stdin()
//         .read_line(&mut current_input)
//         .expect("failed to get information");

//     println!("Got it, ID is: {}", current_input);
//     current_user.id = Some(current_input.trim().clone().parse::<i32>().unwrap());

//     current_user.date = Some(Local::now().to_string());

//     println!("All: {:?}", current_user);
// }

use chrono::prelude::*;
use std::io::BufRead;

#[derive(Debug)]
struct User {
    name: String,
    id: i32,
    date: DateTime<Local>,
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);

    println!("What is your name?");
    let name = lines.next().unwrap();
    println!("Got your name: {}", name);

    println!("ID?");
    let id = lines.next().unwrap().parse().expect("Could not parse ID");
    println!("Got it, ID is: {}", id);

    lines.for_each(|l| println!("a line: {}", l));

    let date = Local::now();

    let current_user = User { name, id, date };

    println!("All: {:?}", current_user);
}

// use chrono::prelude::*;
// use std::io::BufRead;

// #[derive(Debug)]
// struct User {
//     name: String,
//     id: i32,
//     date: DateTime<Local>,
// }

// fn main() {
//     let stdin = std::io::stdin();

//     let current_user = {
//         let mut lines = stdin.lock().lines().filter_map(Result::ok);

//         println!("What is your name?");
//         let name = lines.next().unwrap();
//         println!("Got your name: {}", name);

//         println!("ID?");
//         let id = lines.next().unwrap().parse().expect("Could not parse ID");
//         println!("Got it, ID is: {}", id);

//         let date = Local::now();

//         User { name, id, date }
//     };

//     println!("All: {:?}", current_user);
// }
