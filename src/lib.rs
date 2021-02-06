#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_migrations;

mod entities;
pub mod product_category;
pub mod schema;
mod test_utils;
pub mod utilities;

#[database("pgdatabase")]
pub struct DbConn(diesel::PgConnection);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
