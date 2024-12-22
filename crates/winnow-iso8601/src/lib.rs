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
//! let datetime = winnow_iso8601::datetime("2015-06-26T16:43:23+0200").unwrap();
//! let time = "16:43:23+0200".parse::<winnow_iso8601::Iso8601Time>().unwrap();
//! let date = "2015-02-29".parse::<winnow_iso8601::Iso8601Date>().unwrap();
//! let datetime = "2015-06-26T16:43:23+0200".parse::<winnow_iso8601::Iso8601DateTime>().unwrap();
//! let duration = "P2021Y11M16DT23H26M59.123S".parse::<winnow_iso8601::Duration>().unwrap();
//! ```

#![allow(clippy::uninlined_format_args)]
#![deny(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]
#![warn(clippy::doc_markdown)]
#![no_std]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[macro_use]
extern crate alloc;

mod display;
pub mod parsers;

mod date;
pub use date::{date, Iso8601Date};

pub use winnow_datetime::{Date, DateTime, Time, Timezone};

mod time;
pub use time::{time, Iso8601Time};

mod datetime;
pub use datetime::{datetime, Iso8601DateTime};

mod duration;
pub use duration::{duration, Duration};

mod timezone;
pub use timezone::{timezone, Iso8601Timezone};

#[cfg(feature = "serde")]
mod serde;
