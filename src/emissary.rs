/// Emissaries are data-transfer objects designed to convey data
/// from the backend to a frontend adapter. Pure rust models implementing Emissary
/// are automatically serialized into JSON, ready to be sent to be consumed by a frontend /
/// API consumer.
///
/// Emissaries are special data-transfer objects, designed to import data into a frontend datastore
/// of some sort. The only data required by emissary is a serialization_key, meant to be the key
/// that the root of the data imported wil be accessed by

use std::collections::HashMap;
use std::rc::Rc;


extern crate serde;
extern crate serde_json;

pub trait Emissary {
    type DataType;
    fn serialization_key(&self, Self::DataType) -> String;
}

#[serde_derive(Serialize, Debug)]
pub struct EmissaryContainer<T: serde::Serialize> {
    pub data: T,
    pub key: String
}

impl<T: serde::Serialize> Emissary for EmissaryContainer<T> {
    type DataType = T;
    fn serialization_key(&self, var: T) -> String {
        self.key.clone()
    }
}

pub fn serialize_emissary<T: Emissary + serde::Serialize>(emissary: T) -> String {
    match serde_json::to_string(&emissary) {
        Ok(val) => val,
        Err(err) => String::new()
    }
}