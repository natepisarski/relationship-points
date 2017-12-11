use citadel::system::DatabaseConnection;
use citadel::*;
use citadel::connections::sqlite_connection;

use diesel::*;
use relationship::person;

use super::super::schema::Person;

pub struct PersonCreator {

}

impl creator::Creator<sqlite_connection::SqliteConnection, person::PersonModel> for PersonCreator {
    fn insert(&self, db_connection: &sqlite_connection::SqliteConnection, record: person::PersonModel) -> bool {
        let connection_copy = db_connection.raw_connection();
        let connection_reference = connection_copy.as_ref();

        insert_into(Person::table)
            .values(&record)
            .execute(connection_reference);
        true
    }
}