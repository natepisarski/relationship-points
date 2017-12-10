#![feature(plugin, custom_attribute, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

#[macro_use] extern crate diesel_infer_schema;

pub mod schema;
pub mod models;
pub mod emissary;
pub mod citadel;
pub mod test_value_inserter;

extern crate dotenv;

use diesel::prelude::*;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::time::{SystemTime};

use emissary::*;
use citadel::*;
extern crate serde;

use rocket_contrib::{json, Json, Value};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set!");
    SqliteConnection::establish(&database_url)
        .expect("ERROR CONNECTING TO DATABASE")
}

#[derive(Serialize, Deserialize)]
pub struct TestValueAndDate {
    pub test_value: String,
    pub time: SystemTime
}

#[get("/")]
fn index() -> String {
    use schema::*;
    use schema::TestTable::dsl::*;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("The environment variable DATABASE_URL must be set!");
    //let connection: diesel::Connection = diesel::connection::Connection::establish(&database_url).unwrap();
    let connection = establish_connection();
    let results = TestTable.limit(5)
        .load::<self::models::TestValue>(&connection)
        .expect("Error loading tests");
    let mut final_result: String = "".to_owned();
    for db_test_value in results {
        let value_as_emissary: emissary::emissary::EmissaryContainer<TestValueAndDate> =
            emissary::emissary::create_emissary("entities.test.values".to_owned(),
                            TestValueAndDate {
                                test_value: db_test_value.value,
                                time: SystemTime::now(),
                            }
            );
        final_result =  emissary::emissary::serialize_emissary(
            value_as_emissary
        );
    }
    return final_result;
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}