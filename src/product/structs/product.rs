use serde::{Deserialize, Serialize};

use crate::product::structs::{ExpirationDate, ShippingWeight};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub price: u64,
    pub quntity: u64,
    pub expiry_date: Option<ExpirationDate>,
    pub shipping_info: Option<ShippingWeight>,
}
