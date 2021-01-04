mod front_of_house;

pub use crate::front_of_house::hosting; // Absolute path
//use self::front_of_house::hosting; // Relative path

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    //meal.seasonal_fruit = String::from("blueberries"); // compile error
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house;

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result { unimplemented!(); }
fn function2() -> IoResult<()> { unimplemented!(); }
