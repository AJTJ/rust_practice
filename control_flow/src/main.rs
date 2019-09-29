fn main() {
    println!("the numbers {}", numbers(5, 6)[0]);

    if numbers(5, 6)[0] > 6 {
        println!("YA OVER 6",)
    } else if numbers(5, 6)[0] <= 6 {
        println!("UNDAH or equal to 6",)
    };

    let dog = false;

    if dog {
        println!("yes dog!",)
    }

    if !dog {
        println!("no dog")
    }

    const THE_NUMBER: i32 = numbers(5, 6)[0];

    if THE_NUMBER % 4 == 2 {
        println!("2 remain!")
    }

    let condition = true;

    let dank = if condition { 5 } else { 6 };

    //Different IF TYPES
    // let dank = if condition { 5 } else { "somethign" };

    println!("that dank {}", dank);

    //WILL LOOP continuously
    // loop {
    //     println!("AGAIN")
    // }

    let mut counter = 0;

    let result = loop {
        println!("current counter {}", counter);
        counter += 1;

        if counter == 10 {
            // return value AFTER break
            break counter;
        }
    };

    println!("the result {}", result);

    let mut countdown = 3;

    while countdown != 0 {
        println!("{}!", countdown);

        countdown -= 1;
    }
    println!("LIFT OFF!");

    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90];

    let mut ii = 0;

    while ii < 8 {
        println!("the value is {}", a[ii]);
        ii += 1;
    }

    for e in a.iter() {
        println!("The value in the for is {}", e)
    }

    for num in (1..4).rev() {
        println!("{}!", num)
    }
    println!("SECOND LIFTOFF!");
}

// setting the return data as an array
const fn numbers(x: i32, y: i32) -> [i32; 2] {
    [x + 1, y + 2]
}
