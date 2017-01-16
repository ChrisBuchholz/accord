use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::clone::Clone;

/// rules!("some string".to_string(), [min(5)])
pub fn min(min: usize) -> impl Fn(&String) -> ::ValidatorResult {
    move |s: &String| {
        if s.len() >= min {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must not be less than %1.".to_string(),
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
                msg: "Must not be larger than %1.".to_string(),
                args: vec![max.to_string()],
            })
        }
    }
}

/// rules!("some string".to_string(), [length(5, 64)])
pub fn length(mi: usize, ma: usize) -> impl Fn(&String) -> ::ValidatorResult {
    move |s: &String| {
        match (min(mi)(s), max(ma)(s)) {
            (Err(_), Err(_)) => {
                Err(::Invalid {
                    msg: "Must not be less than %1 and larger than %2.".to_string(),
                    args: vec![mi.to_string(), ma.to_string()],
                })
            }
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            (_, _) => Ok(()),
        }
    }
}

/// rules!(25, [range(12, 127)])
pub fn range<T: PartialOrd + Display + Clone>(a: T, b: T) -> impl Fn(&T) -> ::ValidatorResult {
    move |s: &T| {
        if *s >= a && *s <= b {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must be in the range %1..%2.".to_string(),
                args: vec![a.to_string(), b.to_string()],
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
