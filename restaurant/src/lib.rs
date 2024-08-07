mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit:String::from("banana"),
            }
        }
    }

    pub fn eat_at_rest() {
        let mut meal= back_of_house::Breakfast::summer("ржаной");
        meal.toast = String::from("пшеничный");
        meal.seasonal_fruit = String::from("черника");
        println!("Я бы хотела {} тост, пожалуйста", toast);
    }
}


mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

mod front_of_house {
   pub  mod hosting {
        pub fn add_to_walklist() {}
    }
}

use crate::front_of_house::hosting;
use self::front_of_house::hosting::add_to_walklist;

pub fn eat_at_restaurant() {
    add_to_walklist();
    add_to_walklist();
    add_to_walklist();

}






//    pub mod hosting {
//        pub fn add_to_walklist() {}
//         fn seat_at_table() {}
//     }
//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }
// pub fn eat_at_rest() {
//     crate::front_of_house::hosting::add_to_walklist();
//     front_of_house::hosting::add_to_walklist();
// }