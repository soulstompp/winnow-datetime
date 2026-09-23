//! Parsers for [RFC 9557](https://www.rfc-editor.org/rfc/rfc9557) timestamps, which are RFC 3339
//! timestamps followed by optional suffixes such as `[Europe/Paris][u-ca=gregory]`, producing the
//! types from [winnow-datetime](https://crates.io/crates/winnow-datetime).
//!
//! Validity of a given date is not guaranteed, this parser will happily parse
//! `"2015-02-29"` as a valid date,
//! even though 2015 was no leap year.
//!
//! # Example
//!
//! ```rust
//! let datetime = winnow_rfc9557::parse_datetime("2015-06-26T16:43:23+02:00[Europe/Paris]").unwrap();
//! ```

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

mod clippy;

/// date mod
pub mod date;
pub use date::parse_date;

/// time mod
pub mod time;
pub use time::parse_time;

/// datetime mod
pub mod datetime;
pub use datetime::parse_datetime;

/// timezone mod
pub mod time_zone;
pub use time_zone::parse_time_zone;

pub mod calendar;
pub mod offset;
pub mod suffix;

pub use offset::parse_offset;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
