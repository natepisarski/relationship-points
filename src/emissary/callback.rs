use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

extern crate serde;
extern crate serde_json;
extern crate regex;

use self::emissary;

//! This is the file for interpreting emissary callbacks within backend code.
//! A callback has the form "text/'@callback name'/'@other_callback_name'"
//! The escape character \ can be used to either actually introduce a backslash, or to add a ' or @

/// Returns the name of the callbacks requested in the string
pub fn find_callback_requests() -> Vec<String> {

}