use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::clone::Clone;
use std::ops::Range;

/// rules!("some string".to_string(), [min(5)])
pub fn min(min: usize) -> impl Fn(&String) -> ::ValidatorResult {
    move |s: &String| {
        if s.len() >= min {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must not contain less characters than %1.".to_string(),
                args: vec![min.to_string()],
            })
        }
    }
}

/// rules!("some string".to_string(), [max(64)])
pub fn max(max: usize) -> impl Fn(&String) -> ::ValidatorResult {
    move |s: &String| {
        if s.len() <= max {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must not contain more characters than %1.".to_string(),
                args: vec![max.to_string()],
            })
        }
    }
}

/// rules!("some string".to_string(), [length(5..64)])
pub fn length(range: Range<usize>) -> impl Fn(&String) -> ::ValidatorResult {
    move |s: &String| {
        match (min(range.start)(s), max(range.end)(s)) {
            (Err(_), Err(_)) => {
                Err(::Invalid {
                    msg: "Must not be less characters than %1 and not more than %2.".to_string(),
                    args: vec![range.start.to_string(), range.end.to_string()],
                })
            }
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            (_, _) => Ok(()),
        }
    }
}

/// rules!(25, [range(12..127)])
pub fn range<T: PartialOrd + Display + Clone>(range: Range<T>) -> impl Fn(&T) -> ::ValidatorResult {
    move |s: &T| {
        if *s >= range.start && *s <= range.end {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must be in the range %1..%2.".to_string(),
                args: vec![range.start.to_string(), range.end.to_string()],
            })
        }
    }
}

/// rules!("test@test.test".to_string(), [contains("@"), contains(".")])
pub fn contains<'a>(needle: &'a str) -> impl Fn(&String) -> ::ValidatorResult {
    move |s: &String| {
        if s.contains(needle) {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must contain %1.".to_string(),
                args: vec![needle.to_string()],
            })
        }
    }
}

/// rules!(25, [eq(25)])
pub fn eq<T>(value: T) -> impl Fn(&T) -> ::ValidatorResult
    where T: PartialEq + Display
{
    move |s: &T| {
        if *s == value {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Does not equal %1.".to_string(),
                args: vec![value.to_string()],
            })
        }
    }
}

/// rules!(25, [either(vec![15, 25, 35])])
pub fn either<T>(values: Vec<T>) -> impl Fn(&T) -> ::ValidatorResult
    where T: PartialEq + Display + Clone
{
    move |s: &T| {
        let x = values.iter().find(|x| *x == s);
        let r = x.is_some();
        if r == true {
            Ok(())
        } else {
            let list = values.iter()
                .cloned()
                .fold(String::new(), |acc, v| format!("{}, {}", acc, v));
            Err(::Invalid {
                msg: "Must be one of %1.".to_string(),
                args: vec![list.to_string()],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use validators::{length, range};

    #[test]
    fn test_length() {
        assert!(length(3..5)(&"12".to_string()).is_err());
        assert!(length(3..5)(&"123".to_string()).is_ok());
        assert!(length(3..5)(&"1234".to_string()).is_ok());
        assert!(length(3..5)(&"12345".to_string()).is_ok());
        assert!(length(3..5)(&"123456".to_string()).is_err());
    }

    #[test]
    fn test_range() {
        assert!(range(12..127)(&11).is_err());
        assert!(range(12..127)(&12).is_ok());
        assert!(range(12..127)(&50).is_ok());
        assert!(range(12..127)(&127).is_ok());
        assert!(range(12..127)(&128).is_err());
    }
}
