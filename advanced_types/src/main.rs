use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::io::Result as OtherResult;
use uuid::Uuid;

fn main() {
    // NEW TYPE AND UUID for hidden ID in User tuple-struct
    {
        #[derive(Debug)]
        struct User(HashMap<Uuid, String>);

        impl User {
            pub fn add_user(name: String) -> User {
                let mut my_map = HashMap::new();
                my_map.insert(Uuid::new_v4(), name);
                User(my_map)
            }
        }

        let me = User::add_user(String::from("Jim"));

        println!("{:?}", me);
    }

    //TYPE SYNONYMS AND TYPE ALIASES
    {
        // Not a separate, new type.
        // used for avoiding repetition
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        assert_eq!(x, y);
    }

    // RESULT SIMPLIFICATIONS
    {
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        }

        //this can be simplified to the following

        pub trait SimplifiedWrite {
            fn write(&mut self, buf: &[u8]) -> OtherResult<usize>;
        }
    }

    // EMPTY TYPE OR NEVER TYPE - asserts that the function will NEVER return

    {
        // fn bar() -> ! {};

        // A loop (unless it breaks), always returns !
        // continue returns !
        // panic returns !
        loop {
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);
            break;
        }
    }

    // DYNAMICALLY SIZED TYPES AND THE "SIZED" TRAIT
    // The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

    {
        // let s1: str = "Hello there!";
        let s1: &str = "Hello there!";

        // let vals = [u8];

        println!("{}", s1);

        //...this...
        fn generic_1<T>(_t: T) {}
        //...is the same as this...
        fn generic_2<T: Sized>(_t: T) {}
        // ...generally generics only work on types that have a known size at compile time
        // but this can be relaxed like so:
        // The type might not be Sized, so we need to use it behind some kind of pointer, in this case it is a reference: "&T"
        fn relaxed_generic<T: ?Sized>(_t: &T) {}
    }
}
