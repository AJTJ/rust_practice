use core::num;

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

    // DESTRUCTURING TO BREAK APART VALUES

    // STRUCT DESTRUCTURING

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 17 };

    // let Point { x: a, y: b } = p;
    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(17, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // ENUM DESTRUCTURING (AND NESTED VALUES)

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x directin {}, and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to (HSV) red {}, green {}, and blue {}",
                h, s, v
            )
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to (RGB) red {}, green {}, and blue {}",
                r, g, b
            )
        }
    }

    // MORE COMPLEX DESTRUCTURING

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!(
        "The complex destructure equals feet {}, inches {} and point x: {}, and point y: {}",
        feet, inches, x, y
    );

    // IGNORING VALUES IN PATTERNS

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    };

    foo(3, 4);

    // IGNORING PARTS OF A VALUE WITH A NESTED _
    // The business requirements are that the user should not be allowed to overwrite an existing customization of a setting but can unset the setting and give it a value if it is currently unset.

    let mut _setting_value = Some(5);
    let new_setting_value = Some(10);

    match (_setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value")
        }
        _ => _setting_value = new_setting_value,
    }

    // VARIABLE BINDING ISSUES

    let s = Some(String::from("Hello!"));

    // below "_s" still binds the value to the variable, whereas "_" does not bind at all.
    // if let Some(_s)
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    // IGNORING THE OTHER VALUES WITH ".."

    struct NewPoint {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = NewPoint { x: 0, y: 0, z: 0 };

    match origin {
        NewPoint { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // The following is too ambiguous
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some nubmers: {}", second);
    //     }
    // }

    // MATCH GUARDS: conditional "if" statement

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let k = Some(5);
    let l = 10;

    match k {
        Some(50) => println!("got 50"),
        // Here we can test the value of the outer variable
        // n == l is NOT a pattern and therefore does not introduce new variables
        Some(n) if n == l => println!("Matched, n = {}", n),
        _ => println!("Default case, k = {:?}", k),
    }

    println!("Again, at the end, k = {:?} and l = {}", k, l);

    let xx = 4;
    let yy = true;

    match xx {
        4 | 5 | 6 if yy => println!("yes!"),
        // this works because it behaves like so "(4 | 5 | 6) if y => .." in a match guard
        _ => println!("no"),
    };

    // @ BINDING FOR CREATING A VARIABLE

    enum NewMessage {
        Hello { id: i32 },
    }

    let msg = NewMessage::Hello { id: 5 };

    match msg {
        NewMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range, {}", id_variable),
        NewMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        NewMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}
