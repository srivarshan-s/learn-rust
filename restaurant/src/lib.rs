mod front_of_house {
    pub mod hosting{
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // The next line does not compile as seasonal_fruit is a private field
    // meal.seasonal_fruit = String::from("blueberries");
    println!("I'd like {} toast please", meal.toast);

    // If enum is public, then all its values are then public
    let order1 = crate::back_of_house::Appetizer::Salad;
    let order2 = crate::back_of_house::Appetizer::Soup;

    // Bringing paths into scope with the "use" keyword
    use crate::front_of_house::hosting;
    // "use" can be used with relative paths too
    // use self::front_of_house::hosting;
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Creating Idiomatic use Paths
    use crate::front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();

    // Providing New Names with the "as" keyword
    use std::io::Result as io_result;
    fn function() -> io_result<()> {
        Ok(())
    }

    // Re-exporting Names with "pub use"
    pub use crate::front_of_house::hosting as Hosting;

    // Using External Packages
    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..101);
    use std::collections::HashMap;

    // Using Nested Paths to Clean Up Large "use" Lists
    // use std::cmp::Ordering;
    // use std::io;
    use std::{cmp::Ordering, io};
    // use std::io;
    // use std::io::Write;
    // use std::io::(self, Write);
    
    // The Glob Operator
    use std::collections::*;
}
