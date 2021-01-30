use crate::entities::ProductCategory;
use rocket_contrib::json::Json;
use crate::DbConn;
use diesel::{ExpressionMethods, insert_into, RunQueryDsl, QueryDsl};
use std::borrow::Borrow;
use serde::{Serialize, Deserialize};
use crate::controllers::{GetResponder, PostResponder};

#[derive(Serialize, Deserialize)]
pub struct ProductCategoryRequestBody{
    pub name : String
}


#[get("/")]
pub fn get_all(db_conn: DbConn) -> Json<Vec<ProductCategory>> {
    use crate::schema::product_category::dsl::*;
    let product_categories = product_category.load(&db_conn.0).unwrap();
    Json(product_categories)
}


#[get("/<product_category_id>")]
pub fn get(db_conn: DbConn, product_category_id: u32) -> GetResponder<ProductCategory> {
    use crate::schema::product_category::dsl::*;
    if let Ok(category_by_id) = product_category.find(product_category_id as i32).first(&db_conn.0) {
        return GetResponder::Found(Json(category_by_id));
    }
    GetResponder::NotFound(())
}

#[post("/", format="json", data="<new_product_category>")]
pub fn post(db_conn: DbConn, new_product_category: Json<ProductCategoryRequestBody>) -> PostResponder<ProductCategory> {
    use crate::schema::product_category::dsl::*;

    let product_category_name = new_product_category.into_inner().name;

    let product_categories_with_name: Vec<ProductCategory> = product_category
        .filter(name.eq(&product_category_name))
        .load(&db_conn.0)
        .unwrap();

    if !product_categories_with_name.is_empty() {
        return PostResponder::Existed(Json(product_categories_with_name.into_iter().nth(0).unwrap()));
    }

    let inserted_id = insert_into(product_category)
        .values(name.eq(&product_category_name))
        .returning(id)
        .get_result(db_conn.0.borrow())
        .unwrap();

    let inserted_product_category = ProductCategory{
        id: inserted_id,
        name: product_category_name
    };
    PostResponder::Created(Json(inserted_product_category))
}

#[cfg(test)]
mod tests {

    use testcontainers::{images, Docker, Container, clients::Cli, core::Port, images::postgres::Postgres};
    use std::collections::HashMap;
    use diesel::{PgConnection, insert_into, prelude::*};
    use crate::{DbConn, entities::ProductCategory};
    use super::{static_rocket_route_info_for_get,
                static_rocket_route_info_for_get_all,
                static_rocket_route_info_for_post,
                ProductCategoryRequestBody};
    use rocket::{Config, Rocket, config::Environment,http::{Header, Status}, local::Client};
    use std::cmp::Ordering;


    #[test]
    fn test_saves_product_category_into_db() -> Result<(), String> {
        use crate::schema::product_category::dsl::*;

        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut req = client.post("/productcategory");
        req.set_body("{\"name\":\"product\"}".to_string());
        req.add_header(Header::new("Content-Type", "application/json"));

        let mut response = req.dispatch();
        let response_body = response.body_string().unwrap();
        let response_status = response.status();
        let response_product_category : ProductCategory = serde_json::from_str(&response_body).unwrap();


        let categories_in_db : Vec<ProductCategory> = product_category
            .filter(name.eq("product"))
            .load::<ProductCategory>(&connection)
            .unwrap();

        assert_eq!(categories_in_db.len(), 1);
        assert_eq!(categories_in_db.get(0).unwrap().name, "product".to_string());
        assert_eq!(response_status, Status::Created);
        Ok(())
    }

    #[test]
    fn test_if_product_category_exists_in_db_then_post_returns_it(){
        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut req = client.post("/productcategory");
        req.set_body("{\"name\":\"product\"}".to_string());
        req.add_header(Header::new("Content-Type", "application/json"));


        use crate::schema::product_category::dsl::*;
        let inserted_id: i32 = insert_into(product_category)
            .values(name.eq("product"))
            .returning(id)
            .get_result(&connection)
            .unwrap();

        let mut response = req.dispatch();
        let response_body = response.body_string().unwrap();
        let response_status = response.status();
        let response_product_category = serde_json::from_str::<ProductCategory>(&response_body).unwrap();
        assert_eq!(inserted_id, response_product_category.id);
        assert_eq!("product".to_string(), response_product_category.name);
        assert_eq!(response_status, Status::Ok);
    }

    #[test]
    fn get_product_categories_returns_all_categories() {
        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata);
        let client = Client::new(rocket).expect("valid rocket instance");
        let get_product_categories_request = client.get("/productcategory");

        use crate::schema::product_category::dsl::*;
        insert_into(product_category)
            .values(&vec![name.eq("first_category".to_string()), name.eq("second_category".to_string())])
            .execute(&connection);

        let mut response = get_product_categories_request.dispatch();
        let response_body = response.body_string().unwrap();
        let response_status = response.status();

        let mut returned_product_categories: Vec<ProductCategory> = serde_json::from_str(&response_body).unwrap();
        returned_product_categories.sort_by(|a: &ProductCategory, b: &ProductCategory| -> Ordering {
            a.name.cmp(&b.name)
        });
        assert_eq!(2, returned_product_categories.len());
        assert_eq!("first_category".to_string(), returned_product_categories.get(0).unwrap().name);
        assert_eq!("second_category".to_string(), returned_product_categories.get(1).unwrap().name);
        assert_eq!(Status::Ok, response_status);
    }

    #[test]
    fn get_product_category_by_id_returns_correct_product_category() {
        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata);
        let client = Client::new(rocket).expect("valid rocket instance");

        use crate::schema::product_category::dsl::*;
        let inserted_product_categories: Vec<(i32, String)> = insert_into(product_category)
            .values(vec![name.eq("first"), name.eq("second")])
            .returning(product_category::all_columns())
            .get_results(&connection)
            .unwrap();


        let get_request = client.get(format!("/productcategory/{}", inserted_product_categories.get(0).unwrap().0));
        let mut response = get_request.dispatch();
        let response_body = response.body_string().unwrap();
        let response_status = response.status();
        let returned_product_category: ProductCategory = serde_json::from_str(&response_body).unwrap();

        let expected_product_category = ProductCategory {
            id: inserted_product_categories.get(0).unwrap().0,
            name: inserted_product_categories.get(0).unwrap().1.clone()
        };
        assert_eq!(returned_product_category, expected_product_category);
        assert_eq!(Status::Ok, response_status);
    }

    #[test]
    fn when_getting_missing_product_category_status_is_404() {
        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata);
        let client = Client::new(rocket).expect("valid rocket instance");

        let get_request = client.get(format!("/productcategory/{}", 999));
        let mut response = get_request.dispatch();
        let response_status = response.status();

        assert_eq!(Status::NotFound, response_status);
    }
}
