/// Enforce that a string must contain `needle`.
pub fn contains(needle: &'static str) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        if s.contains(needle) {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must contain %1.".to_string(),
                args: vec![needle.to_string()],
				human_readable: format!("Must contain '{}'", needle)
            })
        }
    })
}

/// Enforce that a string contains only characters in `accepted`
pub fn contain_only(accepted: &'static [char]) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        for c in s.chars() {
			if !accepted.contains(&c) {
				return Err(::Invalid {
					msg: "Must not contain %1.".to_string(),
					args: vec![c.to_string()],
					human_readable: format!("Must not contain '{}'", c)
				});
			}
		}
		Ok(())
    })
}

/// Enforce that a string must not contain `needle`.
pub fn not_contain(needle: &'static str) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        if !s.contains(needle) {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must not contain %1.".to_string(),
                args: vec![needle.to_string()],
                human_readable: format!("Must not contain '{}'", needle)
            })
        }
    })
}

/// Enforce that a string must not contain any of `needles`.
pub fn not_contain_any(needles: &'static [&'static str]) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        for needle in needles {
            if s.contains(needle) {
                return Err(::Invalid {
                    msg: "Must not contain %1.".to_string(),
                    args: vec![needle.to_string()],
                    human_readable: format!("Must not contain '{}'", needle)
                });
            }
        }
        Ok(())
    })
}

/// Convenience function; 0-9, A-z
pub fn alphanumeric() -> Box<Fn(&String) -> ::ValidatorResult> {
    contain_only(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'])
}

/// Convenience function; Alphanumeric & underscore.
pub fn alphanumeric_dashes() -> Box<Fn(&String) -> ::ValidatorResult> {
    contain_only(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_', '-'])
}

#[cfg(test)]
mod tests {
    use super::*;

    // contains
    #[test]
    pub fn contains_valid() {
        assert!(contains("bar")(&"foobar".to_owned()).is_ok());
        assert!(contains("bar")(&"barbaz".to_owned()).is_ok());
        assert!(contains("bar")(&"bar".to_owned()).is_ok());
    }

    #[test]
    pub fn contains_invalid() {
        assert!(contains("bar")(&"foo".to_owned()).is_err());
        assert!(contains("bar")(&"baz".to_owned()).is_err());
        assert!(contains("bar")(&"ba".to_owned()).is_err());
    }

    // contain_only
    #[test]
    pub fn contain_only_valid() {
        assert!(contain_only(&['a', 'b', 'c'])(&"aaaaaaaa".to_owned()).is_ok());
        assert!(contain_only(&['a', 'b', 'c'])(&"bbbbbbbb".to_owned()).is_ok());
        assert!(contain_only(&['a', 'b', 'c'])(&"cccccccc".to_owned()).is_ok());
        assert!(contain_only(&['a', 'b', 'c'])(&"abcabcab".to_owned()).is_ok());
    }

    #[test]
    pub fn contain_only_invalid() {
        assert!(contain_only(&['a', 'b', 'c'])(&"abcdefghijklmnopqrstuvwxyz".to_owned()).is_err());
        assert!(contain_only(&['a', 'b', 'c'])(&"abacus".to_owned()).is_err());
        assert!(contain_only(&['a', 'b', 'c'])(&"sphinx of black quartz, judge my vow".to_owned()).is_err());
    }

    // not_contain
    #[test]
    pub fn not_contain_valid() {
        assert!(not_contain("bar")(&"foobaz".to_owned()).is_ok());
        assert!(not_contain("bar")(&"foo".to_owned()).is_ok());
        assert!(not_contain("bar")(&"baz".to_owned()).is_ok());
    }

    #[test]
    pub fn not_contain_invalid() {
        assert!(not_contain("bar")(&"foobarbaz".to_owned()).is_err());
        assert!(not_contain("bar")(&"bar".to_owned()).is_err());
        assert!(not_contain("bar")(&"embark".to_owned()).is_err());
    }
    
    // not_contain_any
        #[test]
    pub fn not_contain_any_valid() {
        assert!(not_contain_any(&["foo", "bar", "baz"])(&"xyzzyx".to_owned()).is_ok());
        assert!(not_contain_any(&["foo", "bar", "baz"])(&"asdfghjkl".to_owned()).is_ok());
        assert!(not_contain_any(&["foo", "bar", "baz"])(&"takyon".to_owned()).is_ok());
    }

    #[test]
    pub fn not_contain_any_invalid() {
        assert!(not_contain_any(&["foo", "bar", "baz"])(&"foo".to_owned()).is_err());
        assert!(not_contain_any(&["foo", "bar", "baz"])(&"bar".to_owned()).is_err());
        assert!(not_contain_any(&["foo", "bar", "baz"])(&"baz".to_owned()).is_err());
        assert!(not_contain_any(&["foo", "bar", "baz"])(&"foobarbaz".to_owned()).is_err());
    }
    
}