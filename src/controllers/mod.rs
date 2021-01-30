use rocket_contrib::json::Json;
use rocket::{Request, response, response::Responder, http::{ContentType, Status}};
use serde::Serialize;

pub mod product_category;
pub mod product;

pub enum GetResponder<T> {
    Found(Json<T>),
    NotFound(())
}

impl<'r, T : Serialize> Responder<'r> for GetResponder<T> {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        match self {
            Self::Found(json_body) => {
                respond_with_status_header(request, json_body, ContentType::JSON, Status::Ok)
            },
            Self::NotFound(empty) => {
                respond_with_status_header(request, empty, ContentType::JSON, Status::Ok)
            }
        }
    }
}

pub enum PostResponder<T> {
    Created(Json<T>),
    Existed(Json<T>)
}

impl<'r, T: Serialize> Responder<'r> for PostResponder<T> {
    fn respond_to(self, request: &Request) -> response::Result<'r>{
        match self {
            Self::Created(json_body) => {
                respond_with_status_header(request, json_body, ContentType::JSON, Status::Created)
            },
            Self::Existed(json_body) => {
                respond_with_status_header(request, json_body, ContentType::JSON, Status::Ok)
            }
        }
    }
}


fn respond_with_status_header<'r, T: Responder<'r>>(request: &Request, responder: T, content_type: ContentType, status: Status) -> response::Result<'r> {
    let mut intermediate_response = responder.respond_to(request)?;
    intermediate_response.set_header(content_type);
    intermediate_response.set_status(status);
    Ok(intermediate_response)
}
