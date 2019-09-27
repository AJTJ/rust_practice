fn main() {
    if numbers(5, 6)[0] > 6 {
        println!("YA OVER 6",)
    } else {
        println!("UNDAH 6",)
    }
}

// setting the return data as an array
fn numbers(x: i32, y: i32) -> [i32; 2] {
    [x + 1, y + 2]
}
