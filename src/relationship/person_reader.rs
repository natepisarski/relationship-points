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

impl Reader<sqlite_connection::SqliteConnection, Option<person::Person>> for PersonReader {
    fn read(&self, connection: &sqlite_connection::SqliteConnection) -> Option<person::Person> {
        use super::super::schema::Person::dsl::*;

        let connection = connection.raw_connection();
        let usable_connection = connection.as_ref();

        if let PersonReaderSearchCriteria::FirstName(ref name) = self.criteria {
            return Some(Person
                .filter(FirstName.eq(name))
                .first(usable_connection)
                .unwrap());
        } else if let PersonReaderSearchCriteria::LastName(ref name) = self.criteria {
            panic!("");//return Person.filter(LastName.eq(name)).limit(1).load
        } else {
            panic!("No person with the given name found!");
        }
    }
}


        /*
        let matching_statement = match self.criteria {
            PersonReaderSearchCriteria::FirstName(name) => Person.filter(FirstName.eq(name)).limit(1),
            PersonReaderSearchCriteria::LastName(name) => Person.filter(LastName.eq(name)).limit(1),
            PersonReaderSearchCriteria::EmailAddress(email) => Person.filter(EmailAddress.eq(email)).limit(1)
        };
        let results = matching_statement.limit(1).load::<person::Person>(&connection.raw_connection().as_ref());
        let final_result = results.head();
        return final_result;*/