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

/// Convenience function for validating email addresses
pub fn email() -> Box<Fn(&String) -> ::ValidatorResult> {
    regex(r"^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$", "i")
}

/// Convenience function for validating URLs.
pub fn url() -> Box<Fn(&String) -> ::ValidatorResult> {
    regex(r"^(https?://)?([A-Z]+\.)+[A-Z]{2,}(/[A-Z%0-9_]*)*?(/[A-Z%0-9_]*\.[A-Z%0-9_]*)?([#?][A-Z%0-9=&]*){0,2}$", "i")
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

    // email
    #[test]
    pub fn email_valid() {
        assert!(email()(&"asdf@asdf.com".to_owned()).is_ok());
        assert!(email()(&"amazing.email.address@super.amazing.site.tk".to_owned()).is_ok());
        assert!(email()(&"prick@legal.google".to_owned()).is_ok());
    }

    #[test]
    pub fn email_invalid() {
        assert!(email()(&"you tried.".to_owned()).is_err());
        assert!(email()(&"so close @super.amazing.site.tk".to_owned()).is_err());
        assert!(email()(&"prick@legal.g".to_owned()).is_err());
        assert!(email()(&"prick@important legal documents!.google".to_owned()).is_err());
    }

    // url
    #[test]
    pub fn url_valid() {
        assert!(url()(&"http://github.com".to_owned()).is_ok());
        assert!(url()(&"https://github.com".to_owned()).is_ok());
        assert!(url()(&"www.github.com".to_owned()).is_ok());
        assert!(url()(&"github.com".to_owned()).is_ok());
        assert!(url()(&"sub.domains.github.fr".to_owned()).is_ok());
        assert!(url()(&"neet.cool".to_owned()).is_ok());
        
        assert!(url()(&"github.com/org/repo".to_owned()).is_ok());
        assert!(url()(&"github.com/org/repo/logo.jpeg".to_owned()).is_ok());
        assert!(url()(&"github.com/org/repo/README#Usage".to_owned()).is_ok());
        assert!(url()(&"github.com/org/repo?folder=src&sort=desc".to_owned()).is_ok());
    }

    #[test]
    pub fn url_invalid() {
        assert!(url()(&"ftp://super.amazing.site".to_owned()).is_err());
        assert!(url()(&"http/super.amazing.site".to_owned()).is_err());
        assert!(url()(&"my awesome site!!.com".to_owned()).is_err());
        assert!(url()(&"nonon.o".to_owned()).is_err());
        assert!(url()(&"github.com/invalid file".to_owned()).is_err());
        assert!(url()(&"github.com/file? bad query params".to_owned()).is_err());
        assert!(url()(&"github.com/file# bad jump thing".to_owned()).is_err());
    }
}