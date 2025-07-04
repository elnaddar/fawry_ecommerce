use serde::{Deserialize, Serialize};

use crate::product::structs::{ExpirationDate, Product, ShippingWeight};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub name: String,
    pub quantity: u64,
    pub price: u64,
    pub expiry_date: Option<ExpirationDate>,
    pub shipping_info: Option<ShippingWeight>,
}

impl CartItem {
    fn from_product(quantity: u64, product: Product) -> Self {
        if quantity == 0 {
            panic!("Quantity cannot be 0")
        }
        Self {
            name: product.name,
            quantity,
            price: product.price,
            expiry_date: product.expiry_date,
            shipping_info: product.shipping_info,
        }
    }
}
