use rocket::http::Status;
use rocket::response::{Response, Responder};
use error::ApiError;

pub struct ApiResponse<T>(pub Status, pub T);

impl<'r, T: Responder<'r>> Responder<'r> for ApiResponse<T> {
    fn respond(self) -> Result<Response<'r>, Status> {
        Response::build()
            .merge(self.1.respond()?)
            .status(self.0)
            .ok()
    }
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
