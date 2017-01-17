#[macro_use]
extern crate accord;
extern crate serde;
extern crate serde_json;

use accord::{Accord, Result as AccordResult, Error, MultipleError, MultipleInvalid};
use accord::validators::{length, contains, range};

struct Account {
    pub name: String,
    pub email: String,
    pub age: i8,
}

impl Accord for Account {
    fn validate(&self) -> AccordResult {
        rules!{
            "name" => self.name => [length(1..64)],
            "email" => self.email => [length(5..64), contains("@"), contains(".")],
            "age" => self.age => [range(12..127)]
        }
    }
}

fn main() {
    let account = Account {
        name: "".to_string(),
        email: "test".to_string(),
        age: 11,
    };

    // You can use the `rules!` macro on any value.
    // This way of using the the `rules!` macro returns a
    // `Result<(), Error>`.
    let _ = rules!(account.name, [length(1..64)]);
    let _ = rules!(account.email, [length(5..64), contains("@"), contains(".")]);
    let _ = rules!(account.age, [range(12..127)]);

    // You can also use the collection form of the `rules!` macro
    // again using any value you'd like.
    // This way of using the `rules!` macro returns a
    // `Result<(), MultipleError>`. Notice the string slices that has
    // been appended to the lines from last example. These string slices
    // are called tags and are used to distingues between the sets of errors
    // that are returned.
    let _ = rules!{
        "name" => account.name => [length(1..64)],
        "email" => account.email => [length(5..64), contains("@"), contains(".")],
        "age" => account.age => [range(12..127)]
    };

    // And finally, since our `Account` has implemented the
    // `Accord` trait, we can simply do the following, which once
    // again returns `Result<(), MultipleError>`, which we then
    // serialize to JSON using Serde and print:
    if let Err(multiple_error) = account.validate() {
        println!("Errors as json: {}",
                 serde_json::to_string(&multiple_error).unwrap());
    } else {
        println!("No errors occured");
    }
}
