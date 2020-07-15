use std::fmt::Display;

fn main() {
    // In this example, within the inner scope the var x's lifetime ends, thus we are trying to reference something in the println that does not exist.
    //
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);
    //

    // the lifetime signatures here ensure that both parameters and the return value live at least as long as lifetime `a
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // this example functions correctly
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let mut result = longest(string1.as_str(), string2);
    println!("1ST: The longest string is {}", result);

    // this example works fine since result references something that is valid until the end of the inner scope
    let string3 = String::from("apples");
    {
        let string4 = String::from("bananas");
        result = longest(string3.as_str(), string4.as_str());
        println!("2ND: The longest string is {}", result);
    }

    // this example does NOT work because the lifetime references the shortest of the possible outcomes, and since we cannot be sure which that would be (since string6 lifes less long than string5), therefore this will not compile.
    //
    // let string5 = String::from("biglongapples");
    // let newResult;
    // {
    //     let string6 = String::from("bananas");
    //     newResult = longest(string5.as_str(), string6.as_str());
    // }
    // println!("3RD: The longest string is {}", newResult);
    //

    //This is the longest function from Listing 10-22 that returns the longer of two string slices. But now it has an extra parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause. This extra parameter will be printed before the function compares the lengths of the string slices, which is why the Display trait bound is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
