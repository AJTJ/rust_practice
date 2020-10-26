fn main() {
    let x = 4;
    let y = 4;
    // FnOnce + Fn
    let equal_to_x = |z| z == x;

    print!("x: {}", x);
    assert!(equal_to_x(y));
    print!("thing: {}", equal_to_x(y));

    move_func();
}

fn move_func() {
    let x = vec![1, 2, 3];
    let y = vec![1, 2, 3];

    // in this case x the "move" keyword moves "x" into the closure
    let equal_to_x = move |z| z == x;

    // will NOT compile
    // print!("can't use x here: {:?}", x);
    assert!(equal_to_x(y));
}
