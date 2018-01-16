//! Updaters are the citadel component meant to update a record in place.
use citadel::system::DatabaseConnection;
pub trait Updater<T: DatabaseConnection> {
    fn update(&self, connection: &T) -> bool;
}