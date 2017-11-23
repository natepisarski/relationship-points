#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate serde;

use rocket_contrib::{json, Json, Value};

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({"value": 5}))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}