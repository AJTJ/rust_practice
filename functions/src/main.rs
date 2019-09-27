fn main() {
    //this is a STATEMENT
    let _x = 5;
    let y = {
        let x = 3;
        /*
            the following is an EXPRESSION
            it does not have a semicolon
            semicolons are reserved for STATEMENTS
        */
        x + 1
    };
    another_function(y, five(), plus_one(10))
}

fn another_function(x: i32, y: i32, z: i32) {
    println!("We out here, 1st value! {}", x);
    println!("We out here, 2nd value! {}", y);
    println!("We out here, 3rd value! {}", z);
}

// declaring the return type
// 5 is the return value, i.e. the expression
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // if this were x + 1; (with semicolon)
    // it would evaluate to an empty tuple ()
    x + 1
}
