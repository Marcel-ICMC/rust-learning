mod front_of_house {
    pub mod b {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("Coiso")
            }
        }
    }
}

use crate::front_of_house::b::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod banana {
    use super::hosting;

    pub fn nana() {
        hosting::add_to_waitlist();
    }
}

fn main () {
    banana::nana();
}
