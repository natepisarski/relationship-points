#![feature(plugin, custom_attribute, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod models;
pub mod emissary;

extern crate dotenv;

use diesel::prelude::*;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use emissary::*;
extern crate serde;

use rocket_contrib::{json, Json, Value};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set!");
    SqliteConnection::establish(&database_url)
        .expect("ERROR CONNECTING TO DATABASE")
}
#[get("/")]
fn index() -> Json<Value> {
    use schema::*;
    use schema::TestTable::dsl::*;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("The environment variable DATABASE_URL must be set!");
    //let connection: diesel::Connection = diesel::connection::Connection::establish(&database_url).unwrap();
    let connection = establish_connection();
    let results = TestTable.limit(5)
        .load::<self::models::TestValue>(&connection)
        .expect("Error loading tests");
    return Json(json!({"value": results.len()}))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}