use citadel::system::DatabaseConnection;
use citadel::creator::Creator;
use citadel::connections::sqlite_connection::SqliteConnection;
use diesel;

use diesel::*;

use super::schema::TestTable;

#[derive(Insertable)]
#[table_name="TestTable"]
struct TestValue {
    TestId: i32,
    Value: String
}
pub struct TestValueInserter {
}


impl Creator<SqliteConnection, String> for TestValueInserter {
    fn insert(&self, connection: &SqliteConnection, to_insert: String) -> bool {
        let connection_copy = connection.raw_connection();
        let connection_reference: &diesel::SqliteConnection = connection_copy.as_ref();

        let to_insert: TestValue = (TestValue{TestId: 4, Value: to_insert});

        insert_into(TestTable::table)
            .values(&to_insert)
            .execute(connection_reference);
        true
    }
}