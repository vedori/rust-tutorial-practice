// This example uses inline declarations of the module
// however, the code can be defined away from the declaration
// and Rust will look for it in certain places
#![allow(unused)]

// this module doesnt need to be public
mod front_of_house {
    // this submodule needs to be public because `add_to_waitlist`
    // is being publically accessed
    pub mod hosting {
        // also needs to be pub specifying the module itself isnt enough
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    // even if a struct is public it's items need to be specified public for public access
    pub struct Breakfast {
        pub toast: String,

        // customer doesnt need to have control over or see the seasonal fruit that changes
        seasonal_fruit: String,
    }

    impl Breakfast {
        // needs a constructor because the public struct has some private fields
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                // can access private fields in a public function
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // If we were to move eat_at_restaurant() and front_of_house into a new
    // module (instead of crate::) it would make sense to use relative paths

    // If we were to move eat_at_restaurant() to a seperate module it would make
    // sense to use absolute paths to resolve front_of_house items

    // In general, the 2nd scenario is more likely
    // because moving code definitions (like mod foo {}) away from item calls
    // is more likely so absolute paths will be used more

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    // cannot change seasonal fruit its private
    // meal.seasonal_fruit = String.from("Blueberries");

    println!("I'd like {} toast please", meal.toast);
}
