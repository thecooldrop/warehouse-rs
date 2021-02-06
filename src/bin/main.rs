#[macro_use]
extern crate rocket;

use diesel::result::Error;
use diesel::result::Error::RollbackTransaction;
use diesel::{insert_into, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use warehouse_rs::product_category::controllers;
use warehouse_rs::DbConn;

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount(
            "/productcategory",
            routes![
                product_category::get,
                product_category::get_all,
                product_category::post
            ],
        )
        .launch();
}
