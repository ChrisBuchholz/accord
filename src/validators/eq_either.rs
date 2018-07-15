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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[derive(PartialEq, Clone)]
    enum TestEnum {
        Yes,No,Maybe,IDontKnow,RepeatTheQuestion
    }

    impl Display for TestEnum {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Doesn't Matter")
        }
    }

    // eq
    #[test]
    pub fn eq_valid() {
        assert!(eq(1)(&1).is_ok());
        assert!(eq("yes")(&"yes").is_ok());
        assert!(eq(TestEnum::Yes)(&TestEnum::Yes).is_ok());
        assert!(eq(TestEnum::No)(&TestEnum::No).is_ok());
    }

    #[test]
    pub fn eq_invalid() {
        assert!(eq(1)(&2).is_err());
        assert!(eq("yes")(&"yesn't").is_err());
        assert!(eq(TestEnum::Yes)(&TestEnum::No).is_err());
        assert!(eq(TestEnum::No)(&TestEnum::Yes).is_err());
    
    }

    // either
    #[test]
    pub fn either_valid() {
        assert!(either(vec![1, 2, 3])(&1).is_ok());
        assert!(either(vec![1, 2, 3])(&2).is_ok());
        assert!(either(vec![1, 2, 3])(&3).is_ok());

        assert!(either(vec!["yes", "no"])(&"yes").is_ok());
        assert!(either(vec!["yes", "no"])(&"no").is_ok());

        assert!(either(vec![TestEnum::Maybe, TestEnum::IDontKnow])(&TestEnum::Maybe).is_ok());
        assert!(either(vec![TestEnum::Maybe, TestEnum::IDontKnow])(&TestEnum::IDontKnow).is_ok());
    }

    #[test]
    pub fn either_invalid() {
        assert!(either(vec![1, 2, 3])(&4).is_err());
        assert!(either(vec![1, 2, 3])(&5).is_err());
        assert!(either(vec![1, 2, 3])(&6).is_err());

        assert!(either(vec!["yes", "no"])(&"maybe").is_err());
        assert!(either(vec!["yes", "no"])(&"i don't know").is_err());

        assert!(either(vec![TestEnum::Maybe, TestEnum::IDontKnow])(&TestEnum::Yes).is_err());
        assert!(either(vec![TestEnum::Maybe, TestEnum::IDontKnow])(&TestEnum::No).is_err());
        assert!(either(vec![TestEnum::Maybe, TestEnum::IDontKnow])(&TestEnum::RepeatTheQuestion).is_err());
    }
}