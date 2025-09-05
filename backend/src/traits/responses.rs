use actix_web::{HttpResponse, Result as ActixResult};
use serde::Serialize;
use crate::models::Product;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String
}

pub trait ResponseHelper {
    fn to_response(self) -> ActixResult<HttpResponse>;
}

impl ResponseHelper for anyhow::Result<bool> {
    fn to_response(self) -> ActixResult<HttpResponse> {
        match self {
            Ok(true) => Ok(HttpResponse::NoContent().finish()),
            Ok(false) => Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "Not found".to_string()
            })),
            Err(err) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: err.to_string()
            }))
        }
    }
}

impl<T: Serialize> ResponseHelper for anyhow::Result<Option<T>> {
    fn to_response(self) -> ActixResult<HttpResponse> {
        match self {
            Ok(Some(data)) => Ok(HttpResponse::Ok().json(data)),
            Ok(None) => Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "Not found".to_string()
            })),
            Err(err) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: err.to_string()
            }))
        }
    }
}

impl<T: Serialize> ResponseHelper for anyhow::Result<Vec<T>> {
    fn to_response(self) -> ActixResult<HttpResponse> {
        match self {
            Ok(data) => Ok(HttpResponse::Ok().json(data)),
            Err(err) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: err.to_string()
            }))
        }
    }
}

impl ResponseHelper for anyhow::Result<Product> {
    fn to_response(self) -> ActixResult<HttpResponse> {
        match self {
            Ok(data) => Ok(HttpResponse::Ok().json(data)),
            Err(err) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: err.to_string()
            }))
        }
    }
}
