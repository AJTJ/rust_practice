fn main() {
    let s1 = String::from("hello");
    // the ampersand (&) is a reference to a pointer.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
    println!("The string is {}", s1);

    // how to mutate a ref PART 1
    let mut mut_s = String::from("memes");
    change(&mut mut_s);
    println!("The new string is {}", mut_s);

    // how to mutate a ref PART 2
    fn change(some_string: &mut String) {
        some_string.push_str(", are dope")
    }

    // two mutable references are not possible
    // this is to avoid data races and other efficiency reasons
    let z = &mut mut_s;
    // the 'y' would not work, since a mutable reference has already been created with 'z'.
    // let y = &mut mut_s;
    println!("the z {}", z);

    let mut d = String::from("The dankness");

    let anutha = &d;
    // the 'moar' would not work since there is already an immutable ref and you cannot have both a mutable and immutable ref in the same scope.
    // let moar = &mut d;

    println!("anutha {}", anutha);
    //anutha goes out of scope here because it is not used again.

    // here it is ok to declare a mutable ref since `anutha` is out of scope
    let moar = &mut d;

    println!("moar {}", moar);

    let reference_to_nothing = dangle();

    println!("the ref to nothing {}", reference_to_nothing);
}

//DANGLING REF
fn dangle() -> String {
    // s is declared here and thus will be de-allocated after the function ends
    let s = String::from("hello");
    // THIS WOULD BE A DANGLING REF
    // because: returning a ref to something that is gone would cause a dangling ref.
    // &s
    // returning the variable itself actually passes that data, so it's fine
    s
}

//the s is a reference to a string
// NOTE: This reference can also exist because it is scoped differently from 'z' above.
fn calculate_length(s: &String) -> usize {
    s.len()
}
// after this function s goes out of scope, but it has no ownership of it therefore nothing happens (drop() is not called??)

// This function won't work because references are immutable by default.
// fn not_working_change(some_string: &String) {
// some_string.push_str(", world")
// }
