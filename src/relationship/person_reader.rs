use citadel::system::DatabaseConnection;
use citadel::reader::Reader;
use citadel::connections::sqlite_connection;

use diesel::*;
use relationship::person;

use super::super::schema::Person;

pub enum PersonReaderSearchCriteria {
    FirstName(String),
    LastName(String),
    EmailAddress(String)
}

pub struct PersonReader {
    pub criteria: PersonReaderSearchCriteria
}

impl Reader<sqlite_connection::SqliteConnection, person::Person> for PersonReader{
    fn read(&self, connection: &sqlite_connection::SqliteConnection) -> person::Person {
        use super::super::schema::Person::dsl::*;

        let connection = connection.raw_connection();
        let usable_connection = connection.as_ref();
        if let PersonReaderSearchCriteria::FirstName(ref name) = self.criteria {
            return Person.filter(FirstName.eq(name)).limit(1).load::<person::Person>(usable_connection).unwrap().pop().unwrap();
        } else {
            panic!("No person with the given name found!");
        }
        /*
        let matching_statement = match self.criteria {
            PersonReaderSearchCriteria::FirstName(name) => Person.filter(FirstName.eq(name)),
            PersonReaderSearchCriteria::LastName(name) => Person.filter(LastName.eq(name)),
            PersonReaderSearchCriteria::EmailAddress(email) => Person.filter(EmailAddress.eq(email))
        };
        let results = matching_statement.limit(1).load::<person::Person>(&connection.raw_connection().as_ref());
        let final_result = results.head();
        return final_result;*/
    }
}