use crate::garden::vegetables::Asparagus;
use std::{cmp::Ordering, io};

// tells the compiler to include the code it finds in src/garden.rs
pub mod garden;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn main() {
    println!("Hello, world!");
}
