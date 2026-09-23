//! Common types and parsers shared by the winnow-datetime family of crates.
//!
//! Most users want one of the format-specific crates, which parse into the types defined here:
//!
//! * [winnow-rfc3339](https://crates.io/crates/winnow-rfc3339) for RFC 3339 timestamps
//! * [winnow-rfc9557](https://crates.io/crates/winnow-rfc9557) for RFC 3339 with time zone and
//!   calendar suffixes
//! * [winnow-iso8601](https://crates.io/crates/winnow-iso8601) for ISO 8601 dates, times,
//!   durations and intervals
//!
//! This crate provides the AST ([`Date`], [`Time`], [`DateTime`], [`Duration`], [`Interval`],
//! ...), the component parsers in [`parser`] for building new formats, and, behind the
//! `chrono`, `jiff` and `time` features, `TryFrom` conversions into those libraries.
//!
//! # Example
//!
//! ```rust
//! use winnow_datetime::{Date, Time};
//!
//! let time = Time {
//!     hour: 16,
//!     minute: 43,
//!     second: 23,
//!     nanosecond: 870_479_000,
//!     ..Default::default()
//! };
//! let date = Date::YMD { year: 2015, month: 6, day: 26 };
//! # let _ = (time, date);
//! ```

extern crate core;

mod clippy;
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub mod convert;
mod macros;
pub mod parser;
pub mod types;
pub mod util;

pub use types::Calendar;
pub use types::Date;
pub use types::DateTime;
pub use types::Duration;
pub use types::FractionalDuration;
pub use types::Interval;
pub use types::NamedTimeZone;
pub use types::Offset;
pub use types::Time;
pub use types::TimeZone;

use winnow::Partial;

/// Type for holding partial data for parsers
pub type PartialInput<'i> = Partial<&'i [u8]>;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
