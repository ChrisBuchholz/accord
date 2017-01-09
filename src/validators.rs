use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::clone::Clone;

pub fn min(min: usize) -> impl Fn(&String) -> ::Result {
    move |s: &String| {
        if s.len() >= min {
            Ok(())
        } else {
            // TODO: cant use format! because of linking failure
            // see issue https://github.com/rust-lang/rust/issues/39031
            // Err(vec![::Error::Invalid { msg: format!("{} must not be less than {}.", s, min) }])
            Err(vec![::Error::Invalid { msg: "".to_owned() }])
        }
    }
}

pub fn max(max: usize) -> impl Fn(&String) -> ::Result {
    move |s: &String| {
        if s.len() <= max {
            Ok(())
        } else {
            // TODO: cant use format! because of linking failure
            // see issue https://github.com/rust-lang/rust/issues/39031
            // Err(vec![::Error::Invalid { msg: format!("{} must not be larger than {}.", s, max) }])
            Err(vec![::Error::Invalid { msg: "".to_owned() }])
        }
    }
}

pub fn length(mi: usize, ma: usize) -> impl Fn(&String) -> ::Result {
    move |s: &String| {
        let errors = [min(mi)(s), max(ma)(s)]
            .iter()
            .cloned()
            .filter_map(|r| r.err())
            .flat_map(|r| r)
            .collect::<Vec<_>>();
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

pub fn range<T: PartialOrd + Display>(a: T, b: T) -> impl Fn(&T) -> ::Result {
    move |s: &T| {
        if *s >= a && *s <= b {
            Ok(())
        } else {
            Err(vec![::Error::Invalid { msg: format!("{} is not in the range {}...{}.", s, a, b) }])
        }
    }
}

pub fn contains<'a>(needle: &'a str) -> impl Fn(&String) -> ::Result {
    move |s: &String| {
        if s.contains(needle) {
            Ok(())
        } else {
            // TODO: cant use format! because of linking failure
            // see issue https://github.com/rust-lang/rust/issues/39031
            // Err(vec![::Error::Invalid { msg: format!("{} must contain a {}.", s, needle) }])
            Err(vec![::Error::Invalid { msg: "".to_owned() }])
        }
    }
}

pub fn eq<T>(value: T) -> impl Fn(&T) -> ::Result
    where T: PartialEq + Display
{
    move |s: &T| {
        if *s == value {
            Ok(())
        } else {
            Err(vec![::Error::Invalid { msg: format!("{} does not equal {}.", s, value) }])
        }
    }
}

pub fn either<T>(values: Vec<T>) -> impl Fn(&T) -> ::Result
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
            Err(vec![::Error::Invalid { msg: format!("{} is not one of {}.", s, list) }])
        }
    }
}
