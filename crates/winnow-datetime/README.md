# winnow-datetime, making building new datetime format parsing a breeze. 

[![crates.io](https://img.shields.io/crates/v/winnow-datetime?style=flat-square)](https://crates.io/crates/winnow-datetime)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/winnow-datetime)

[winnow]: https://github.com/winnow-rs/winnow

## About

This library contains parsers for various possible ways to parse the pieces that make up a datetime. This would behave
most of which are covered strictly from . If you are looking for a parser to parse a date that you have come accross in 
while parsing a log, well-known format see the Supported Formats section below. 

However, 

## Parsing Something Strange
Despite there being countless specifications some people will still come up with their own way to poetically express a
datetime. So if you are looking to parse those you can build the provided structs with any combination of the pieces
needed. 

## Supported Formats
### RFC3339
Two of the most common formats are currently included in the workspace:
- [RFC3339](https://en.wikipedia.org/wiki/RFC_3339) which is a straightforward format that was inteded for communicating
on the internet. These will often come up in logs and protocol headers.
### ISO8601
- [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) is a more ambitious format that can represent a wide range of dates
- time ranges and time periods. 

This library hopes to provide common parsers and an AST which can be used to convert between these formats, along with
others that are share a lot in common with RFC_3339, such as SQL dates/times. Each format crate should follow the
following format:

### Format Crates
In order for the format crates to run tests from `winnow-assert` they should allow follow the
following structure:
```
-- /src
  |-- lib.rs
  |-- date.rs
      |-- parse_date(&str) -> Result<winnow_datetime::Date, Error>
      |-- coverage() -> winnow_assert::DateCoverage
  |-- datetime.rs
        |-- parse_datetime() -> Result<winnow_datetime::DateTime, Error>
        |-- coverage() -> winnow_assert::DateTimeCoverage
  |-- [duration.rs
        | -- parse_duration() -> Result<winnow_datetime::Duration, Error>
        | -- coverage()] -> winnow_assert::DurationCoverage
  |-- offset.rs
        | -- parse_offset() -> Result<winnow_datetime::Offset, Error>
        | -- coverage() -> winnow_assert::OffsetCoverage
  |-- parsers.rs
        | - date() -> PResult<winnow_datetime::Date>
        | - datetime() -> PResult<winnow_datetime::DateTime>
        | - [duration()] -> PResult<winnow_datetime::Duration>
        | - offset() -> PResult<winnow_datetime::Offset>
        | - time() -> PResult<winnow_datetime::Time>
  |-- time.rs
        | -- parse_time() -> Result<winnow_datetime::Time, Error>
        | -- coverage() -> winnow_assert::TimeCoverage
```


