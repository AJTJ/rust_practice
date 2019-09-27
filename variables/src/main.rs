fn main() {
    // MUTABLE VARIABLE
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // CONST variable
    const MAX_POINTS: u32 = 100_000;
    println!("the max points is: {}", MAX_POINTS);

    // SHADOWING a variable
    let y = 10;

    let y = y + 10;

    let y = y + 12;

    println!("The value of y is: {}", y);

    let spaces = "      ";
    let spaces = spaces.len();

    println!("The length of spaces is: {}", spaces);

    // DATA TYPES

    let _guess: u32 = "42".parse().expect("Not a number!");

    println!("The guess is: {}", _guess);

    // SCALAR VALUES

    // FLOATING POINT
    let float_point = 2.0;

    println!("The floater is {}", float_point);

    // Character Type
    let z = 'z';

    println!("z {}", z);

    // COMPOUND TYPES

    // TUPLE

    let biggle: (i32, f32, i8) = (400, 6.2, 5);

    // destructuring into three parts
    let (uu, yy, tt) = biggle;

    println!("the tuple uu is {} {} {}", uu, yy, tt);

    // accessed by index too
    let four_hundred = biggle.0;

    let six_point_2 = biggle.1;

    println!("def 400 {}, and 6.2 {}", four_hundred, six_point_2);

    // ARRAY

    let my_first_rust_array: [i32; 4] = [1, 2, 3, 4];

    println!("2nd element of array {}", my_first_rust_array[1]);

    // shortcut: the first value is the value of all elements of the array
    let array_of_all_4s = [4; 10];

    println!(
        "a bunch of fours {} {}",
        array_of_all_4s[0], array_of_all_4s[1]
    );

    let alphabet_array: [char; 4] = ['A', 'B', 'C', 'D'];

    println!("the alph {}", alphabet_array[1])
}
