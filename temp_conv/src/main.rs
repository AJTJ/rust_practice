fn main() {
    println!("50 C in F is {}", celc_far(50, false));
    println!("122 F in C is {}", celc_far(122, true));
}

fn celc_far(temp: i32, is_f: bool) -> i32 {
    // (32°F − 32) × 5/9 = 0°C
    if is_f {
        (temp - 32) * 5 / 9
    } else {
        temp * 9 / 5 + 32
    }
}
