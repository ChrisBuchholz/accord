#![feature(conservative_impl_trait)]

pub mod validators;

#[derive(Debug, Clone)]
pub enum Error {
    Invalid { msg: String },
}

/// The return type used for all validators and validations
pub type Result = std::result::Result<(), Vec<Error>>;

/// Trait that can be used for accepting types that can be validated
pub trait Accord {
    fn validate(&self) -> Result;
}

/// Runs a list of validators on subject and returns a Result
///
/// # Examples
///
/// ```
/// use accord::validate;
/// use accord::validators::{min, either};
///
/// let _ = validate(&"a string".to_string(), &[&min(1)]);
/// let _ = validate(&25, &[&either(vec![10, 20])]);
/// ```
pub fn validate<T>(subject: &T, validators: &[&Fn(&T) -> Result]) -> Result {
    let errors = validators.iter()
        .filter_map(move |f| f(subject).err())
        .flat_map(|r| r)
        .collect::<Vec<_>>();
    // TODO: do this cleaner
    // possibly with .collect::<result<(), vec<_>>>()
    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(())
    }
}

/// Makes using the validate functions a tad easier
///
/// # Examples
///
/// Using the raw validate function:
///
/// ```
/// extern crate accord;
///
/// use accord::validate;
/// use accord::validators::{length, contains, range};
///
/// fn main() {
///     let email = "test@test.test".to_string();
///     let password = "kfjsdkfjsdkfjfksjdfkdsfjs".to_string();
///     let age = 25;
///
///     let email_errors = validate(&email, &[&length(5, 64), &contains("@"), &contains(".")]);
///     let password_errors = validate(&password, &[&length(8, 64)]);
///     let age_errors = validate(&age, &[&range(12, 127)]);
/// }
/// ```
///
/// The same can be expressed using the rules! macro like so:
///
/// ```
/// #[macro_use]
/// extern crate accord;
///
/// use accord::validate;
/// use accord::validators::{length, contains, range};
///
/// fn main() {
///     let email = "test@test.test".to_string();
///     let password = "kfjsdkfjsdkfjfksjdfkdsfjs".to_string();
///     let age = 25;
///
///     let errors = rules!{
///         email => [length(5, 64), contains("@"), contains(".")],
///         password => [length(8, 64)],
///         age => [range(12, 127)]
///     };
/// }
/// ```
#[macro_export]
macro_rules! rules {
    ( $( $a:expr => [ $( $b:expr ),* ] ),* ) => {{
        let errors = [$(validate(&$a, &[$(&$b),*])),*]
            .iter()
            .cloned()
            .filter_map(move |r| r.err())
            .flat_map(|r| r)
            .collect::<Vec<_>>();
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }}
}
