# Citadel System

## Proper import support
Right now, each CRUD component needs to AT LEAST import `citadel::system::DatabaseConnection`, `citadel::*`, `citadel::connections::sqlite_connection` (or whichever connection being used). It would be handy to have a `citadel::components` for CRUD components, and `citadel::system` for re-exporting everything else.

## Database-Connection agnostic components (1.0 requirement)
Currently, despite having an abstract DatabaseConnection trait, CRUD components are implemented against a discrete connection (i.e SqliteConnection). 

The only function that the `DatabaseConnection` reference serves is to provide a connection on which diesel can be run against. As long as something returning a diesel Connection can be used, this can work.

## Macros with codegen (1.0 requirement)
Macros make the citadel system much more fluid. At the very least, the macros need to be able to:
* Create CRUD components 

## Creators inconsistant with the rest of the components
Creators should rely on the construction of their struct rather than an argument supplied to the call site. i.e, insert gains the signature `fn insert(&self, connection: &T) -> bool`

# Emissary System
No proper complaints on emissary at the moment, but they will come when end-to-end inches closer.

# Practice Project Structure

## Unclog root relationship namespace
As it stands, many things are just jammed into this namespace. There needs to be a better way to separate concerns, even possibly encompassing some CQRS-esque design pattern, although an existing solution for this would be preferable

# Project Steps
## Codification of Supporting Packages
Emissary and Citadel, with tests