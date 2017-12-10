use citadel::system::DatabaseConnection;
use diesel::prelude::*;
use diesel::sqlite;
use diesel;
use std::rc::Rc;

pub struct SqliteConnection {
    pub connection_file_path: String,
    connection: Rc<sqlite::SqliteConnection>
}

impl SqliteConnection {
    pub fn new(database_connection_string: &str) -> SqliteConnection {
        SqliteConnection{
            connection_file_path:  String::from(database_connection_string),
            connection: Rc::new(diesel::SqliteConnection::establish(database_connection_string).unwrap())
        }
    }
}
impl DatabaseConnection for SqliteConnection {
    type ConnectionType = sqlite::SqliteConnection;
    fn establish_connection(&mut self) -> () {
        self.connection =
            Rc::new(sqlite::SqliteConnection::establish(&self.connection_file_path)
            .expect(
                &format!("[SqliteConnection] Cannot connect to file: {}",
                         &self.connection_file_path)
            ))
    }

    fn end_connection(&mut self) -> () {
        // [TODO|]
    }

    fn raw_connection(&self) -> Rc<sqlite::SqliteConnection> {
        self.connection.clone()
    }
}