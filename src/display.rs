use core::fmt::{self, Display};

use super::{Date, DateTime, Duration, Time};

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // like `2015-11-02`
            Date::YMD { year, month, day } => write!(f, "{:04}-{:02}-{:02}", year, month, day),
            // like `2015-W45-01`
            Date::Week { year, ww, d } => write!(f, "{:04}-{:02}-{:02}", year, ww, d),
            // like `2015-306`
            Date::Ordinal { year, ddd } => write!(f, "{:04}-{:03}", year, ddd),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        write!(
            f,
            "{:02}:{:02}:{:02}.{}+{:02}:{:02}",
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
            self.timezone.offset_hours,
            self.timezone.offset_minutes
        )
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_zero() {
            write!(f, "P0D")?;
            return Ok(());
        }

        write!(f, "P")?;

        if self.years > 0 {
            write!(f, "{}Y", self.years)?
        }

        if self.months > 0 {
            write!(f, "{}M", self.months)?
        }

        if self.weeks > 0 {
            write!(f, "P{}W", self.weeks)?
        }

        if self.days > 0 {
            write!(f, "{}D", self.days)?
        }

        if self.has_time() {
            write!(f, "T")?
        }

        if self.hours > 0 {
            write!(f, "{}H", self.hours)?
        }
        if self.minutes > 0 {
            write!(f, "{}M", self.minutes)?
        }

        if self.milliseconds > 0 {
            write!(f, "{}.{}S", self.seconds, self.milliseconds)?
        } else if self.seconds > 0 {
            write!(f, "{}S", self.seconds)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::{duration, Stream};

    use super::*;

    fn test_duration_reparse(dur: Duration) {
        let serialized = format!("{}", dur);
        let reparsed = duration(&mut Stream::new(serialized.as_bytes())).unwrap();
        assert_eq!(dur, reparsed);
    }

    #[test]
    fn display_duration() {
        let duration = Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 16,
            hours: 23,
            minutes: 26,
            seconds: 59,
            milliseconds: 123,
        };
        test_duration_reparse(duration);
    }
}
