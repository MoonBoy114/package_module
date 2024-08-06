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