use std::fmt;
use std::ops::Add;

fn main() {
    // ASSOCIATED TYPES
    // We don't need to annotate types because we can't implement a trait on a type multiple times.
    // There can only be one "impl Iterator for SOMETHING"
    {
        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }
    }
    // DEFAULT GENERIC TYPE PARAMETERS
    // OPERATOR OVERLOADING
    // i.e. customizing the behavior of an operator
    // Here we are allowing let point3 = point1 + point2;
    // instead of let point3 = point1.add(point2);
    {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        println!(
            "Point 1: {:?} + Point 2: {:?} = {:?}",
            Point { x: 1, y: 3 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 3 } + Point { x: 2, y: 1 }
        )
    }

    // The definition of "Add"
    // Rhs is short for "right hand side", and it defines the type of the rhs parameter.
    // and also defines that the default is "Self"
    {
        trait Add<Rhs = Self> {
            type Output;

            fn add(self, rhs: Rhs) -> Self::Output;
        }
    }

    // CUSTOMIZING THE DEFAULT TYPE
    {
        #[derive(Debug)]
        struct Millimeters(u32);
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }

        let mils = Millimeters(2500);
        let mets = Meters(659);

        println!("mils + mets = {:?}", mils + mets);
    }

    // Calling methods with the same name

    {
        trait Pilot {
            fn fly(&self);
        };

        trait Wizard {
            fn fly(&self);
        };

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }
        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human;

        // Because the "fly" method takes a "self" paramter, if had two types that both implement one trait, Rust could figure out which implementation of a trait to use based on the type of "self".
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }

    // ASSOCIATED FUNCTIONS that don't have the self paramter
    {
        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        println!("A baby dog is called a {}", Dog::baby_name());
        // to access the Animal implementation, we need fully qualified syntax.
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

        // FULLY QUALIFIED SYNTAX PATTERN
        // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    }
    // SUPERTRAITS, requiring one trait's functionality with another trait
    {
        //Because we’ve specified that OutlinePrint requires the Display trait, we can use the to_string function that is automatically implemented for any type that implements Display. If we tried to use to_string without adding a colon and specifying the Display trait after the trait name, we’d get an error saying that no method named to_string was found for the type &Self in the current scope.
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        };
        struct Point {
            x: i32,
            y: i32,
        }

        // This will not work unless Point implements Display
        impl OutlinePrint for Point {}

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        let my_point = Point { x: 12, y: 3 };

        println!("{:?}", OutlinePrint::outline_print(&my_point));
    }

    // NEWTYPE PATTERN to IMPLEMENT EXTERNAL TRAITS ON EXTERNAL TYPES
    {
        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);

        // The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding. We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>. If we wanted the new type to have every method the inner type has, implementing the Deref trait (discussed in Chapter 15 in the “Treating Smart Pointers Like Regular References with the Deref Trait” section) on the Wrapper to return the inner type would be a solution. If we don’t want the Wrapper type to have all the methods of the inner type—for example, to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.
    }
}
