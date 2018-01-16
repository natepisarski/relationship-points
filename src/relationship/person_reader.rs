use citadel::system::DatabaseConnection;
use citadel::reader::Reader;
use citadel::connections::sqlite_connection;

use diesel::*;
use relationship::person;

use relationship::person_search_criteria::*;

pub struct PersonReader {
    pub criteria: PersonSearchCriteria
}

impl Reader<sqlite_connection::SqliteConnection, Option<person::PersonModel>> for PersonReader {
    fn read(&self, connection: &sqlite_connection::SqliteConnection) -> Option<person::PersonModel> {
        use super::super::schema::Person::dsl::*;

        let connection = connection.raw_connection();
        let usable_connection = connection.as_ref();

        match self.criteria {
            PersonSearchCriteria::FirstName(ref name) =>
                Some(Person
                .filter(FirstName.eq(name))
                .first(usable_connection)
                .unwrap()),
            PersonSearchCriteria::LastName(ref name) =>
                Some(Person
                    .filter(LastName.eq(name))
                    .first(usable_connection)
                    .unwrap()),
            PersonSearchCriteria::EmailAddress(ref email) =>
                Some(Person
                    .filter(EmailAddress.eq(email))
                    .first(usable_connection)
                    .unwrap())
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