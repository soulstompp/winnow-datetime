# winnow-datetime, making datetime parsing a breeze.

[winnow]: https://github.com/winnow-rs/winnow
[winnow-datetime]: https://crates.io/crates/winnow-datetime
[winnow-iso8601]: https://crates.io/crates/winnow-iso8601
[winnow-rfc3339]: https://crates.io/crates/winnow-rfc3339
[winnow-datetime-assert]: https://crates.io/crates/winnow-datetime-assert

## About
`winnow-datetime` is a family of crates for parsing datetime formats with [winnow] with a consistent API 
and using a common AST that converts to common rust datetime libraries. These parsers will ensure that the datetime is
correctly formatted and does some validation on values as specified for each format. Date validity is not ensured until
conversion.

The core crate, [winnow-datetime], provides the AST and common parsers and parser macros to build out
the parsers needed to parse datetime components that behaves like any core winnow parser and can be called from any
winnow parser or combinator.

## Format-specific Crates
Most public parsers will return the one of the types defined in [winnow-datetime], most of which should convert directly
to an equivalent object from the datetime library. Some formats accept values that leave some ambiguity in the conversion
process and the consumer will need to decide how to handle these cases, however, these should be fairly rare. 

* [winnow-rfc3339] - parsers for RFC3339 dates and times, this is probably the one you are looking for.
* [winnow-iso8601] - parsers for ISO8601 dates, times, durations, and intervals.

## Testing
* [winnow-datetime-assert] - provides macros for building test and benchmark binaries for format-specific parser crates 
  built with [winnow-datetime].
