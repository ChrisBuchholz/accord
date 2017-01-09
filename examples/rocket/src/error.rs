use rocket::http::Status;
use rocket::response::{Response, Responder};
use rocket_contrib::JSON;
use std::error::Error;
use accord::Error as AccordError;

#[derive(Debug)]
pub enum ApiError {
    AccordError(AccordError),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

impl<'r> Responder<'r> for ApiError {
    fn respond(self) -> Result<Response<'r>, Status> {
        let message = match self {
            ApiError::AccordError(error) => format!("{:?}", error),
        };
        JSON(ErrorResponse {
                status: "error",
                message: message,
            })
            .respond()
    }
}

impl From<AccordError> for ApiError {
    fn from(error: AccordError) -> ApiError {
        ApiError::AccordError(error)
    }
}
