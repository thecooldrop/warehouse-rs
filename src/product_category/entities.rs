use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Insertable, Queryable, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[table_name = "product_category"]
/// Product category
///
/// Represents possible grouping of products. Examples of products would be "Books", "Clothes"
/// or "Office supplies"
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
}
