// struct has fields private or public case-by-case
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

// on the other hand, all enum variants are public if enum is public
pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
