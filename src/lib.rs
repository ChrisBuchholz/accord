#![feature(conservative_impl_trait)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod validators;

#[derive(Debug, Clone, Serialize)]
pub struct Invalid {
    pub msg: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MultipleInvalid {
    pub tag: String,
    pub invalids: Vec<Invalid>,
}

impl MultipleInvalid {
    pub fn invalid(tag: String, msg: String, args: Vec<String>) -> MultipleInvalid {
        MultipleInvalid {
            tag: tag,
            invalids: vec![Invalid {
                               msg: msg,
                               args: args,
                           }],
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Error(pub Vec<Invalid>);

#[derive(Debug, Clone, Serialize)]
pub struct MultipleError(pub Vec<MultipleInvalid>);

pub type ValidatorResult = std::result::Result<(), Invalid>;

pub type Result = std::result::Result<(), MultipleError>;

/// Trait that can be used for accepting types that can be validated
pub trait Accord {
    fn validate(&self) -> Result;
}

/// Runs a list of validators on data.
///
/// # Examples
///
/// The `rules!` macro has two forms, the first being `rules!(
/// You can run a list of validator on a single piece of data and get a `Result<(), Vec<Invalid>>`
/// back using `rules!` single form, which can be done like so:
///
/// ```
/// #[macro_use]
/// extern crate accord;
///
/// use accord::Error;
/// use accord::validators::{length, contains, range};
///
/// fn main() {
///     let email = "test@test.test".to_string();
///     let password = "kfjsdkfjsdkfjfksjdfkdsfjs".to_string();
///     let age = 25;
///
///     let _ = rules!(email, [length(5, 64), contains("@"), contains(".")]);
///     let _ = rules!(password, [length(8, 64)]);
///     let _ = rules!(age, [range(12, 127)]);
/// }
/// ```
///
/// If you have more than one piece of data to test, you can uses its collection form, which
/// returns a `Result<(), Vec<MultipleResult>>`.
///
/// Notice that in the collection form, you also provide a tag, like *email* or *password*.
/// This makes it easy to distingues between all the `MultipleInvalids` in the `Vector`.
///
/// ```
/// #[macro_use]
/// extern crate accord;
///
/// use accord::{MultipleError, MultipleInvalid};
/// use accord::validators::{length, contains, range};
///
/// fn main() {
///     let email = "test@test.test".to_string();
///     let password = "kfjsdkfjsdkfjfksjdfkdsfjs".to_string();
///     let age = 25;
///
///     let _ = rules!{
///         "email" => email => [length(5, 64), contains("@"), contains(".")],
///         "password" => password => [length(8, 64)],
///         "age" => age => [range(12, 127)]
///     };
/// }
/// ```
#[macro_export]
macro_rules! rules {
    ( $a:expr, [ $( $b:expr ),* ] ) => {{
        let invalids = [$($b(&$a)),*]
           .iter()
           .cloned()
           .filter_map(move |r| r.err())
           .collect::<Vec<_>>();
        if invalids.len() > 0 {
            Err(Error(invalids))
        } else {
            Ok(())
        }
    }};
    ( $( $a:expr => $b:expr => [ $( $c:expr ),* ] ),* ) => {{
        let multiple_invalids = vec![$(MultipleInvalid {
                tag: $a.to_string(),
                invalids: [$($c(&$b)),*]
                    .iter()
                    .cloned()
                    .filter_map(move |r| r.err())
                    .collect::<Vec<_>>()
            }),*]
            .iter()
            .cloned()
            .filter(|m| m.invalids.len() > 0)
            .collect::<Vec<_>>();
        if multiple_invalids.len() > 0 {
            Err(MultipleError(multiple_invalids))
        } else {
            Ok(())
        }
    }};
}