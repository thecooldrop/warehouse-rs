use super::schema::*;
use diesel::Queryable;
use diesel::associations::BelongsTo;


#[derive(Queryable, Identifiable)]
#[table_name = "product"]
pub struct Product {
    id: i32,
    description: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Product, foreign_key = "product_id")]
#[belongs_to(RawProductCategory, foreign_key = "product_category_id")]
#[table_name = "product_category_classification"]
pub struct ProductCategoryClassification {
    id: i32,
    product_id: i32,
    product_category_id: i32,
    is_primary_classification: bool,
}

trait ProductCategory{
    fn new(id: i32, name: String) -> Self;
}

#[derive(Identifiable)]
#[table_name = "product_category"]
pub struct RawProductCategory {
    id: i32,
    name: String,
}

#[derive(Identifiable)]
#[table_name = "product_category"]
pub struct UpperProductCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable)]
#[table_name = "product_category"]
pub struct LowerProductCategory {
    pub id: i32,
    pub name: String
}

impl ProductCategory for RawProductCategory {
    fn new(id: i32, name: String) -> Self {
        RawProductCategory {
            id,
            name
        }
    }
}

impl ProductCategory for UpperProductCategory {
    fn new(id: i32, name: String) -> Self {
        UpperProductCategory {
            id,
            name
        }
    }
}

impl ProductCategory for LowerProductCategory {
    fn new(id: i32, name: String) -> Self {
        LowerProductCategory {
            id,
            name
        }
    }
}

impl Queryable<product_category::SqlType, diesel::pg::Pg> for RawProductCategory {
    type Row = (i32, String);
    fn build(row: Self::Row) -> Self {
        ProductCategory::new(row.0, row.1)
    }
}

impl Queryable<product_category::SqlType, diesel::pg::Pg> for UpperProductCategory {
    type Row = (i32, String);
    fn build(row: Self::Row) -> Self {
        ProductCategory::new(row.0, row.1)
    }
}

impl Queryable<product_category::SqlType, diesel::pg::Pg> for LowerProductCategory {
    type Row = (i32, String);
    fn build(row: Self::Row) -> Self {
        ProductCategory::new(row.0, row.1)
    }
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(UpperProductCategory, foreign_key="upper_category_id")]
#[belongs_to(LowerProductCategory, foreign_key="lower_category_id")]
#[table_name = "product_category_rollup"]
pub struct ProductCategoryRollup {
    id: i32,
    upper_category_id: i32,
    lower_category_id: i32,
}

#[derive(Queryable, Identifiable)]
#[table_name = "inventory_item"]
pub struct InventoryItem {
    id: i32,
    product_id: i32,
    instance_description: String,
}