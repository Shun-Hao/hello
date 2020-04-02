pub fn log_mylib() {
    println!("*******************Chapter 7***********************");
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("call add_to_waitlist");
            seat_at_table();
        }

        fn seat_at_table() {
            println!("call seat_at_table");
            super::more_seat(); // item in child module can use parent's private function
        }
    }

    fn more_seat() {
        println!("bring more seat");
    }
}


//use crate::front_of_house::hosting;
use self::front_of_house::hosting as myhost;
use rand::Rng;

pub fn test_front () {
    myhost::add_to_waitlist();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("rand: {}", secret_number);
}

fn do_in_back() {
    println!("run do_in_back");
}

mod back_of_house {
    pub mod hosting {
        pub fn fix_incorrect_order() {
            cook_order();
            super::super::do_in_back();
        }

        fn cook_order() {
            println!("in cook_order");
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
            pub fn set_seasonal_fruit(&mut self, fruit: &str) {
                self.seasonal_fruit = String::from(fruit);
            }

            pub fn show_breakfast(&self) {
                println!("breakfast is: {} {}", self.toast, self.seasonal_fruit);
            }
        }

        #[derive(Debug)]
        pub enum Appetizer {
            _Soup,
            _Salad,
        }
    }
    
}

use back_of_house::hosting;

pub fn test_back () {
    hosting::fix_incorrect_order();

    // items in struct are private by default
    let mut bft = hosting::Breakfast::summer("Wheat");
    bft.show_breakfast();
    bft.set_seasonal_fruit("apple");
    bft.show_breakfast();


    // items in enum are pub by default
    let appet = hosting::Appetizer::_Soup;
    println!("appetizer: {:#?}", appet);
}