use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RustValue{
    Number(f64),
    String(String),
    HashMap(HashMap<String, RustValue>),
    Vec(Vec<RustValue>),
    Bool(bool),
    None
}
pub trait Deserialize {
    fn deserialize(self) -> RustValue;
}
