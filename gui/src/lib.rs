pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("this would draw a button");
    }
}

// A trait with one method: "draw"
pub trait Draw {
    fn draw(&self);
}

// Here `Screen` holds a vector called components
// And this vector is of type `Box<dyn Draw>`, which is a trait object
// i.e. a stand-in for any type inside a Box that implements the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// `run()` calls the draw method on each of its `components'
// This is even more generic than using a `generic` since a true generic would require each concrete type in `Screen` be of the same type
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
