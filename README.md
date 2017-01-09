# Accord

[![Build Status](https://travis-ci.org/ChrisBuchholz/accord.svg?branch=master)](https://travis-ci.org/ChrisBuchholz/accord)
[![Current Crates.io Version](https://img.shields.io/crates/v/accord.svg)](https://crates.io/crates/accord)

⚠️ Accord uses the unstable feature
[conservative_impl_trait] and thus requires nightly until the feature lands in beta/stable.

[contribute]: #Contributing
[conservative_impl_trait]: https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md

Accord is a library for validating data according to rules like "contains", "length", "either".

Accord is two fold, the first part being a set of validator-functions that
for example tests that a `String` is a minimum 5 characters long or that an `i32`
is either 10 or 20, and the second part being a set of primitives that enables
you to pair these validators and run a "validation suite" on the contents of
a struct or any other data, to make sure that it contains what you expect it does
and if not, it returns useful error messages that can be shown to a user to help
the user fix the validation issue. 

## Usage tl;dr:

```rust
#[macro_use]
extern crate accord;

use accord::validate;
use accord::validators::{length, contains, range};

fn main() {
    let email = "test@test.test".to_string();
    let password = "kfjsdkfjsdkfjfksjdfkdsfjs".to_string();
    let age = 25;

    let errors = rules!{
        email => [length(5, 64), contains("@"), contains(".")],
        password => [length(8, 64)],
        age => [range(12, 127)]
    };
}
```

## Documentation

* [Examples]: Usage examples are available in the examples/ directory
* [API Documentation]: Documentation generated from the source code, comments and examples

[examples]: https://github.com/ChrisBuchholz/accord/tree/master/examples
[API Documentation]: https://docs.rs/accord

## Building locally

### Nightly

Accord requires a nightly version of Rust since it uses the feature
[conservative_impl_trait] which is not available in neither the
beta or stable version yet.

Building: `cargo build`

Testing: `cargo test`

## Contributing

Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

1. Submit a feature request or bug report as an [issue][issues].
2. Ask for improved documentation as an [issue][issues].
3. Contribute code via [pull requests][pulls].

[issues]: https://github.com/ChrisBuchholz/accord/issues
[pulls]: https://github.com/ChrisBuchholz/accord/pulls

To keep a high standard of quality, contributed code must be:

  * **Commented:** Public items _must_ be commented.
  * **Documented:** Exposed items _must_ have rustdoc comments with
    examples, if applicable.
  * **Styled:** Your code should be `rustfmt`'d when possible.
  * **Simple:** Your code should accomplish its task as simply and
     idiomatically as possible.
  * **Tested:** You must add (and pass) convincing tests for any functionality you add.
  * **Focused:** Your code should do what it's supposed to do and nothing more.

All pull requests are code reviewed and tested by the CI. Note that unless you
explicitly state otherwise, any contribution intentionally submitted for
inclusion in Accord by you shall be MIT License without any additional terms or conditions.

*Thanks to [Rocket] for showing how to form a great contributing-section.*

[rocket]: https://github.com/SergioBenitez/Rocket/blob/master/README.md#contributing

## License

Accord is Copyright (c) 2016 Christoffer Buchholz. It is free software, and
may be redistributed under the terms specified in the [LICENSE] file.

[LICENSE]: /LICENSE

