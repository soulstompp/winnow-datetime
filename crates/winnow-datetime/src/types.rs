use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct DateTime {
    /// The date part
    pub date: Date,
    /// The time part
    pub time: Time,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct PartialDateTime {
    // optional date part
    pub date: Option<PartialDate>,
    // optional time part
    pub time: Option<PartialTime>,
}

pub trait OffsetFormat: Sized {
    type Err;

    // Format the date
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    // Parse a date from a string
    fn parse(s: &str) -> Result<Self, Self::Err>;
}

/// A date object.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Date {
    /// consists of year, month and day of month
    YMD { year: i32, month: u32, day: u32 },
    /// consists of year, week and day of week
    Week { year: i32, week: u32, day: u32 },
    /// consists of year and day of year
    Ordinal { year: i32, day: u32 },
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum PartialDate {
    /// a standalone year
    Year { year: Option<i32> },
    /// consists of year, month and day of month
    YMD {
        year: Option<i32>,
        month: Option<u32>,
        day: Option<u32>,
    },
    /// consists of year, week and day of week
    YWD {
        year: Option<i32>,
        week: Option<u32>,
        day: Option<u32>,
    },
    /// consists of year and day of year
    YDDD { year: Option<i32>, day: Option<u32> },
}

/// A time object.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct Time {
    /// a 24th of a day
    pub hour: u32,
    /// 60 discrete parts of an hour
    pub minute: u32,
    /// a minute are 60 of these
    pub second: u32,
    /// everything after a `.`
    pub millisecond: u32,
    /// Note, offset can't be partial, so a regular Offset is used
    pub offset: Option<Offset>,
    /// time zone, which is more reliable than offset
    pub time_zone: Option<TimeZone>,
    /// calendar that the date should be calulcated off of
    pub calendar: Option<Calendar>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct PartialTime {
    pub hour: Option<u32>,
    pub minute: Option<u32>,
    pub second: Option<u32>,
    pub millisecond: Option<u32>,
    pub offset: Option<Offset>,
}

/// struct holding an optional number of repetitions and an `IntervalRange`
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Interval {
    pub repetitions: Option<Option<u32>>,
    pub range: IntervalRange,
}

///     Closed - Start date and end date, such as "2007-03-01T13:00:00Z/2008-05-11T15:30:00Z"
///     Closed Start - Start and time period, such as "2007-03-01T13:00:00Z/P1Y2M10DT2H30M"
///     Closed End - and end, such as "P1Y2M10DT2H30M/2008-05-11T15:30:00Z"
///     Open -  such as "P1Y2M10DT2H30M", with additional context information
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum IntervalRange {
    Closed {
        start: PartialDateTime,
        end: PartialDateTime,
    },
    ClosedStart {
        start: PartialDateTime,
        duration: Duration,
    },
    ClosedEnd {
        duration: Duration,
        end: PartialDateTime,
    },
    Open {
        duration: Duration,
    },
}

/// Struct holding offset offsets
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Offset {
    LocalUnknown {
        critical: bool,
    },
    Fixed {
        /// hour offset offset
        hours: i32,
        /// minute offset offset
        minutes: i32,
        critical: bool,
    },
}

/// A time duration.
///
/// ## Duration Grammar
///
/// | Duration     | ABNF Description                                     |
/// | ------------ | ---------------------------------------------------- |
/// | `dur-second` | 1*DIGIT "S"                                          |
/// | `dur-minute` | 1*DIGIT "M" [`dur-second`]                           |
/// | `dur-hour`   | 1*DIGIT "H" [`dur-minute`]                           |
/// | `dur-time`   | "T" (`dur-hour` / `dur-minute` / `dur-second`)       |
/// | `dur-day`    | 1*DIGIT "D"                                          |
/// | `dur-week`   | 1*DIGIT "W"                                          |
/// | `dur-month`  | 1*DIGIT "M" [`dur-day`]                              |
/// | `dur-year`   | 1*DIGIT "Y" [`dur-month`]                            |
/// | `dur-date`   | (`dur-day` / `dur-month` / `dur-year`) [`dur-time`]  |
/// | `duration`   | "P" (`dur-date` / `dur-time` / `dur-week`)           |
///
/// ## Examples
/// ```
/// // 1 year, 11 months, 16 days, 23 hours, 26 minutes, 59 seconds, 123 milliseconds
/// use winnow_datetime::types::Duration;
///
/// let d = Duration {
///      years: 1,
///      months: 11,
///      weeks: 0,
///      days: 16,
///      hours: 23,
///      minutes: 26,
///      seconds: 59,
///      milliseconds: Some(0.123)
/// };
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(default))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Duration {
    /// Number of calendar years
    pub years: u32,
    /// Number of months
    pub months: u32,
    /// Number of weeks
    pub weeks: u32,
    /// Number of days
    pub days: u32,
    /// Number of hours
    pub hours: u32,
    /// Number of minutes
    pub minutes: u32,
    /// Number of seconds
    pub seconds: u32,
    /// Number of milliseconds
    pub milliseconds: Option<f32>,
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.years == other.years
            && self.months == other.months
            && self.weeks == other.weeks
            && self.days == other.days
            && self.hours == other.hours
            && self.minutes == other.minutes
            && self.seconds == other.seconds
            && self.milliseconds.map(|v| v.to_bits()) == other.milliseconds.map(|v| v.to_bits())
    }
}

impl Eq for Duration {}

impl Duration {
    /// Whether this duration represents a zero duration.
    pub fn is_zero(&self) -> bool {
        let Duration {
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
        } = self;

        [*years, *months, *weeks, *days, *hours, *minutes, *seconds]
            .iter()
            .all(|&x| x == 0)
            && milliseconds.unwrap_or(0.0) == 0.0
    }

    /// Whether this duration has a time component.
    pub fn has_time(&self) -> bool {
        let Duration {
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            ..
        } = self;

        [*days, *hours, *minutes, *seconds].iter().all(|&x| x > 0)
            || milliseconds.unwrap_or(0.0) > 0.0
    }
}

impl From<Duration> for ::core::time::Duration {
    fn from(duration: Duration) -> Self {
        let Duration {
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
        } = duration;

        let secs = years * 365 * 86_400
            + months * 30 * 86_400
            + weeks * 7 * 86_400
            + days * 86_400
            + hours * 3600
            + minutes * 60
            + seconds;
        let nanos = (milliseconds.unwrap_or(0.0) * 1_000_000_000.0).floor();
        Self::new(secs as u64, nanos as u32)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Copy, Clone)]
pub struct DurationPart {
    pub whole: u32,
    pub frac: Option<f32>,
}

impl PartialEq for DurationPart {
    fn eq(&self, other: &Self) -> bool {
        self.whole == other.whole
            && self.frac.map(|f| f.to_bits()) == other.frac.map(|f| f.to_bits())
    }
}

impl Eq for DurationPart {}

/// A time duration with fractional precision.
///
/// ## FractionalDuration Grammar
///
/// | FractionalDuration     | ABNF Description                                     |
/// | ------------ | ---------------------------------------------------- |
/// | `dur-second` | 1*DIGIT "S"                                          |
/// | `dur-minute` | 1*DIGIT "M" [`dur-second`]                           |
/// | `dur-hour`   | 1*DIGIT "H" [`dur-minute`]                           |
/// | `dur-time`   | "T" (`dur-hour` / `dur-minute` / `dur-second`)       |
/// | `dur-day`    | 1*DIGIT "D"                                          |
/// | `dur-week`   | 1*DIGIT "W"                                          |
/// | `dur-month`  | 1*DIGIT "M" [`dur-day`]                              |
/// | `dur-year`   | 1*DIGIT "Y" [`dur-month`]                            |
/// | `dur-date`   | (`dur-day` / `dur-month` / `dur-year`) [`dur-time`]  |
/// | `duration`   | "P" (`dur-date` / `dur-time` / `dur-week`)           |
///
/// ## Examples
/// ```
/// // 1 year, 11 months, 16 days, 23 hours, 26 minutes, 59 seconds, 123 milliseconds
/// use winnow_datetime::types::FractionalDuration;
///
/// let d = FractionalDuration {
///      years: (1, None),
///      months: (11, None),
///      weeks: (0, None),
///      days: (16, None),
///      hours: (23, None),
///      minutes: (26, None),
///      seconds: (59, Some(0.123)),
/// };
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(default))]
#[derive(Debug, Default, Copy, Clone)]
pub struct FractionalDuration {
    /// Number of calendar years
    pub years: (u32, Option<f32>),
    /// Number of months
    pub months: (u32, Option<f32>),
    /// Number of weeks
    pub weeks: (u32, Option<f32>),
    /// Number of days
    pub days: (u32, Option<f32>),
    /// Number of hours
    pub hours: (u32, Option<f32>),
    /// Number of minutes
    pub minutes: (u32, Option<f32>),
    /// Number of seconds
    pub seconds: (u32, Option<f32>),
}

impl PartialEq for FractionalDuration {
    fn eq(&self, other: &Self) -> bool {
        [
            self.years,
            self.months,
            self.weeks,
            self.days,
            self.hours,
            self.minutes,
            self.seconds,
        ]
        .iter()
        .zip([
            other.years,
            other.months,
            other.weeks,
            other.days,
            other.hours,
            other.minutes,
            other.seconds,
        ])
        .all(|(part, other_part)| {
            part.0 == other_part.0
                && part.1.map(|v| v.to_bits()) == other_part.1.map(|v| v.to_bits())
        })
    }
}

impl Eq for FractionalDuration {}

impl FractionalDuration {
    /// Whether this duration represents a zero duration.
    pub fn is_zero(&self) -> bool {
        let FractionalDuration {
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
        } = self;
        [*years, *months, *weeks, *days, *hours, *minutes, *seconds]
            .iter()
            .all(|&x| x.0 == 0 && x.1.unwrap_or(0.0) == 0.0)
    }

    /// Whether this duration has a time component.
    pub fn has_time(&self) -> bool {
        let FractionalDuration {
            days,
            hours,
            minutes,
            seconds,
            ..
        } = self;
        [*days, *hours, *minutes, *seconds]
            .iter()
            .all(|&x| x.0 > 0 || x.1.unwrap_or(0.0) > 0.0)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TimeZone {
    Named{ zone: NamedTimeZone },
    Fixed{ offset: Offset },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct NamedTimeZone {
    /// Time zone name
    pub identifier: String,
    pub critical: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct Calendar {
    pub identifier: String,
    pub critical: bool,
}
