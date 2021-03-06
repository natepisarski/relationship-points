#![feature(plugin, custom_attribute, custom_derive)]
#![plugin(rocket_codegen)]
#![recursion_limit="1024"]

pub mod schema;
pub mod models;
pub mod emissary;
pub mod relationship;
pub mod test_value_inserter;

extern crate rocket;
extern crate serde;
extern crate dotenv;

extern crate rocket_contrib;
extern crate citadel_crud;

extern crate rocket_cors;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

#[macro_use] extern crate diesel_infer_schema;

use citadel_crud::components::creator::Creator;
use citadel_crud::system::DatabaseConnection;
use citadel_crud::components::reader::Reader;
use citadel_crud::connections::sqlite_connection;

use diesel::prelude::*;

use dotenv::dotenv;
use test_value_inserter::*;

use relationship::person_search_criteria::PersonSearchCriteria;
use relationship::person_reader::{PersonReader};

use std::env;
use std::time::{SystemTime};

use rocket::request::FromRequest;
use rocket_contrib::Json;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use std::io::Cursor;


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
    use schema::TestTable::dsl::*;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("The environment variable DATABASE_URL must be set!");

    let mut connection =
        sqlite_connection::SqliteConnection::new(&database_url);

    let inserter = TestValueInserter{};
    inserter.insert(&connection, String::from("THIS IS A BRAND SPANKIN NEW TEST VALUE"));

    let guy_named_john = (PersonReader {
        criteria: PersonSearchCriteria::FirstName(String::from("John"))
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

#[derive(Serialize, Deserialize)]
pub struct EndpointProperty {
    id: u32,
    name: String
}

#[post("/endpoint", format = "application/json", data = "<endpoint_property>")]
fn endpoint(endpoint_property: Json<EndpointProperty>) -> String {
    let property: EndpointProperty = endpoint_property.into_inner();
    return format!("ID: {}, NAME: {}", property.id, property.name);
}

fn main() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allow_credentials: false,
        ..Default::default()
    };

    rocket::ignite().mount("/", routes![index, endpoint]).attach(options).launch();
}