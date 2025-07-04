use chrono::Local;

use crate::product::structs::{ExpirationDate, ShippingWeight};

mod product;

fn main() {
    let x = ExpirationDate::now();
    let y = ShippingWeight::new(12f64);

    println!("Current Date is {x}");
    println!("Current Weight is {y}");
}
