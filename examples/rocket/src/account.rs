use accord::{Accord, Result as AccordResult, MultipleError, MultipleInvalid};
use accord::validators::{length, contains, range};
use rocket::{Request, Data, Outcome};
use rocket::http::Status;
use rocket::data::{self, FromData};
use rocket_contrib::JSON;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub email: String,
    pub age: i8,
}

impl Accord for Account {
    fn validate(&self) -> AccordResult {
        rules!{
            "name" => self.name => [length(1, 64)],
            "email" => self.email => [length(5, 64), contains("@"), contains(".")],
            "age" => self.age => [range(12, 127)]
        }
    }
}

impl FromData for Account
    where Account: Accord
{
    type Error = MultipleError;
    fn from_data(r: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let json = JSON::<Account>::from_data(&r, data).unwrap();
        let account = json.unwrap();
        if let Err(error) = account.validate() {
            Outcome::Failure((Status::from_code(422).unwrap(), error))
        } else {
            Outcome::Success(account)
        }
    }
}
