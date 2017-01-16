#[macro_use]
extern crate accord;

use accord::{Accord, Result as AccordResult, MultipleError, MultipleInvalid};
use accord::validators::{length, contains, range};

struct Account {
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

#[test]
fn main() {
    let okay = Account {
        name: "Test Test".to_string(),
        email: "test@test.test".to_string(),
        age: 25,
    };

    let error = Account {
        name: "Test".to_string(),
        email: "testtest.test".to_string(),
        age: 11,
    };

    assert!(okay.validate().is_ok());
    assert!(error.validate().is_err());
}
