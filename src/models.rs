#[derive(Queryable)]
pub struct TestValue {
    pub test_id: Option<i32>,
    pub value: String
}