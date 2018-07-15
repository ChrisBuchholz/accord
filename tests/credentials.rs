#[macro_use]
extern crate accord;

use accord::{Accord, Result as AccordResult};
use accord::validators::{length, contains, not_contain_any};

struct Credentials {
    pub email: String,
    pub password: String,
}

impl Accord for Credentials {
    #[cfg(not(feature = "inclusive_range"))]
    fn validate(&self) -> AccordResult {
        rules!{
            "email" => self.email => [length(5, 64), contains("@"), contains(".")],
            "password" => self.password => [not_contain_any(&["1234", "admin", "password"])]
        }
    }

    #[cfg(feature = "inclusive_range")]
    fn validate(&self) -> AccordResult {
        rules!{
            "email" => self.email => [length(5..=64), contains("@"), contains(".")],
            "password" => self.password => [not_contain_any(&["1234", "admin", "password"])]
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
        password: "admin1234password".to_string(),
    };

    assert!(a.validate().is_ok());
    assert!(b.validate().is_err());
}
