use super::schema::*;

#[derive(Queryable, Identifiable)]
#[table_name = "product"]
/// A product which is sold by the company
///
/// The product represents only the description of product. For example "A Book - FirstName Last"
/// represents only that we are aware of this product, while separate entities are used to represent
/// other information, such as how much stock of this product do we have, or which competitor too
/// sells this product.
pub struct Product {
    id: i32,
    description: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Product, foreign_key = "product_id")]
#[belongs_to(
    crate::product_category::entities::ProductCategory,
    foreign_key = "product_category_id"
)]
#[table_name = "product_category_classification"]
/// Classification of products into categories
///
/// Specifies to which categories does a product belong.
pub struct ProductCategoryClassification {
    id: i32,
    product_id: i32,
    product_category_id: i32,
    is_primary_classification: bool,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "product_category_rollup"]
/// Hierarchy of product categories
///
/// Represents how categories can consists of other subcategories. For example "Clothes" category
/// can contain "Bikinis", "Shirts", "Swimwear" as subcategories. One category can have many parent
/// categories.
///
/// Note category should not be related to children of its children. Basically the relationship of
/// categories should not be transitive.
pub struct ProductCategoryRollup {
    id: i32,
    upper_category_id: i32,
    lower_category_id: i32,
}

#[derive(Queryable, Identifiable)]
#[table_name = "inventory_item"]
/// Instance of product
///
/// Represents an instance of product. For example if we have product "A" which is a book, then we
/// can have 10 inventory items, which are product "A", basically we can have 10 copies of book in
/// stock.
///
/// Note currently this type represents a serializable product. This means that each instance of a
/// product is considered to be unique. There is currently no concept of "lot" or "stack" of product
pub struct InventoryItem {
    id: i32,
    product_id: i32,
    instance_description: String,
}
