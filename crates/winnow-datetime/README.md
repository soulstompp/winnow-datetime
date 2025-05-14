# winnow-datetime, making building new datetime parsers a breeze.

[![crates.io](https://img.shields.io/crates/v/winnow-datetime?style=flat-square)](https://crates.io/crates/winnow-datetime)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/winnow-datetime)

[winnow]: https://github.com/winnow-rs/winnow
[rfc3339]: https://en.wikipedia.org/wiki/RFC_3339
[winnow-iso8601]: https://crates.io/crates/winnow-iso8601
[winnow-rfc3339]: https://crates.io/crates/winnow-rfc3339
[winnow-datetime-assert]: https://crates.io/crates/winnow-datetime-assert

## About

This library hopes to provide common parsers and an AST which can be used to convert between these formats, along with
others that are share a lot in common with RFC_3339, such as SQL dates/times. Each format crate should follow the
following format.

For most use cases using one of the format specific crates will be the best option. However, if you are looking to parse
something that doesn't fit into one of the provided formats you can use the parsers provided in this crate to build up
your own parser.

## Format-specific Crates
### RFC3339
[winnow-rfc3339] provides parsers for [RFC3339][rfc3339] dates and times. This
is the most common format used on the internet.

### ISO8601
[winnow-iso8601] provides parsers for [ISO8601](https://en.wikipedia.org/wiki/ISO_8601)
dates, times, durations, and intervals. [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) is a very ambitious format
that can represent a wide range of date and time concepts.

### Conversion
[winnow-datetime] provides a set of TryInto implementations to convert to common rust date/time libraries. Currently
chrono, jiff, and time are supported. Each have a feature flag of the same name as the lib to enable support for the
conversions. The TryInto implementations are available with the features and so try_into() could be called to convert to
any of the compatible types.

## Parsing Something Strange
Despite there being countless specifications some people will still come up with their own way to poetically express a
datetime. So if you are looking to parse those you can build the provided structs with any combination of the pieces
needed. It is probably best to start with the [winnow-rfc3339] crate and only replace the parsers where they differ.

Most parsers assume that date and time components are specified from the largest to smallest unit. However, even if the
format you need to parse isn't this library would still be a good option to build off, there will just be some additional
parts that will have be written by hand, particularly pieces currently handled by macros.

### Test and Benchmarking
Format-specific crates will use [winnow-datetime-assert] to build test and benchmark binaries. This crate provides a
large set of test cases which greatly help ensure reliability and performance. Any crates off of these parsers should
consider implementing these tests to avoid finding countless edge-cases by trial-and-error.
