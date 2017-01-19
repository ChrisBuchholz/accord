#![feature(plugin, inclusive_range_syntax)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate accord;

extern crate rocket;
extern crate serde;
extern crate serde_json;

mod account;
mod error;
mod response;

use response::{ApiResult, ApiResponse};
use account::Account;
use rocket_contrib::JSON;
use rocket::http::Status;
use accord::MultipleError;

#[post("/", data = "<account>")]
fn json(account: Result<Account, MultipleError>) -> ApiResult<JSON<Account>> {
    account.map(|a| ApiResponse(Status::Ok, JSON(a))).map_err(|e| e.into())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![json])
        .launch();
}
