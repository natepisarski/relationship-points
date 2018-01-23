use citadel_crud::system::DatabaseConnection;
use citadel_crud::*;
use citadel_crud::connections::sqlite_connection::SqliteConnection;
use citadel_crud::components::creator::Creator;

use diesel::*;
use relationship::person;

use super::super::schema::Person;

pub struct PersonCreator {

}

impl Creator<SqliteConnection, person::PersonModel> for PersonCreator {
    fn insert(&self, db_connection: &SqliteConnection, record: person::PersonModel) -> bool {
        let connection_copy = db_connection.raw_connection();
        let connection_reference = connection_copy.as_ref();

        insert_into(Person::table)
            .values(&record)
            .execute(connection_reference);
        true
    }
}