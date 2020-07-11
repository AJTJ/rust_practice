use std::cmp::Ordering;

// fn largest<T>(number_list: &[T]) -> T {
//     let mut largest = number_list[0];

//     for &number in number_list {
//         // TODO: Fix this
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// A struct with two generic types
// struct: a collection of named pieces of different types
#[derive(Debug)]
struct DoublePoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> DoublePoint<T, U> {
    fn mixup<V, W>(self, other: DoublePoint<V, W>) -> DoublePoint<T, W> {
        DoublePoint {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//behold the tuple struct, for creating a 'newtype'
struct Inches(i32);

fn main() {
    // a vector: i.e. a re-sizable array   of all the same data types
    let number_list = vec![34, 50, 25, 100, 65];

    // let largest_num = largest(&number_list);
    // println!("The largest number is {}", largest_num);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let largest_num = largest(&number_list);
    // println!("The largest number is {}", largest_num);

    let char_list = vec!['g', 'y', 'a', 'z'];

    // let largest_char = largest(&char_list);
    // println!("The largest char is {}", largest_char);

    let my_point = DoublePoint { x: 33, y: 55.2 };

    let p = Point { x: 5, y: 22 };

    let pp = DoublePoint { x: 22, y: 3.5 };
    let pp2 = DoublePoint {
        x: "Dream",
        y: "Scape",
    };
    let pp3 = pp.mixup(pp2);

    println!("pp3 = {:?}", pp3);

    println!("p.x = {}", p.x())
}
