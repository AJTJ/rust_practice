use std::ops::Deref;

fn main() {
    // USING BOX TO STORE AN I32
    let my_box = Box::new(5);
    println!("my box = {}", my_box);

    // BASIC USE OF CONS TO CREATE A RECURSIVE DATA STRUCTURE
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // NON-RECURSIVE VARIANT OF A LINKED LIST
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // EX 1
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // DEREFERENCING SO IT IS NO LONGER A POINTER/REFERNCE, BUT AN ACTUAL INTEGER
    assert_eq!(5, *y);

    // EX 2
    let a = 5;
    // HERE WE ARE POINTING TO AN INSTANCE OF A BOX POINTING TO A COPIED VALUE OF X, RATHER THAN A REF POINTING TO A VALUE X
    let b = Box::new(x);

    assert_eq!(5, a);
    // MUST BE DEREFERENCED HERE AS WELL
    assert_eq!(5, *b);

    // TUPLE STRUCT WITH ONE ELEMENT "NEW TYPE PATTERN"
    #[derive(Debug)]
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // IMPLEMENTING A TRAIT REQUIRES PROVIDING IMPLEMENTATIONS FOR THE TRAIT'S REQUIRED METHODS
    // THE METHOD HERE IS: "deref"
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    let new_thang = MyBox::new(32);

    println!("new thang = {:?}", new_thang)
}
