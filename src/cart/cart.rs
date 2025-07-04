use std::collections::HashMap;

use clap::builder::Str;
use serde::{Deserialize, Serialize};

use crate::{
    cart::{self, CartItem},
    product::structs::Product,
    traits::crud::{Crud, Error, Success},
};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Cart {
    balance: u64,
    items: HashMap<String, CartItem>,
}

const CART_DATA_PATH: &str = "cart.json";

impl Cart {
    pub fn new() -> Self {
        Self {
            balance: 1000,
            items: HashMap::new(),
        }
    }

    pub fn init_storage() -> Result<Success, Error> {
        let path = std::path::Path::new(CART_DATA_PATH);
        match path.exists() {
            true => Err(Error {
                msg: "Couldn't Initialize cart file.".to_string(),
            }),
            false => match std::fs::write(path, "{\"balance\":1000,\"items\":{}}") {
                Ok(_) => Ok(Success {
                    msg: "Initialized cart file successfully".to_string(),
                }),
                Err(err) => Err(Error {
                    msg: err.to_string(),
                }),
            },
        }
    }

    pub fn get() -> Result<Cart, Error> {
        let path = std::path::Path::new(CART_DATA_PATH);
        match std::fs::read_to_string(path) {
            Ok(str) => match serde_json::from_str(&str) {
                Ok(cart) => Ok(cart),
                Err(err) => Err(Error {
                    msg: err.to_string(),
                }),
            },
            Err(err) => Err(Error {
                msg: err.to_string(),
            }),
        }
    }

    pub fn get_item(name: &str) -> Result<CartItem, Error> {
        match Self::get() {
            Ok(cart) => match cart.items.get(name) {
                Some(item) => Ok(item.clone()),
                None => Err(Error {
                    msg: "Notfound".to_string(),
                }),
            },
            Err(err) => Err(err),
        }
    }

    pub fn write_all(cart: Cart) -> Result<Success, Error> {
        let path = std::path::Path::new(CART_DATA_PATH);
        match serde_json::to_string(&cart) {
            Ok(json) => match std::fs::write(path, json) {
                Ok(_) => Ok(Success {
                    msg: "Written Successfully".to_string(),
                }),
                Err(err) => Err(Error {
                    msg: err.to_string(),
                }),
            },
            Err(err) => Err(Error {
                msg: err.to_string(),
            }),
        }
    }

    pub fn _add_item_helper(item: CartItem) -> Result<Success, Error> {
        let product = Product::get(&item.name).unwrap();
        match Self::get() {
            Ok(mut cart) => {
                if item.quantity > product.quntity {
                    Err(Error {
                        msg: "Not enough quantity".to_string(),
                    })
                } else {
                    cart.items.insert(item.name.clone(), item);
                    match Self::write_all(cart) {
                        Ok(_) => Ok(Success {
                            msg: "Added Successfully".to_string(),
                        }),
                        Err(err) => Err(err),
                    }
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn add_item(item: CartItem) -> Result<Success, Error> {
        match Self::get_item(&item.name) {
            Ok(mut old_item) => {
                old_item.quantity += item.quantity;
                Self::_add_item_helper(old_item)
            }
            Err(_) => Self::_add_item_helper(item),
        }
    }

    pub fn delete(name: &str) -> Result<Success, Error> {
        match Self::get() {
            Ok(mut cart) => match cart.items.remove(name) {
                Some(_) => match Self::write_all(cart) {
                    Ok(_) => Ok(Success {
                        msg: "removed item successfully".to_string(),
                    }),
                    Err(err) => Err(err),
                },
                None => Err(Error {
                    msg: "Notfound".to_string(),
                }),
            },
            Err(err) => Err(err),
        }
    }
}

impl Cart {
    pub fn checkout() {
        if let Ok(cart) = Self::get() {
            let mut subtotal: u64 = 0;
            let mut shipping: f64 = 0f64;
            let mut shipment_notice = String::new();
            let mut checkout_recipt = String::new();

            shipment_notice.push_str("** Shipment notice **\n");
            checkout_recipt.push_str("** Checkout receipt **\n");

            for (_k, v) in cart.items {
                subtotal += v.price * v.quantity;

                let recipt_line = format!(
                    "{}x {}{:<10}{:<5}\n",
                    v.quantity,
                    v.name.clone(),
                    "",
                    v.quantity * v.price
                );
                checkout_recipt.push_str(&recipt_line);

                if let Some(exp) = v.expiry_date
                    && exp.is_expired()
                {
                    panic!("{} item is expired", v.name)
                }

                if let Some(ship) = v.shipping_info {
                    shipping += v.quantity as f64 * ship.get_in_f64();
                    let ship_line = format!(
                        "{}x {}{:<10}{:.2}g\n",
                        v.quantity,
                        v.name.clone(),
                        "",
                        v.quantity as f64 * ship.get_in_f64()
                    );
                    shipment_notice.push_str(&ship_line);
                }

                shipment_notice.push_str(&format!("Total package weight {shipping}g\n"));
                shipping = 50f64;

                println!("{shipment_notice}");
                println!("{checkout_recipt}");
                println!("{:-<20}", "");
                println!("{:<15}{:<5}", "Subtotal", subtotal);
                println!("{:<15}{:.2}", "Shipping", shipping);
                println!("{:<15}{:.2}", "Total", shipping + subtotal as f64);
            }
        } else {
            println!("Error");
        }
    }
}
