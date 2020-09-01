use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

fn main() {
  let _my_phrase = "apple";

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
}
