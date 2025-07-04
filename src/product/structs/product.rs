use crate::product::structs::{ExpirationDate, ShippingWeight};

pub struct Product {
    name: String,
    price: u64,
    quntity: u64,
    expiry_date: Option<ExpirationDate>,
    shipping_info: Option<ShippingWeight>
}
