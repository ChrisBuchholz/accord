use rocket::http::Status;
use rocket::response::status;
use rocket::response::{Response, Responder};
use error::ApiError;

pub struct ApiResponse<T>(pub Status, pub T);

impl<'r, T: Responder<'r>> Responder<'r> for ApiResponse<T> {
    fn respond(self) -> Result<Response<'r>, Status> {
        status::Custom(self.0, self.1).respond()
    }
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
