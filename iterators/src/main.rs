fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // this changed the internal state of the iter, thus needing to be mut
    v1_iter.next();

    for val in v1_iter {
        println!("Got val: {}", val);
    }
}
