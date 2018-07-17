use regex::RegexBuilder;

/// Enforce that a string must match a given regex
/// Flags is a string containing all the flags that affect the regex.
/// i = ignore case
/// m = multiline
/// s = dot matches new line
/// U = swaps greedy modifiers (a* is lazy, a*? is greedy)
/// u = disable unicode support
/// x = ignore whitespace
/// Note that this doesn't test the length of the match; meaning
/// r"a..." will match "a12345678" as a match is present.
/// To force the value to match the regex exactly, use ^ and $
/// e.g: r"^a...$" will not match "a12345678"
pub fn regex(regex: &'static str, flags: &'static str) -> Box<Fn(&String) -> ::ValidatorResult> {
    let regex = RegexBuilder::new(regex)
        .case_insensitive(flags.contains("i"))
        .multi_line(flags.contains("m"))
        .dot_matches_new_line(flags.contains("s"))
        .swap_greed(flags.contains("U"))
        .unicode(!flags.contains("u"))
        .ignore_whitespace(flags.contains("x"))
        .build()
        .expect("Invalid regex in validator!");
    Box::new(move |s: &String| {
        if regex.is_match(s) {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must match regex '/%1/%2'.".to_string(),
                // TODO: sucks to clone this, it could probably be a &'static str
                args: vec![regex.as_str().to_owned(), flags.to_owned()],
				human_readable: format!("Must match regex '/{}/{}'", regex.as_str().clone(), flags)
            })
        }
    })
}

#[cfg(test)]
mod tests
{
    use super::*;
    // regex
    #[test]
    pub fn regex_valid() {
        assert!(regex(r"^a...$", "")(&"a123".to_owned()).is_ok());
        assert!(regex(r"^a...$", "")(&"abcd".to_owned()).is_ok());
        assert!(regex(r"^a...$", "")(&"a!?£".to_owned()).is_ok());
    }
    #[test]
    pub fn regex_invalid() {
        assert!(regex(r"^a...$", "")(&"a".to_owned()).is_err());
        assert!(regex(r"^a...$", "")(&"abc".to_owned()).is_err());
        assert!(regex(r"^a...$", "")(&"a123457".to_owned()).is_err());
        assert!(regex(r"^a...$", "")(&"b".to_owned()).is_err());
        assert!(regex(r"^a...$", "")(&"baaa".to_owned()).is_err());
    }
}