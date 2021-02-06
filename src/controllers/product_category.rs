use rocket_contrib::json::Json;
use crate::DbConn;
use diesel::{ExpressionMethods, insert_into, RunQueryDsl, QueryDsl};
use serde::{Serialize, Deserialize};
use crate::controllers::{GetResponder, PostResponder};
use rocket::http::Status;
use self::entities::ProductCategory;

#[derive(Serialize, Deserialize)]
pub struct ProductCategoryRequestBody{
    pub name : String
}


#[get("/")]
pub fn get_all(db_conn: DbConn) -> Result<Json<Vec<ProductCategory>>, diesel::result::Error> {
    use crate::schema::product_category::dsl::*;
    let product_categories = product_category.load(&db_conn.0)?;
    Ok(Json(product_categories))
}


#[get("/<product_category_id>")]
pub fn get(db_conn: DbConn, product_category_id: i32) -> Result<GetResponder<ProductCategory>, diesel::result::Error> {
    use crate::schema::product_category::dsl::*;
    return match product_category.find(product_category_id).first(&db_conn.0) {
        Ok(category_by_id) => {
            Ok(GetResponder::Found(Json(category_by_id)))
        },
        Err(diesel::NotFound) => {
            Ok(GetResponder::NotFound(()))
        },
        Err(e) => Err(e),
    }
}

#[post("/", format="json", data="<new_product_category>")]
pub fn post(db_conn: DbConn, new_product_category: Json<ProductCategoryRequestBody>) -> Result<PostResponder<ProductCategory>, diesel::result::Error> {
    use crate::schema::product_category::dsl;

    let product_categories_with_name: Vec<ProductCategory> = dsl::product_category
        .filter(dsl::name.eq(&new_product_category.name))
        .load(&db_conn.0)?;

    match product_categories_with_name.into_iter().nth(0) {
        Some(first_category) => {
            return Ok(PostResponder::Existed(Json(first_category)))
        },
        None => {
            let (id, name) = insert_into(dsl::product_category)
                .values(dsl::name.eq(&new_product_category.name))
                .returning((dsl::id, dsl::name))
                .get_result(&db_conn.0)?;
            Ok(PostResponder::Created(Json(ProductCategory{id, name})))
        }
    }
}

#[delete("/<id>")]
pub fn delete(conn: DbConn, id: i32) -> Result<Status, diesel::result::Error> {
    use crate::schema::product_category::dsl;
    let connection = conn.0;
    diesel::delete(dsl::product_category)
        .filter(dsl::id.eq(id))
        .execute(&connection)?;
    Ok(Status::Ok)
}

#[put("/<id>", format="json", data="<put_category>")]
pub fn put(conn: DbConn, id: i32, put_category: Json<ProductCategoryRequestBody>) -> Result<Json<ProductCategory>, diesel::result::Error> {
    use crate::schema::product_category::dsl;
    let connection = conn.0;
    let new_product_category = ProductCategory{
        id,
        name: put_category.into_inner().name
    };
    diesel::insert_into(dsl::product_category)
        .values(&new_product_category)
        .on_conflict(dsl::id)
        .do_update()
        .set(dsl::name.eq(&new_product_category.name))
        .execute(&connection)?;
    Ok(Json(new_product_category))
}

pub(in crate) mod entities {
    use crate::schema::*;
    use serde::{Serialize, Deserialize};

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
}

#[cfg(test)]
mod tests {
    use super::{ProductCategoryRequestBody, entities::ProductCategory};
    use testcontainers::{clients::Cli};
    use diesel::prelude::*;
    use rocket::{http::{Header, Status, ContentType}, local::Client};
    use std::cmp::Ordering;
    use crate::schema::product_category::dsl::product_category;


    #[test]
    fn test_saves_product_category_into_db() -> Result<(), diesel_migrations::RunMigrationsError> {
        use crate::schema::product_category::dsl::*;

        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata)?;
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
        assert_eq!(categories_in_db.get(0).unwrap(), &response_product_category);
        assert_eq!(response_status, Status::Created);
        Ok(())
    }

    #[test]
    fn test_if_product_category_exists_in_db_then_post_returns_it() -> Result<(), diesel_migrations::RunMigrationsError>{
        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata)?;
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut req = client.post("/productcategory");
        req.set_body("{\"name\":\"product\"}".to_string());
        req.add_header(Header::new("Content-Type", "application/json"));


        use crate::schema::product_category::dsl::*;
        let inserted_id: i32 = diesel::insert_into(product_category)
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
        Ok(())
    }

    #[test]
    fn get_product_categories_returns_all_categories() -> Result<(), diesel_migrations::RunMigrationsError>{
        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata)?;
        let client = Client::new(rocket).expect("valid rocket instance");
        let get_product_categories_request = client.get("/productcategory");

        use crate::schema::product_category::dsl::*;
        diesel::insert_into(product_category)
            .values(&vec![name.eq("first_category".to_string()), name.eq("second_category".to_string())])
            .execute(&connection)?;

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
        Ok(())
    }

    #[test]
    fn get_product_category_by_id_returns_correct_product_category() -> Result<(), diesel_migrations::RunMigrationsError>{
        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata)?;
        let client = Client::new(rocket).expect("valid rocket instance");

        use crate::schema::product_category::dsl::*;
        let inserted_product_categories: Vec<(i32, String)> = diesel::insert_into(product_category)
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
        Ok(())
    }

    #[test]
    fn when_getting_missing_product_category_status_is_404() -> Result<(), diesel_migrations::RunMigrationsError> {
        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, _) = crate::test_utils::start_rocket_with_db(&database_metadata)?;
        let client = Client::new(rocket).expect("valid rocket instance");

        let get_request = client.get(format!("/productcategory/{}", 999));
        let response = get_request.dispatch();
        let response_status = response.status();

        assert_eq!(Status::NotFound, response_status);
        Ok(())
    }

    #[test]
    fn delete_call_removes_product_category_from_db() -> Result<(), diesel_migrations::RunMigrationsError>{
        use crate::schema::product_category::dsl::*;

        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata)?;
        let client = Client::new(rocket).expect("Valid rocket instance");

        let inserted_product_category_id: i32 = diesel::insert_into(product_category)
            .values(name.eq("testcat"))
            .returning(id)
            .get_result(&connection)
            .unwrap();

        let saved_product_categories: Vec<ProductCategory> = product_category.load(&connection).unwrap();
        assert_eq!(saved_product_categories.len(), 1);

        let delete_request = client.delete(format!("/productcategory/{}", inserted_product_category_id));
        let response = delete_request.dispatch();
        let response_status = response.status();

        let remaining_product_cateogries: Vec<ProductCategory> = product_category.load(&connection).unwrap();
        assert_eq!(response_status, Status::Ok);
        assert_eq!(remaining_product_cateogries.len(), 0);
        Ok(())
    }

    #[test]
    fn put_overwrites_product_category() -> Result<(), diesel_migrations::RunMigrationsError>{
        use crate::schema::product_category::dsl::*;

        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata)?;
        let client = Client::new(rocket).expect("Valid Rocket instance");

        diesel::insert_into(product_category)
            .values(name.eq("testcategory"))
            .execute(&connection)?;

        let inserted_product_categories: Vec<ProductCategory> = product_category.load(&connection).unwrap();
        assert_eq!(inserted_product_categories.len(), 1);
        let inserted_product_category = inserted_product_categories.get(0).unwrap();

        let replacement_product_category = ProductCategoryRequestBody {
            name: "putcategory".to_string()
        };

        let mut put_request = client.put(format!("/productcategory/{}", inserted_product_category.id));
        put_request.set_body(serde_json::to_string(&replacement_product_category).unwrap());
        put_request.add_header(ContentType::JSON);

        let mut response = put_request.dispatch();
        let response_status = response.status();
        let response_body = response.body_string().unwrap();
        let returned_product_category: ProductCategory = serde_json::from_str(&response_body).unwrap();

        let expected_product_category = ProductCategory{
            id: inserted_product_category.id,
            name: returned_product_category.name
        };

        if let Ok(in_db_category) = product_category
            .find(expected_product_category.id)
            .first::<ProductCategory>(&connection) {
            assert_eq!(in_db_category, expected_product_category)
        }

        assert_eq!(response_status, Status::Ok);
        Ok(())
    }

    #[test]
    fn put_creates_new_product_category() -> Result<(), diesel_migrations::RunMigrationsError> {
        use crate::schema::product_category::dsl;

        let docker_cli = Cli::default();
        let database_metadata = crate::test_utils::start_database(&docker_cli);
        let (rocket, connection) = crate::test_utils::start_rocket_with_db(&database_metadata)?;
        let client = Client::new(rocket).expect("Expected valid rocket instance");

        let put_category = ProductCategoryRequestBody {
            name: "putcategory".to_string()
        };
        let mut put_request = client.put("/productcategory/5");
        put_request.add_header(ContentType::JSON);
        put_request.set_body(serde_json::to_string(&put_category).unwrap());

        let mut put_response = put_request.dispatch();
        let put_status = put_response.status();
        let response_body = put_response.body_string().unwrap();

        let expected_product_category = ProductCategory {
            id: 5,
            name: put_category.name
        };
        let response_product_category: ProductCategory = serde_json::from_str(&response_body).unwrap();

        let categories_in_db: Vec<ProductCategory> = product_category.load(&connection)?;
        assert_eq!(categories_in_db.len(), 1);
        assert_eq!(response_product_category, expected_product_category);
        assert_eq!(categories_in_db.get(0).unwrap(), &expected_product_category);
        Ok(())
    }
}
