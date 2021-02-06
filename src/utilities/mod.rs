use rocket::{
    http::{ContentType, Status},
    response,
    response::Responder,
    Request,
};
use rocket_contrib::json::Json;
use serde::Serialize;

pub enum GetResponder<T> {
    Found(Json<T>),
    NotFound(()),
}

impl<'r, T: Serialize> Responder<'r> for GetResponder<T> {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        match self {
            Self::Found(json_body) => {
                respond_with_status_header(request, json_body, ContentType::JSON, Status::Ok)
            }
            Self::NotFound(empty) => {
                respond_with_status_header(request, empty, ContentType::JSON, Status::NotFound)
            }
        }
    }
}

pub enum PostResponder<T> {
    Created(Json<T>),
    Existed(Json<T>),
}

impl<'r, T: Serialize> Responder<'r> for PostResponder<T> {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        match self {
            Self::Created(json_body) => {
                respond_with_status_header(request, json_body, ContentType::JSON, Status::Created)
            }
            Self::Existed(json_body) => {
                respond_with_status_header(request, json_body, ContentType::JSON, Status::Ok)
            }
        }
    }
}

fn respond_with_status_header<'r, T: Responder<'r>>(
    request: &Request,
    responder: T,
    content_type: ContentType,
    status: Status,
) -> response::Result<'r> {
    let mut intermediate_response = responder.respond_to(request)?;
    intermediate_response.set_header(content_type);
    intermediate_response.set_status(status);
    Ok(intermediate_response)
}

#[cfg(test)]
mod tests {
    use rocket::{Rocket, Request};
    use rocket::local::{Client, LocalRequest};
    use crate::utilities::{GetResponder, PostResponder};
    use rocket_contrib::json::Json;
    use rocket::response::Responder;
    use rocket::http::{Status, ContentType};

    #[test]
    fn get_responder_returns_ok_if_variant_is_found() {
        let rocket = Rocket::ignite();
        let client = Client::new(rocket).unwrap();
        let local_request = client.get("/");
        let request = local_request.inner().clone();
        let get_responder_under_test = GetResponder::Found(Json(()));

        match get_responder_under_test.respond_to(&request) {
            Ok(response) => assert_eq!(Status::Ok, response.status()),
            Err(status) => panic!(format!("Failed with status: {}", status)),
        }
    }

    #[test]
    fn get_responder_returns_json_if_variant_is_found() {
        let rocket = Rocket::ignite();
        let client = Client::new(rocket).unwrap();
        let local_request = client.get("/");
        let request = local_request.inner();
        let get_responder_under_test = GetResponder::Found(Json(()));

        match get_responder_under_test.respond_to(request) {
            Ok(response) => assert_eq!(Some(ContentType::JSON), response.content_type()),
            Err(status) => panic!(format!("Failed because content type is not JSON with status {}", status)),
        }
    }

    #[test]
    fn get_responder_status_is_404_if_not_found() {
        let rocket = Rocket::ignite();
        let client = Client::new(rocket).unwrap();
        let local_request = client.get("/");
        let request = local_request.inner();
        let get_responder_under_test = GetResponder::<()>::NotFound(());

        match get_responder_under_test.respond_to(request) {
            Ok(response) => assert_eq!(Status::NotFound, response.status()),
            Err(status) => panic!(format!("Failed because status code of responder is not 404 with status : {}", status)),
        }
    }

    #[test]
    fn get_responder_returns_json_if_variant_is_not_found() {
        let rocket = Rocket::ignite();
        let client = Client::new(rocket).unwrap();
        let local_request = client.get("/");
        let request = local_request.inner();
        let get_responder_under_test = GetResponder::<()>::NotFound(());

        match get_responder_under_test.respond_to(request) {
            Ok(response) => assert_eq!(Some(ContentType::JSON), response.content_type()),
            Err(status) => panic!(format!("Failed with status {} because responder does not set content type to JSON", status)),
        }
    }


    #[test]
    fn post_responder_returns_ok_if_variant_is_existed() {
        let rocket = Rocket::ignite();
        let client = Client::new(rocket).unwrap();
        let local_request = client.get("/");
        let request = local_request.inner();
        let post_responder_under_test = PostResponder::Existed(Json(()));

        match post_responder_under_test.respond_to(request) {
            Ok(response) => assert_eq!(Status::Ok, response.status()),
            Err(status) => panic!(format!("Test failed because PostResponder::Created does not have Status::Ok")),
        }
    }

    #[test]
    fn post_responder_returns_json_if_variant_is_existed() {
        let rocket = Rocket::ignite();
        let client = Client::new(rocket).unwrap();
        let local_request = client.get("/");
        let request = local_request.inner();
        let post_responder_under_test = PostResponder::Existed(Json(()));

        match post_responder_under_test.respond_to(request) {
            Ok(response) => assert_eq!(Some(ContentType::JSON), response.content_type()),
            Err(status) => panic!(format!("Test failed because PostResponder::Created does not have ContentType::Json")),
        }
    }


    #[test]
    fn post_responder_returns_created_if_variant_is_created() {
        let rocket = Rocket::ignite();
        let client = Client::new(rocket).unwrap();
        let local_request = client.get("/");
        let request = local_request.inner();
        let post_responder_under_test = PostResponder::Created(Json(()));

        match post_responder_under_test.respond_to(request) {
            Ok(response) => assert_eq!(Status::Created, response.status()),
            Err(status) => panic!(format!("Test failed because PostResponder::Created does not have Status::Ok")),
        }
    }

    #[test]
    fn post_responder_returns_json_if_variant_is_created() {
        let rocket = Rocket::ignite();
        let client = Client::new(rocket).unwrap();
        let local_request = client.get("/");
        let request = local_request.inner();
        let post_responder_under_test = PostResponder::Created(Json(()));

        match post_responder_under_test.respond_to(request) {
            Ok(response) => assert_eq!(Some(ContentType::JSON), response.content_type()),
            Err(status) => panic!(format!("Test failed because PostResponder::Created does not have ContentType::Json")),
        }
    }

}
