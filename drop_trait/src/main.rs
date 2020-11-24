struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("My other stuff"),
    };
    println!("Created some CustomSmartPointers");

    // MANUALLY DROPPING
    // We cannot manually call the drop trait like so:
    // c.drop()
    // We can call the std::mem::drop function, like so
    drop(c);
    println!("Dropped one of the custom pointers early");
}
