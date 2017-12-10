use citadel::system::DatabaseConnection;
use diesel::prelude::*;
use diesel::sqlite;
use std::rc::Rc;

pub struct SqliteConnection {
    pub connection_file_path: String,
    connection: Rc<sqlite::SqliteConnection>
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