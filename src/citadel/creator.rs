//! Creator is a citadel trait that is implemented in order to write new data into the database.

use citadel::system::DatabaseConnection;

pub trait Creator<T: DatabaseConnection, W> {
    fn insert(&self, connection: T, insertion_object: W) -> bool;
}