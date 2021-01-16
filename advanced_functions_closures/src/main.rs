fn add_one(x: i32) -> i32 {
    x + 1
}

// unlike closures, "fn" is a type rather than a trait, so we specify "fn" as the parameter type directrly rather than declaring a generic type param with one of the "Fn" traits as a trait bound.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The ans is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    // using a closure
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // using named function instead (with fully qualified syntax, because there are multiple fns named "to_string")
    let other_list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    // Initializer Function
    {
        #[derive(Debug)]
        enum Status {
            Value(u32),
            Stop,
        }

        //
        let list_of_statuses: Vec<_> = (0u32..20).map(Status::Value).collect();
        let another_list_of_statuses: Vec<_> = (0u32..20).map(|i| Status::Value(i)).collect();
        println!("list of statuses: {:?}", list_of_statuses);
        println!("another list of statuses: {:?}", another_list_of_statuses);
    }

    // Returning Closures
    // Cannot be returned directly because they do NOT have a concrete type that is returnable.
    {
        // DOES NOT WORK, because rust does not mnow how much space it will need to store the closure
        // fn returns_closure() -> dyn Fn(i32) -> i32 {
        //     |x| x + 1
        // }

        // DOES WORK
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
        // ALSO WORKS
        fn also_returns_closure() -> impl Fn(i32) -> i32 {
            |x| x + 1
        }
    }
}
