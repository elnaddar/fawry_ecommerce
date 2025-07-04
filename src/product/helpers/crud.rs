use std::collections::HashMap;

use crate::product::structs::Product;
use crate::traits::crud::*;

const PRODUCTS_DATA_PATH: &str = "products.json";

impl Crud<String, Product> for Product {
    fn init_storage() -> Result<Success, Error> {
        let path = std::path::Path::new(PRODUCTS_DATA_PATH);
        match path.exists() {
            true => Err(Error {
                msg: "Couldn't Initialize products file.".to_string(),
            }),
            false => match std::fs::write(path, "{}") {
                Ok(_) => Ok(Success {
                    msg: "Initialized products file successfully".to_string(),
                }),
                Err(err) => Err(Error {
                    msg: err.to_string(),
                }),
            },
        }
    }

    fn get_all() -> Result<HashMap<String, Product>, Error> {
        let path = std::path::Path::new(PRODUCTS_DATA_PATH);
        match std::fs::read_to_string(path) {
            Ok(str) => match serde_json::from_str(&str) {
                Ok(map) => Ok(map),
                Err(err) => Err(Error {
                    msg: err.to_string(),
                }),
            },
            Err(err) => Err(Error {
                msg: err.to_string(),
            }),
        }
    }

    fn get(key: &String) -> Result<Product, Error> {
        match Product::get_all() {
            Ok(products) => match products.get(key) {
                Some(product) => Ok(product.clone()),
                None => Err(Error {
                    msg: "Not Found".to_string(),
                }),
            },
            Err(err) => Err(err),
        }
    }

    fn create(product: Product) -> Result<Success, Error> {
        match Self::get_all() {
            Ok(mut data) => {
                data.insert(product.name.clone(), product);
                match Self::write_all(data) {
                    Ok(_) => Ok(Success {
                        msg: "Created Successfully".to_string(),
                    }),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }

    fn update(key: &String, product: Product) -> Result<Success, Error> {
        match Self::get(key) {
            Ok(_) => Self::create(product),
            Err(err) => Err(err),
        }
    }

    fn delete(key: &String) -> Result<Success, Error> {
        match Self::get_all() {
            Ok(mut data) => {
                if data.remove(key).is_some() {
                    match Self::write_all(data) {
                        Ok(_) => Ok(Success {
                            msg: "Deleted Successfully".to_string(),
                        }),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(Error {
                        msg: "Not Found".to_string(),
                    })
                }
            }
            Err(err) => Err(err),
            
        }
    }

    fn write_all(data: HashMap<String, Product>) -> Result<Success, Error> {
        let path = std::path::Path::new(PRODUCTS_DATA_PATH);
        match serde_json::to_string(&data) {
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
}
