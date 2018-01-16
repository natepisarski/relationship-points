use citadel::system::DatabaseConnection;
use citadel::*;
use citadel::updater::Updater;
use citadel::connections::sqlite_connection;

use diesel;
use diesel::*;
use relationship::person;

use super::super::schema::Person;

use relationship::person_search_criteria::*;

pub struct PersonUpdater {
    pub change_criteria: (PersonSearchCriteria, PersonSearchCriteria)
}

impl Updater<sqlite_connection::SqliteConnection> for PersonUpdater {
    fn update(&self, connection: &sqlite_connection::SqliteConnection) -> bool {
        use super::super::schema::Person::dsl::*;

        let change_criteria = &self.change_criteria;
        let &(ref criteria, ref update) = change_criteria;

        let connection = connection.raw_connection();
        let usable_connection = connection.as_ref();

        let result = match criteria {
            &PersonSearchCriteria::FirstName(ref name) => match update {
                &PersonSearchCriteria::FirstName(ref updatedName) =>
                    diesel::update(Person.filter(FirstName.eq(name))).set(FirstName.eq(updatedName)).execute(usable_connection),
                &PersonSearchCriteria::EmailAddress(ref updatedEmail) =>
                    diesel::update(Person.filter(FirstName.eq(name))).set(EmailAddress.eq(updatedEmail)).execute(usable_connection),
                &PersonSearchCriteria::LastName(ref updatedLastName) =>
                    diesel::update(Person.filter(FirstName.eq(name))).set(LastName.eq(updatedLastName)).execute(usable_connection),
            },
            &PersonSearchCriteria::LastName(ref name) => match update {
                &PersonSearchCriteria::FirstName(ref updatedName) =>
                    diesel::update(Person.filter(LastName.eq(name))).set(FirstName.eq(updatedName)).execute(usable_connection),
                &PersonSearchCriteria::EmailAddress(ref updatedEmail) =>
                    diesel::update(Person.filter(LastName.eq(name))).set(EmailAddress.eq(updatedEmail)).execute(usable_connection),
                &PersonSearchCriteria::LastName(ref updatedLastName) =>
                    diesel::update(Person.filter(LastName.eq(name))).set(LastName.eq(updatedLastName)).execute(usable_connection),
            },
            &PersonSearchCriteria::EmailAddress(ref email) => match update {
                &PersonSearchCriteria::FirstName(ref updatedName) =>
                    diesel::update(Person.filter(EmailAddress.eq(email))).set(FirstName.eq(updatedName)).execute(usable_connection),
                &PersonSearchCriteria::EmailAddress(ref updatedEmail) =>
                    diesel::update(Person.filter(EmailAddress.eq(email))).set(EmailAddress.eq(updatedEmail)).execute(usable_connection),
                &PersonSearchCriteria::LastName(ref updatedLastName) =>
                    diesel::update(Person.filter(EmailAddress.eq(email))).set(LastName.eq(updatedLastName)).execute(usable_connection),
            }
        };

        match result {
            Ok(_r) => true,
            Err(_e) => false
        }
    }
}