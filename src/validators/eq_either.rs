use std::fmt::Display;


/// Enforce that `T` must equal `value`.
pub fn eq<T: 'static>(value: T) -> Box<Fn(&T) -> ::ValidatorResult>
    where T: PartialEq + Display
{
    Box::new(move |s: &T| {
        if *s == value {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Does not equal %1.".to_string(),
                args: vec![value.to_string()],
				human_readable: format!("Does not equal '{}'", value)
            })
        }
    })
}


/// Enforce that `T` equals any of the values in `values`.
pub fn either<T: 'static>(values: Vec<T>) -> Box<Fn(&T) -> ::ValidatorResult>
    where T: PartialEq + Display + Clone
{
    Box::new(move |s: &T| {
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
                human_readable: format!("Must be one of {}", list.to_string())
            })
        }
    })
}