fn main() {
    // the VECTOR
    let mut v1: Vec<i32> = Vec::new();
    //SHORTHAND
    let v2 = vec![1, 2, 3];

    // PUSHING
    v1.push(1);
    v1.push(4);
    v1.push(7);
    v1.push(3);

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    // the code will panic and crash using the bracket notation if the index referenced is empty.
    let third: &i32 = &v1[2];
    println!("v1's third type: {}", third);
    println!("v1 = {:?}", v1);

    let also_third: i32 = v1[2];
    println!("v1's third type, again: {}", also_third);
    println!("v1 = {:?}", v1);

    // using match to CHECK
    match v1.get(2) {
        Some(third) => println!("The third ele is {}", third),
        None => println!("The third ain't nuffin"),
    };

    // MACRO for a vector
    let v = vec![1, 2, 3, 4, 5];

    //borrows v as immutable
    let first = &v[0];

    // this mutable reference is illegal because v is already borrowed as immutable
    // v.push(6);

    println!("The first element is: {}", first);

    // EXPLAINS REFERENCE RULES
    /*
    The code ABOVE might look like it should work: why should a reference to the first element care about what changes at the end of the vector? This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
    */

    let veccy = vec![22, 44, 55];

    //iterating over IMMUTABLE REFERENCES of a vector
    for i in &veccy {
        println!("the ele is {}", i);
    }

    let mut veccy2 = vec![22, 44, 55];
    //iterating over MUTABLE REFERENCES of a vector
    //The * operator dereferences
    for i in &mut veccy2 {
        *i += 50;
        println!("the ele in veccy2 is {:#?}", i);
    }

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Text(String),
        Float(f64),
    };

    let char_input = vec![
        SpreadsheetCell::Float(33.33),
        SpreadsheetCell::Text(String::from("Some text")),
        SpreadsheetCell::Int(66),
    ];

    println!(
        "The first value in the char_input vec is {:?}",
        char_input[0]
    );
} // v1 and v2 would go out of scope here
