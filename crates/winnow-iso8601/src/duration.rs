use core::str::FromStr;

use crate::parsers;
use alloc::string::String;

/// A time duration.
///
/// ## Duration grammar[^rfc3339]
/// [^rfc3339]: Durations from RFC 3339 <https://www.rfc-editor.org/rfc/rfc3339#page-13>
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
/// assert_eq!(winnow_iso8601::parse_duration("P2021Y11M16DT23H26M59.123S"),
/// Ok(winnow_iso8601::Duration {
///      years: 2021,
///      months: 11,
///      weeks: 0,
///      days: 16,
///      hours: 23,
///      minutes: 26,
///      seconds: 59,
///      milliseconds: 123
/// }))
/// ```
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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
    pub milliseconds: u32,
}

impl Duration {
    /// Whether this duration represents a zero duration.
    pub fn is_zero(&self) -> bool {
        *self
            == Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            }
    }

    /// Whether this duration has a time component.
    pub fn has_time(&self) -> bool {
        [
            self.days,
            self.hours,
            self.minutes,
            self.seconds,
            self.milliseconds,
        ]
        .iter()
        .any(|&x| x > 0)
    }
}

impl Default for Duration {
    fn default() -> Duration {
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        }
    }
}

impl FromStr for Duration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_duration(s)
    }
}

impl From<Duration> for ::core::time::Duration {
    fn from(duration: Duration) -> Self {
        let secs = u64::from(duration.years) * 365 * 86_400
            + u64::from(duration.months) * 30 * 86_400
            + u64::from(duration.weeks) * 7 * 86_400
            + u64::from(duration.days) * 86_400
            + u64::from(duration.hours) * 3600
            + u64::from(duration.minutes) * 60
            + u64::from(duration.seconds);
        let nanos = duration.milliseconds * 1_000_000;
        Self::new(secs, nanos)
    }
}

/// Parses a duration string.
///
/// A string starts with `P` and can have one of the following formats:
///
/// * Fully-specified duration: `P1Y2M3DT4H5M6S`
/// * Duration in weekly intervals: `P1W`
/// * Fully-specified duration in [`DateTime`](`crate::DateTime`) format: `P<datetime>`
///
/// Both fully-specified formats get parsed into the YMDHMS Duration variant.
/// The weekly interval format gets parsed into the Weeks Duration variant.
///
/// The ranges for each of the individual units are not expected to exceed
/// the next largest unit.
///
/// These ranges (inclusive) are as follows:
///
/// * Year (any valid u32)
/// * Month 0 - 12
/// * Week 0 - 52
/// * Day 0 - 31
/// * Hour 0 - 24
/// * Minute 0 - 60
/// * Second 0 - 60
///
/// ## Examples
///
/// ```rust
/// let duration = winnow_iso8601::parse_duration("P1Y2M3DT4H5M6S").unwrap();
/// let duration = winnow_iso8601::parse_duration("P1W").unwrap();
/// ```
pub fn parse_duration(mut i: &str) -> Result<Duration, String> {
    match parsers::duration(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse duration {}: {}", i, e)),
    }
}
