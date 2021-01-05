use alloc::prelude;

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => print!("Got 50"),
        // in the following case "y" matches whatever value already exists in "x"
        // it is NOT the "y" declared in the outer scope. It is a NEW variable declaration
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let c = 1;

    match c {
        // using the OR expression
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let d = 5;

    match d {
        // using RANGE
        1..=5 => println!("Is one to 5"),
        _ => println!("Is anything else"),
    }

    let e = 'c';

    match e {
        // RANGE with characters
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 17 };

    // let Point { x: a, y: b } = p;
    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
