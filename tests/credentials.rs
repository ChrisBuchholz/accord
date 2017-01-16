#[macro_use]
extern crate accord;

use accord::{Accord, Result as AccordResult, MultipleError, MultipleInvalid};
use accord::validators::{length, contains};

struct Credentials {
    pub email: String,
    pub password: String,
}

impl Accord for Credentials {
    fn validate(&self) -> AccordResult {
        rules!{
            "email" => self.email => [length(5, 64), contains("@"), contains(".")],
            "email" => self.password => [length(8, 64)]
        }
    }
}

#[test]
fn main() {
    let a = Credentials {
        email: "test@test.test".to_string(),
        password: "lfdsfsfsfghdgdljddsjfkdlsf".to_string(),
    };

    let b = Credentials {
        email: "t".to_string(),
        password: "l".to_string(),
    };

    assert!(a.validate().is_ok());
    assert!(b.validate().is_err());
}
