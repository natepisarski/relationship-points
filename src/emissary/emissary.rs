/// Emissaries are data-transfer objects. They're designed to move data into a semi-permanent cache.
/// This could be the store of a Redux application, or just in-memory cache in a program.
///
/// Emissaries all have a key, the data, and a list of callbacks.
///
/// The key is a namespaced identifier for the root of the data. 'users.current' may be a key
/// for storing a user object.
///
/// The data is the data, which must be able to be serialized, that will be stored where the key
/// specifies.
///
/// The callbacks are strings, which have named variables with either ~ or @. These are the most complex
/// and variable part of the system. A callback like 'http://server.com/api/getUser?userName=~"users.current.email"'
/// will pull a stored emissary value and use it. A callback like 'http://server.com/api/getUser?userName=@"userName"'
/// will simply create a string where the variable must be filled in.
///
/// Callbacks do not need to be URLs. In a CQRS model, it could feed the string which initiates a query / command.

use std::vec::Vec;

extern crate serde;
extern crate serde_json;

/// Emissary is the trait that allows data to be transfered. It contains the data, whatever it may be,
/// a serialization key, and a list of callbacks.
pub trait Emissary {

    /// Whatever type of data is being stored
    type DataType;

    /// Where in a stateful cache this data is stored (i.e "accounts" "user.details")
    fn serialization_key(&self, Self::DataType) -> String;

    /// A list of callbacks (i.e "@'application.baseUrl'/Accounts/GetAccount?userName=@'user.email')
    fn get_callbacks(&self) -> Vec<String>;

    fn data_transfer_type(&self) -> &'static str;
}

/// An emissary container is a structure that makes wrapping up one-off data structures as an
/// emissary easier. If you provide it with data, a key, and a list of callbacks (normally empty)
/// it will structure whatever the data is in the emissary format.
#[derive(Serialize, Debug)]
pub struct EmissaryContainer<T: serde::Serialize> {

    /// The data, as long as it can be serialized.
    pub data: T,

    /// The serialization key. This creates the key property when serialized.
    pub key: String,

    /// A list of callbacks that this data can use to communicate to the layer sending it.
    pub callbacks: Vec<String>
}

/// All emissary containers are de-facto Emissary objects.
impl<T: serde::Serialize> Emissary for EmissaryContainer<T> {

    type DataType = T;

    fn serialization_key(&self, var: T) -> String {
        self.key.clone()
    }

    fn get_callbacks(&self) -> Vec<String> {
        self.callbacks.clone()
    }

    fn data_transfer_type(&self) -> &'static str {
        "EMISSARY"
    }
}

/// Given an emissary, serialize it with default serde serialization (bound to change)
pub fn serialize_emissary<T: Emissary + serde::Serialize>(emissary: T) -> String {
    match serde_json::to_string(&emissary) {
        Ok(val) => val,
        Err(err) => String::new()
    }
}

/// Given enough data to make an emissary, this creates an emissary container which can immediately
/// be serialized.
pub fn create_emissary<T: serde::Serialize>(emissary_key: String, inner_data: T) -> EmissaryContainer<T> {
    EmissaryContainer {
        data: inner_data,
        key: emissary_key.clone(),
        callbacks: vec![]
    }
}