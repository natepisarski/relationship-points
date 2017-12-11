#![feature(plugin, custom_attribute, custom_derive)]
#![plugin(rocket_codegen)]
#![recursion_limit="1024"]

pub mod schema;
pub mod models;
pub mod emissary;
pub mod citadel;
pub mod relationship;
pub mod test_value_inserter;

extern crate rocket;
extern crate serde;
extern crate dotenv;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

#[macro_use] extern crate diesel_infer_schema;

use citadel::*;
use citadel::creator::Creator;
use citadel::system::DatabaseConnection;
use citadel::reader::Reader;
use citadel::connections::sqlite_connection;

use diesel::prelude::*;

use dotenv::dotenv;
use test_value_inserter::*;

use emissary::*;

use relationship::person::Person;
use relationship::person_reader::{PersonReaderSearchCriteria, PersonReader};

use rocket_contrib::{json, Json, Value};

use std::env;
use std::time::{SystemTime};

pub fn establish_connection() -> diesel::SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set!");
    diesel::SqliteConnection::establish(&database_url)
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

    let mut connection =
        sqlite_connection::SqliteConnection::new(&database_url);

    let inserter = TestValueInserter{};
    inserter.insert(&connection, String::from("THIS IS A BRAND SPANKIN NEW TEST VALUE"));

    let guy_named_john = (PersonReader {
        criteria: PersonReaderSearchCriteria::FirstName(String::from("John"))
    }).read(&connection);
    println!("Johns email is {}", guy_named_john.unwrap().email_address);

    let results = TestTable.limit(5)
        .load::<self::models::TestValue>(connection.raw_connection().as_ref())
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