//! Deleter is a citadel trait designed to remove data from a database. This can either be a whole
//! table or just data within a table

use citadel::system::DatabaseConnection;

pub trait Deleter<T: DatabaseConnection> {
    fn delete(&self, connection: &T) -> bool;
}