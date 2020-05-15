struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
            || self.width > other.height && self.height > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 20,
    };

    let sq = Rectangle::square(7);

    println!("The area of the rectangle is {} square pxls.", rect1.area());

    println!(
        "rec1 can hold rec2? {}, rec1 can hold rec3? {}",
        rect1.can_hold(&rect2),
        rect1.can_hold(&rect3)
    );

    println!("check my square area! {}", sq.area())
}
