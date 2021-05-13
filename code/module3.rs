mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


pub fn eat_at_restaurant() {
    pub use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {
    hosting::add_to_waitlist();
}

