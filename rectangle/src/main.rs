fn main() {
    // FIRST METHOD Using Variables
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {}", area1(width1, height1));

    // SECOND METHOD Using typles
    let tup_rec = (30, 50);

    println!("V2 area: {}", area2(tup_rec));

    //THIRD METHOD Using structs
    let str_rec = Rectangle {
        width: 30,
        height: 50,
    };

    println!("V2 area: {}", area3(&str_rec))
}

fn area1(width: i32, height: i32) -> i32 {
    height * width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: i32,
    height: i32,
}

fn area3(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
