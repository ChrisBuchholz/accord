#![feature(plugin)]
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

use rocket::{Request, Data, Outcome, Error};
use rocket::http::Status;
use rocket::data::{self, FromData};
use rocket_contrib::JSON;
use error::ApiError;
use response::{ApiResult, ApiResponse};
use account::Account;
use accord::{Accord, Error as AccordError};

impl FromData for Account
    where Account: Accord
{
    type Error = AccordError;
    fn from_data(r: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let json = JSON::<Account>::from_data(&r, data).unwrap();
        let account = json.unwrap();
        if let Err(err) = account.validate() {
            let error = err[0].clone();
            return Outcome::Failure((Status::from_code(422).unwrap(), error));
        }
        Outcome::Success(account)
    }
}

#[post("/", data = "<account>")]
fn json(account: Account) -> ApiResult<JSON<Account>> {
    Ok(ApiResponse(Status::Ok, JSON(account)))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![json])
        .launch();
}
