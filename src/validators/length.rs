use ValidatorResult;
#[cfg(feature = "inclusive_range")]
use std::ops::RangeInclusive;


/// Enforce that a `String` is maximum `max` characters long.
pub fn max(max: usize) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        if s.len() <= max {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must not contain more characters than %1.".to_string(),
                args: vec![max.to_string()],
                human_readable: format!("Must contain less than {} characters", max)
            })
        }
    })
}

// Enforce that a `String` is minimum `min` characters long.
pub fn min(min: usize) -> Box<Fn(&String) -> ValidatorResult> {
    Box::new(move |s: &String| {
        if s.len() >= min {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must contain more than %1 characters".to_string(),
                args: vec![min.to_string()],
                human_readable: format!("Must contain more than {} characters", min)
            })
        }
    })
}

#[cfg(not(feature = "inclusive_range"))]
/// Enforce that a string is minimum `mi` and maximum `ma` characters long.
pub fn length(mi: usize, ma: usize) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        match (min(mi)(s), max(ma)(s)) {
            (Err(_), Err(_)) => {
                Err(::Invalid {
                    msg: "Must not be less characters than %1 and not more than %2.".to_string(),
                    args: vec![mi.to_string(), ma.to_string()],
                    human_readable: format!("Must contain between {} and {} characters", mi, ma - 1)
                })
            }
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            (_, _) => Ok(()),
        }
    })
}

#[cfg(not(feature = "inclusive_range"))]
/// Enforce that a string is minimum `mi` and maximum `ma` characters long if it is present. Always ok if not present.
pub fn length_if_present(mi: usize, ma: usize) -> Box<Fn(&Option<String>) -> ::ValidatorResult> {
    Box::new(move |s: &Option<String>| {
        if s.is_none() {
            return Ok(());
        }
        let s = s.as_ref().unwrap();
        match (min(mi)(s), max(ma)(s)) {
            (Err(_), Err(_)) => {
                Err(::Invalid {
                    msg: "Must not be less characters than %1 and not more than %2.".to_string(),
                    args: vec![mi.to_string(), ma.to_string()],
                    human_readable: format!("Must contain between {} and {} characters", mi, ma - 1)
                })
            }
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            (_, _) => Ok(()),
        }
    })
}

#[cfg(feature = "inclusive_range")]
/// Enforce that a string is minimum `mi` and maximum `ma` characters long.
pub fn length(range: RangeInclusive<usize>) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        match range {
            RangeInclusive::NonEmpty { ref start, ref end } => {
                match (min(*start)(s), max(*end)(s)) {
                    (Err(_), Err(_)) => {
                        Err(::Invalid {
                            msg: "Must not be less characters than %1 and not more than %2."
                                .to_string(),
                            args: vec![start.to_string(), end.to_string()],
                            human_readable: format!("Must contain between {} and {} characters", mi, ma)
                        })
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                    (_, _) => Ok(()),
                }
            }
            _ => panic!("range must be a RangeInclusive::NonEmpty"),
        }
    })
}

#[cfg(feature = "inclusive_range")]
/// Enforce that a string is minimum `mi` and maximum `ma` characters long if it is present. Always ok if not present.
pub fn length_if_present(range: RangeInclusive<usize>) -> Box<Fn(&Option<String>) -> ::ValidatorResult> {
    Box::new(move |s: &Option<String>| {
        if s.is_none() {
            return Ok(());
        }
        let s = s.as_ref().unwrap();
        match range {
            RangeInclusive::NonEmpty { ref start, ref end } => {
                match (min(*start)(s), max(*end)(s)) {
                    (Err(_), Err(_)) => {
                        Err(::Invalid {
                            msg: "Must not be less characters than %1 and not more than %2."
                                .to_string(),
                            args: vec![start.to_string(), end.to_string()],
                            human_readable: format!("Must contain between {} and {} characters", start, end)
                        })
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                    (_, _) => Ok(()),
                }
            }
            _ => panic!("range must be a RangeInclusive::NonEmpty"),
        }
    })
}
