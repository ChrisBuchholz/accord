use std::fmt::Display;

#[cfg(feature = "inclusive_range")]
use std::ops::RangeInclusive;

#[cfg(not(feature = "inclusive_range"))]
pub fn range<T: 'static + PartialOrd + Display + Clone>(a: T,
                                                        b: T)
                                                        -> Box<Fn(&T) -> ::ValidatorResult> {
    Box::new(move |s: &T| {
        if *s >= a && *s <= b {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must be in the range %1..%2.".to_string(),
                args: vec![a.to_string(), b.to_string()],
				human_readable: format!("Must be between {} and {}", a, b)
            })
        }
    })
}

#[cfg(feature = "inclusive_range")]
pub fn range<T: 'static + PartialOrd + Display + Clone>(range: RangeInclusive<T>)
                                                        -> Box<Fn(&T) -> ::ValidatorResult> {
    // do bounds checking here so we can panic early if needed
    if range.end() <= range.start() {
        panic!("Invalid range!"); // TODO: Bad way to do this.
    }

    Box::new(move |s: &T| {
        let start = range.start();
        let end = range.end();

        if *s >= *start && *s <= *end {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must be in the range %1..%2.".to_string(),
                args: vec![start.to_string(), end.to_string()],
				human_readable: format!("Must be between {} and {}", start, end)
            })
        }
    })
}

#[cfg(test)]
#[cfg(not(feature = "inclusive_range"))]
mod tests {
    use super::*;

    // range

    #[test]
    pub fn range_valid() {
        assert!(range(1, 100)(&1).is_ok());
        assert!(range(1, 100)(&50).is_ok());
        assert!(range(1, 100)(&100).is_ok());
    }

    #[test]
    pub fn range_invalid() {
        assert!(range(1, 100)(&0).is_err());
        assert!(range(1, 100)(&101).is_err());
    }
}

#[cfg(test)]
#[cfg(feature = "inclusive_range")]
mod tests {
    use super::*;

    // range

    #[test]
    pub fn range_valid() {
        assert!(range(1..=100)(&1).is_ok());
        assert!(range(1..=100)(&50).is_ok());
        assert!(range(1..=100)(&100).is_ok());
    }

    #[test]
    pub fn range_invalid() {
        assert!(range(1..=100)(&0).is_err());
        assert!(range(1..=100)(&101).is_err());
    }
}