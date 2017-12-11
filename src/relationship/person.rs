use super::super::schema::Person;
/// People are modeled with characteristics which define them.
/// A relationship consists of two (or more) people.
#[derive(Queryable, Insertable)]
#[table_name="Person"]
pub struct PersonModel {

    #[column_name(PersonId)]
    pub person_id: Option<i32>,

    #[column_name(FirstName)]
    pub first_name: Option<String>,

    #[column_name(LastName)]
    pub last_name: Option<String>,

    #[column_name(EmailAddress)]
    pub email_address: String,

    #[column_name(Age)]
    pub age: Option<i32>,

    #[column_name(Gender)]
    pub gender: Option<String>
}