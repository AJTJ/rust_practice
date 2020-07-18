#[cfg(test)]
mod tests {
    // using "super" and the "*"
    // super references the scope above
    // "*" is a glob to grab everything
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    //Using RESULT allows us to use the ? operator in the body
    fn two_plus_two() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // This test will force a failure of the test
    // #[test]
    // fn another() {
    //     panic!("Make the test fail!")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // assert checks to see that it is true
        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // Second argument here is for a custom error msg
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // should_panic ensures the test passes if it panics
    // the expected substring is used to make the should_panic more useful, since it now expects a specific substring on panic
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
