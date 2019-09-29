fn main() {
    println!("50 C in F is {}", celc_far(50, false));
    println!("122 F in C is {}", celc_far(122, true));

    let fib_num = 20;
    println!("{} fib number is {}", fib_num, gen_fib(20));
}

fn celc_far(temp: i32, is_f: bool) -> i32 {
    // (32°F − 32) × 5/9 = 0°C
    if is_f {
        (temp - 32) * 5 / 9
    } else {
        temp * 9 / 5 + 32
    }
}

fn gen_fib(target: i32) -> i32 {
    let mut first_number = 0;
    let mut second_number = 1;
    for _ in 0..target - 2 {
        let outcome = first_number + second_number;
        first_number = second_number;
        second_number = outcome;
    }

    return second_number;
}
