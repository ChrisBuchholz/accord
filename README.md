# Accord

[![Build Status](https://travis-ci.org/ChrisBuchholz/accord.svg?branch=master)](https://travis-ci.org/ChrisBuchholz/accord)
[![Current Crates.io Version](https://img.shields.io/crates/v/accord.svg)](https://crates.io/crates/accord)
[![API Documentation](https://docs.rs/accord/badge.svg)](https://docs.rs/accord)
[![codecov](https://codecov.io/gh/ChrisBuchholz/accord/branch/master/graph/badge.svg)](https://codecov.io/gh/ChrisBuchholz/accord)

⚠️ Accord uses the unstable feature
[conservative_impl_trait] and thus requires nightly until the feature lands in beta/stable.

[contribute]: #Contributing
[conservative_impl_trait]: https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md

Accord is a library for validating data according to rules like *length*, *contains*, *range* and *either*.

Accord is two fold, the first part being a set of validator-functions that
for example tests that a `String` has a minimum of 5 characters or that an `i32`
is either 10 or 20, and the second part being the `rules!` macro which allows you
to run a set of validators on a single piece of data, or a whole collection of data
and get back a set of errors which explains exactly what is wrong. The errors can
easily be serialized using [Serde] and then be used in for example a REST API to
report to the user which of the data the user posted contains illegal values.

See the [Rocket example] for how to use Accord with [Rocket] to validate `JSON` input
and return explanations for any occuring error as `JSON` which then can be
parsed by the requesting application and shown to the user to guide them in
how to fix their input values according to the applications rules.

Error messages uses numbered placeholders meaning that an error message could
be *"Must not be less than %1."* with an accompanien list `[5]`, which makes
it easy to translate *"Must not be less than %1."* without having to deal with the
variable value `5`.

[Serde]: https://serde.rs
[Rocket]: https://rocket.rs
[Rocket example]: https://github.com/ChrisBuchholz/accord/tree/master/examples/rocket

## Usage tl;dr:

```rust
#[macro_use]
extern crate accord;

use accord::{Error, MultipleError, MultipleInvalid};
use accord::validators::{length, contains, range};

fn main() {
    let email = "test@test.test".to_string();
    let password = "kfjsdkfjsdkfjfksjdfkdsfjs".to_string();
    let age = 25;

    // This way of using the the `rules!` macro returns a
    // `Result<(), Error>`
    let _ = rules!(email, [length(5, 64), contains("@"), contains(".")]);
    let _ = rules!(password, [length(8, 64)]);
    let _ = rules!(age, [range(12, 127)]);

    // This way of using the `rules!` macro returns a 
    // `Result<(), MultipleError>`. Notice the string slices that has
    // been appended to the lines from last example. These string slices
    // are called tags and are used to distingues between the sets of errors
    // that are returned. 
    let _ = rules!{
        "email" => email => [length(5, 64), contains("@"), contains(".")],
        "password" => password => [length(8, 64)],
        "age" => age => [range(12, 127)]
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

*Thanks to [Rocket][rocket-contrib] for showing how to form a great contributing-section.*

[rocket-contrib]: https://github.com/SergioBenitez/Rocket/blob/master/README.md#contributing

## License

Accord is Copyright (c) 2017 Christoffer Buchholz. It is free software, and
may be redistributed under the terms specified in the [LICENSE] file.

[LICENSE]: /LICENSE
