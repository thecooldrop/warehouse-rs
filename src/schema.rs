table! {
    inventory_item (id) {
        id -> Int4,
        product_id -> Int4,
        instance_description -> Nullable<Text>,
        warehouse_id -> Nullable<Int4>,
    }
}

table! {
    product (id) {
        id -> Int4,
        description -> Varchar,
    }
}

table! {
    product_category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    product_category_classification (id) {
        id -> Int4,
        product_id -> Int4,
        product_category_id -> Int4,
        is_primary_classification -> Bool,
    }
}

table! {
    product_category_rollup (id) {
        id -> Int4,
        upper_category_id -> Int4,
        lower_category_id -> Int4,
    }
}

table! {
    warehouse (id) {
        id -> Int4,
        description -> Text,
    }
}

joinable!(inventory_item -> product (product_id));
joinable!(inventory_item -> warehouse (warehouse_id));
joinable!(product_category_classification -> product (product_id));
joinable!(product_category_classification -> product_category (product_category_id));

allow_tables_to_appear_in_same_query!(
    inventory_item,
    product,
    product_category,
    product_category_classification,
    product_category_rollup,
    warehouse,
);
