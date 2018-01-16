use citadel::system::DatabaseConnection;
use citadel::*;
use citadel::deleter::Deleter;
use citadel::connections::sqlite_connection;

use diesel;
use diesel::*;
use relationship::person;

use super::super::schema::Person;

use relationship::person_search_criteria::*;

pub struct PersonDeleter {
    pub criteria: PersonSearchCriteria
}

impl Deleter<sqlite_connection::SqliteConnection> for PersonDeleter {
    fn delete(&self, connection: &sqlite_connection::SqliteConnection) -> bool {
        use super::super::schema::Person::dsl::*;

        let connection = connection.raw_connection();
        let usable_connection = connection.as_ref();

        let result = match self.criteria {
            PersonSearchCriteria::FirstName(ref name) =>
                diesel::delete(Person.filter(FirstName.eq(name))).execute(usable_connection),
            PersonSearchCriteria::LastName(ref name) =>
                diesel::delete(Person.filter(LastName.eq(name))).execute(usable_connection),
            PersonSearchCriteria::EmailAddress(ref email) =>
                diesel::delete(Person.filter(EmailAddress.eq(email))).execute(usable_connection),
        };
        match result {
            Ok(r) => true,
            Err(e) => false
        }
    }
}