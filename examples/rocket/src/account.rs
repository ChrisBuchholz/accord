use error::ApiError;
use accord::{Accord, validate, Result as AccordResult, Error as AccordError};
use accord::validators::{length, contains, range};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub email: String,
    pub age: i8,
}

impl Accord for Account {
    fn validate(&self) -> AccordResult {
        rules!{
            self.name => [length(1, 64)],
            self.email => [length(5, 64), contains("@"), contains(".")],
            self.age => [range(12, 127)]
        }
    }
}
