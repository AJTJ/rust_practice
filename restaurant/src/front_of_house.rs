// a module containing other modules for a restaurant style system.
// crate -> front_of_house -> hosting etc..
// front_of_house isn't private, but is accessible from eat_at_restaurant since they are siblings.
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
  }
}

// imports the hosting contents
pub mod hosting;
