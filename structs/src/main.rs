// creating a new STRUCT
// this debug line allows us to pretty print the struct
// we are declaring the owned `String` type rather than the &str string slice type, to enforce that this struct owns all of its data and for that data to be valid as long as the entiere struct is valid. Lifetimes will have to be used if we want to use &str.
#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

// creating a new user with a struct
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

fn main() {
    let mut user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("name1"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1: {:?}", user1);

    //updating using dot notation
    user1.email = String::from("newemail@ex.com");

    println!("user1 with new email: {:?}", user1);

    // using Struct update syntax
    let user2 = User {
        email: String::from("user2@ex.com"),
        username: String::from("name2"),
        ..user1
        // you could also use dot notation
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
    };
    println!("user2: {:?}", user2);

    //how to create a TUPLE STRUCT
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:#?}", black);
    println!("origin: {:#?}", origin);
}
