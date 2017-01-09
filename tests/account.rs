#[macro_use]
extern crate accord;

use accord::{Accord, validate, Result as AccordResult};
use accord::validators::{length, contains, range};

struct Account {
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

#[test]
fn main() {
    let a = Account {
        name: "Test Test".to_string(),
        email: "test@test.test".to_string(),
        age: 25,
    };

    let b = Account {
        name: "Test".to_string(),
        email: "testtest.test".to_string(),
        age: 11,
    };

    assert!(a.validate().is_ok());
    assert!(b.validate().is_err());
}
