//! ISO8601 is a parser library for the
//! [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format
//! and partially RFC3339.
//!
//! Validity of a given date is not guaranteed, this parser will happily parse
//! `"2015-02-29"` as a valid date,
//! even though 2015 was no leap year.
//!
//! # Example
//!
//! ```rust
//! let datetime = winnow_iso8601::parse_datetime("2015-06-26T16:43:23+0200").unwrap();
//! ```

#![warn(clippy::doc_markdown)]
#![no_std]

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

/// duration mod
pub mod duration;
pub use duration::parse_duration;

/// offset mod
pub mod offset;

/// interval mod
pub mod interval;

/// fractional_duration mod
pub mod fractional_duration;
pub mod partial_date;
pub mod partial_datetime;
pub mod partial_time;
pub use fractional_duration::parse_fractional_duration;

pub use offset::parse_offset;
