use relationship::gender::Gender;

/// People are modeled with characteristics which define them.
/// A relationship consists of two (or more) people.
#[derive(Queryable)]
pub struct Person {
    pub person_id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_address: String,
    pub age: Option<i32>,
    pub gender: Option<String>
}