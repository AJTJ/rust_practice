fn main() {
    println!("50 C in F is {}", celc_far(50, false));
    println!("122 F in C is {}", celc_far(122, true));

    let fib_num = 20;
    println!("{} fib number is {}", fib_num, gen_fib_sequence(20));
}

fn celc_far(temp: i32, is_f: bool) -> i32 {
    // (32°F − 32) × 5/9 = 0°C
    if is_f {
        (temp - 32) * 5 / 9
    } else {
        temp * 9 / 5 + 32
    }
}

fn gen_fib_sequence(target: i32) -> i32 {
    let mut x = 0;
    let mut y = 1;
    for _ in 0..target - 2 {
        let outcome = x + y;
        x = y;
        y = outcome;
    }

    return y;
}
