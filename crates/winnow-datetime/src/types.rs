/// Compound struct, holds Date and Time.
/// ```
/// # use std::str::FromStr;
/// use winnow_datetime::DateTime;
/// /*
/// assert_eq!(
///     winnow_datetime::DateTime::from_str("2023-02-18T17:08:08.793Z"),
///     Ok(winnow_datetime::DateTime {
///         date: winnow_datetime::Date::YMD{ year: 2023, month: 2, day: 18},
///         time: winnow_datetime::Time{ hour: 17, minute: 8, second: 8, millisecond: 793, offset: Offset { offset_hours: 0, offset_minutes: 00 }}
///     })
/// )
/// */
/// ```
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct DateTime {
    /// The date part
    pub date: Date,
    /// The time part
    pub time: Time,
}

use core::fmt;

pub trait OffsetFormat: Sized {
    type Err;

    // Format the date
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    // Parse a date from a string
    fn parse(s: &str) -> Result<Self, Self::Err>;
}

#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Date {
    /// consists of year, month and day of month
    YMD { year: i32, month: u32, day: u32 },
    /// consists of year, week and day of week
    Week { year: i32, ww: u32, d: u32 },
    /// consists of year and day of year
    Ordinal { year: i32, ddd: u32 },
}

impl Default for Date {
    fn default() -> Date {
        Date::YMD {
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

/// A time object.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Time {
    /// a 24th of a day
    pub hour: u32,
    /// 60 discrete parts of an hour
    pub minute: u32,
    /// a minute are 60 of these
    pub second: u32,
    /// everything after a `.`
    pub millisecond: u32,
    /// the hour part of the offset offset from UTC
    pub offset: Option<Offset>,
}

impl Time {
    /// Change this time's offset offset.
    ///
    /// # Arguments
    ///
    /// * `tzo` - A tuple of `(hours, minutes)` specifying the offset offset from UTC.
    pub fn set_tz(&self, tzo: Option<(i32, i32)>) -> Time {
        let mut t = *self;

        if let Some(tzo) = tzo {
            t.offset = Some(Offset {
                offset_hours: tzo.0,
                offset_minutes: tzo.1,
            });
        } else {
            t.offset = None;
        }

        t
    }
}

/// Struct holding offset offsets
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Offset {
    /// hour offset offset
    pub offset_hours: i32,
    /// minute offset offset
    pub offset_minutes: i32,
}

#[macro_export]
macro_rules! year_ymd_parser {
    (
        $year:expr,        // Fully qualified year parser
        $month:expr,       // Fully qualified month parser
        $day:expr,         // Fully qualified day parser
        $separator:expr    // Separator parser (e.g., `literal("-")` or `opt(literal("-"))`)
    ) => {
        $crate::combinator::trace(stringify!($date), move |input: &mut _| {
            $crate::seq!($date {
                year: $year,
                _: $separator,
                month: $month,
                _: $separator,
                day: $day,
            })
            .parse_next(input)
        })
    };
}
