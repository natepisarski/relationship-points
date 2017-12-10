use diesel::Connection;
use std::rc::Rc;

/// A database connection is the basic type for all database connections.
/// It requires only that the implementor provides access to
pub trait DatabaseConnection {
    type ConnectionType: Connection;

    fn establish_connection(&mut self);
    fn end_connection(&mut self) -> ();
    fn raw_connection(&self) -> Rc<Self::ConnectionType>;
}