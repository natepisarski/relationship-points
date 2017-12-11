//! Readers are the citadel component meant to read records out from the database.
//! Their generic type is meant to demonstrate what kind of data is returned.
use citadel::system::DatabaseConnection;

pub trait Reader<T: DatabaseConnection, W> {
    fn read(&self, connection: &T) -> W;
}