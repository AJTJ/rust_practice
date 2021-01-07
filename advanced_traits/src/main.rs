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
}
