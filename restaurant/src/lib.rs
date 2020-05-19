#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn serve_order() {}

// using a semi-colon after front_of_house;, instead of front_of_house {} tells Rust to load the contents of the module from another file with the same name as the module.
mod front_of_house;

pub use crate::front_of_house::hosting;

// An example of using `super` to access the root dir of the parent module
mod back_of_house {
    fn fix_incorrect_ordre() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}

    // Demonstrating a use of having one field of a struct public (keeping the other private)
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // this public fn is necessary in order to create an instance of Breakfast, since Breakfast has a private field.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // designating and enum as pub makes all of its variants pub
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

//bringing a module into scope
// also re-exporting it
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //Absolute path
    // the crate keyword works here since `add_to_waitlist` is defined in the same crate
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path access to the add_to_waitlist
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast plz.", meal.toast);

    // the next line won't compile, since fruit is private
    // meal.seasonal_fruit = String::from("blueberries")

    // an example of using ALL public variants of an enum.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // hosting was brought into scope with `use`
    hosting::add_to_waitlist();
}
