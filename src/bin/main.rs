#[macro_use]
extern crate rocket;



use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use diesel::{insert_into, ExpressionMethods, RunQueryDsl, QueryResult, QueryDsl};
use diesel::result::Error;
use diesel::result::Error::RollbackTransaction;
use std::borrow::Borrow;
use rocket::http::Status;
use warehouse_rs::controllers::product_category;
use warehouse_rs::DbConn;



fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/productcategory", routes![product_category::get, product_category::get_all, product_category::post])
        .launch();
}