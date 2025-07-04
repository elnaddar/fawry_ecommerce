use std::collections::HashMap;

#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

pub struct Success {
    pub msg: String,
}

pub trait Crud<K, V> {
    fn init_storage() -> Result<Success, Error>;
    fn write_all(v: HashMap<K, V>) -> Result<Success, Error>;
    fn get_all() -> Result<HashMap<K, V>, Error>;
    fn get(key: &K) -> Result<V, Error>;
    fn create(t: V) -> Result<Success, Error>;
    fn update(key: &K, v: V) -> Result<Success, Error>;
    fn delete(key: &K) -> Result<Success, Error>;
}
