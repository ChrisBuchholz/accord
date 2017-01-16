use rocket::http::Status;
use rocket::response::{Response, Responder, status};
use rocket_contrib::JSON;
use accord::MultipleError;

#[derive(Debug)]
pub enum ApiError {
    MultipleError(MultipleError),
}

impl<'r> Responder<'r> for ApiError {
    fn respond(self) -> Result<Response<'r>, Status> {
        match self {
            ApiError::MultipleError(e) => {
                status::Custom(Status::from_code(422).unwrap(), JSON(e)).respond()
            }
        }
    }
}

impl From<MultipleError> for ApiError {
    fn from(e: MultipleError) -> ApiError {
        ApiError::MultipleError(e)
    }
}
