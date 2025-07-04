use std::fmt::{self, write};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct ShippingWeight(f64);

impl ShippingWeight {
    pub fn new(weight: f64) -> Self {
        Self(weight)
    }
    pub fn get_in_f64(&self) -> f64 {
        self.0
    }

    pub fn calculate_price(&self) -> f64 {
        self.0 * 1.5
    }
}

impl fmt::Display for ShippingWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} gm", self.0)
    }
}
