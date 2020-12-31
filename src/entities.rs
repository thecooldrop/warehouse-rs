use super::schema::*;


#[derive(Queryable, Identifiable)]
#[table_name = "product"]
pub struct Product {
    id: i32,
    description: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Product, foreign_key = "product_id")]
#[belongs_to(ProductCategory, foreign_key = "product_category_id")]
#[table_name = "product_category_classification"]
pub struct ProductCategoryClassification {
    id: i32,
    product_id: i32,
    product_category_id: i32,
    is_primary_classification: bool,
}

#[derive(Identifiable, Queryable)]
#[table_name = "product_category"]
pub struct ProductCategory {
    id: i32,
    name: String,
}

#[derive(Identifiable, Queryable)]
#[table_name = "product_category"]
pub struct UpperProductCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(UpperProductCategory, foreign_key="upper_category_id")]
#[belongs_to(ProductCategory, foreign_key="lower_category_id")]
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

impl From<UpperProductCategory> for ProductCategory {
    fn from(upper_category: UpperProductCategory) -> Self {
        ProductCategory {
            id: upper_category.id,
            name: upper_category.name
        }
    }
}