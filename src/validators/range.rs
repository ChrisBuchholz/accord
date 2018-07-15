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
    Box::new(move |s: &T| {
        match range {
            RangeInclusive::NonEmpty { ref start, ref end } => {
                if *s >= *start && *s <= *end {
                    Ok(())
                } else {
                    Err(::Invalid {
                        msg: "Must be in the range %1..%2.".to_string(),
                        args: vec![start.to_string(), end.to_string()],
						human_readable: format!("Must be between {} and {}", start, end)
                    })
                }
            } 
            _ => panic!("range must be a RangeInclusive::NonEmpty"),
        }
    })
}