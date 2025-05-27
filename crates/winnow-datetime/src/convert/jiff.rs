use crate::Offset;
use core::convert::TryFrom;

impl TryFrom<crate::Time> for jiff::civil::Time {
    type Error = jiff::Error;

    fn try_from(t: crate::Time) -> Result<Self, Self::Error> {
        jiff::civil::Time::new(
            t.hour.try_into().unwrap(),
            t.minute.try_into().unwrap(),
            t.second.try_into().unwrap(),
            t.millisecond.try_into().unwrap(),
        )
    }
}

impl crate::Time {
    /// create a [`jiff::civil::Time`] if possible
    pub fn into_civil_time(self) -> Option<jiff::civil::Time> {
        jiff::civil::Time::try_from(self).ok()
    }
}

impl TryFrom<crate::Date> for jiff::civil::Date {
    type Error = jiff::Error;

    fn try_from(d: crate::Date) -> Result<Self, Self::Error> {
        match d {
            crate::Date::YMD { year, month, day } => {
                jiff::civil::Date::new(year as i16, month as i8, day.try_into().unwrap())
            }

            crate::Date::Week { year, week, day } => {
                let wd = match day {
                    1 => jiff::civil::Weekday::Monday,
                    2 => jiff::civil::Weekday::Tuesday,
                    3 => jiff::civil::Weekday::Wednesday,
                    4 => jiff::civil::Weekday::Thursday,
                    5 => jiff::civil::Weekday::Friday,
                    6 => jiff::civil::Weekday::Saturday,
                    7 => jiff::civil::Weekday::Sunday,
                    _ => panic!("Invalid day of week"),
                };

                Ok(jiff::civil::ISOWeekDate::new(
                    year.try_into().unwrap(),
                    week.try_into().unwrap(),
                    wd,
                )?
                .into())
            }

            crate::Date::Ordinal { year, day } => {
                jiff::civil::Date::new(year.try_into().unwrap(), 1, 1)?
                    .with()
                    .day_of_year(day.try_into().unwrap())
                    .build()
            }
        }
    }
}

impl crate::Date {
    /// create a [`jeff::civil::Date`] if possible
    pub fn into_civil_date(self) -> Option<jiff::civil::Date> {
        jiff::civil::Date::try_from(self).ok()
    }
}

impl TryFrom<crate::DateTime> for jiff::civil::DateTime {
    type Error = jiff::Error;

    fn try_from(dt: crate::DateTime) -> Result<Self, Self::Error> {
        let naive_date = jiff::civil::Date::try_from(dt.date)?;
        let naive_time = jiff::civil::Time::try_from(dt.time)?;

        Ok(naive_date.to_datetime(naive_time))
    }
}

impl TryFrom<crate::DateTime> for jiff::Zoned {
    type Error = jiff::Error;

    fn try_from(dt: crate::DateTime) -> Result<Self, Self::Error> {
        let naive_date = jiff::civil::Date::try_from(dt.date)?;
        let naive_time = jiff::civil::Time::try_from(dt.time.clone())?;
        let naive_datetime = naive_date.to_datetime(naive_time);

        let o_seconds = match dt.time.offset {
            Some(o) => match o {
                Offset::Fixed {
                    hours,
                    minutes,
                    critical: _,
                } => hours * 3600 + minutes * 60,
                Offset::LocalUnknown { critical: _ } => 0,
            },
            None => 0,
        };

        let offset = jiff::tz::Offset::from_seconds(o_seconds.try_into().unwrap())?;

        let tz = jiff::tz::TimeZone::fixed(offset);
        naive_datetime.to_zoned(tz)
    }
}

impl crate::DateTime {
    /// create a [`jiff::civil::DateTime`] if possible
    pub fn into_datetime(self) -> Option<jiff::civil::DateTime> {
        jiff::civil::DateTime::try_from(self).ok()
    }

    pub fn into_zoned(self) -> Option<jiff::Zoned> {
        jiff::Zoned::try_from(self).ok()
    }
}

impl TryFrom<crate::Duration> for jiff::Span {
    type Error = jiff::Error;

    fn try_from(d: crate::Duration) -> Result<Self, Self::Error> {
        let ms = d.milliseconds.unwrap_or(0.0).trunc();
        let ns = (d.milliseconds.unwrap_or(0.0).fract() * 1_000_000.0).round();

        Ok(jiff::Span::new()
            .years(d.years)
            .months(d.months)
            .weeks(d.weeks)
            .days(d.days)
            .hours(d.hours)
            .minutes(d.minutes)
            .seconds(d.seconds)
            .milliseconds(ms as i64)
            .nanoseconds(ns as i64))
    }
}

#[cfg(test)]
mod date_and_time {
    use core::convert::TryFrom;

    #[test]
    fn time_from_hms() {
        let iso = crate::Time {
            hour: 23,
            minute: 40,
            second: 0,
            millisecond: 0,
            offset: Default::default(),
            time_zone: None,
            calendar: None,
        };
        let time = jiff::civil::Time::try_from(iso).unwrap();
        assert_eq!(time.hour(), 23);
        assert_eq!(time.minute(), 40);
        assert_eq!(time.second(), 0);
    }

    #[test]
    fn date_from_ymd() {
        let iso = crate::Date::YMD {
            year: 2023,
            month: 2,
            day: 8,
        };

        let date = jiff::civil::Date::try_from(iso).unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month() as u8, 2);
        assert_eq!(date.day(), 8);
    }

    #[test]
    fn datetime_from_ymd_hms() {
        let dt = crate::DateTime {
            date: crate::Date::YMD {
                year: 2024,
                month: 3,
                day: 9,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                millisecond: 0,
                offset: Default::default(),
                time_zone: None,
                calendar: None,
            },
        };

        let datetime = time::PrimitiveDateTime::try_from(dt).unwrap();
        assert_eq!(datetime.year(), 2024);
        assert_eq!(datetime.month() as u8, 3);
        assert_eq!(datetime.day(), 9);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 40);
        assert_eq!(datetime.second(), 0);
    }

    #[test]
    fn date_from_yddd() {
        let dt = crate::Date::Ordinal {
            year: 2024,
            day: 122,
        };

        let date = jiff::civil::Date::try_from(dt).unwrap();
        assert_eq!(date, jiff::civil::date(2024, 5, 1));
    }

    #[test]
    fn span_from_duration() {
        let d = crate::Duration {
            years: 5,
            months: 4,
            weeks: 3,
            hours: 2,
            days: 1,
            minutes: 30,
            seconds: 15,
            milliseconds: Some(500.544),
        };

        let s = jiff::Span::try_from(d).unwrap();

        assert_eq!(s.get_years(), 5);
        assert_eq!(s.get_months(), 4);
        assert_eq!(s.get_weeks(), 3);
        assert_eq!(s.get_hours(), 2);
        assert_eq!(s.get_minutes(), 30);
        assert_eq!(s.get_seconds(), 15);
        assert_eq!(s.get_milliseconds(), 500);
        assert_eq!(s.get_nanoseconds(), 544006);
    }
}
