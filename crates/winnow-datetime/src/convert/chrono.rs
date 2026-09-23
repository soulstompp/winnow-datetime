use crate::Offset;
use chrono::TimeZone;
use core::convert::TryFrom;

// TODO: we already do validity checks on our own,
// would be nice if we could use the unsafe versions of these conversions
impl TryFrom<crate::Date> for chrono::NaiveDate {
    type Error = ();

    fn try_from(d: crate::Date) -> Result<Self, Self::Error> {
        let maybe = match d {
            crate::Date::YMD { year, month, day } => {
                chrono::NaiveDate::from_ymd_opt(year, month, day)
            }

            crate::Date::Week { year, week, day } => {
                let wd = match day {
                    1 => chrono::Weekday::Mon,
                    2 => chrono::Weekday::Tue,
                    3 => chrono::Weekday::Wed,
                    4 => chrono::Weekday::Thu,
                    5 => chrono::Weekday::Fri,
                    6 => chrono::Weekday::Sat,
                    7 => chrono::Weekday::Sun,
                    _ => return Err(()),
                };

                chrono::NaiveDate::from_isoywd_opt(year, week, wd)
            }

            crate::Date::Ordinal { year, day } => chrono::NaiveDate::from_yo_opt(year, day),
        };
        maybe.ok_or(())
    }
}

impl crate::Date {
    /// create a [`chrono::NaiveDate`] if possible
    pub fn into_naive(&self) -> Option<chrono::NaiveDate> {
        chrono::NaiveDate::try_from(*self).ok()
    }
}

#[cfg(test)]
mod test_date {
    use crate::Date;
    use chrono::Datelike;
    use core::convert::TryFrom;

    #[test]
    fn naivedate_from_ymd() {
        let d = crate::Date::YMD {
            year: 2023,
            month: 2,
            day: 8,
        };
        let naive = chrono::NaiveDate::try_from(d).unwrap();
        assert_eq!(naive.year(), 2023);
        assert_eq!(naive.month(), 2);
        assert_eq!(naive.day(), 8);
    }

    #[test]
    fn naivedate_from_ywd() {
        let d = Date::Week {
            year: 2023,
            week: 6,
            day: 2,
        };
        let naive = chrono::NaiveDate::try_from(d).unwrap();
        assert_eq!(naive.year(), 2023);
        assert_eq!(naive.month(), 2);
        assert_eq!(naive.day(), 7);
    }

    #[test]
    fn naivedate_from_ordinal() {
        let d = crate::Date::Ordinal {
            year: 2023,
            day: 39,
        };
        let naive = chrono::NaiveDate::try_from(d).unwrap();
        assert_eq!(naive.year(), 2023);
        assert_eq!(naive.month(), 2);
        assert_eq!(naive.day(), 8);
    }
}

impl TryFrom<crate::Time> for chrono::NaiveTime {
    type Error = ();
    fn try_from(t: crate::Time) -> Result<Self, Self::Error> {
        chrono::NaiveTime::from_hms_nano_opt(t.hour, t.minute, t.second, t.nanosecond).ok_or(())
    }
}

impl crate::Time {
    /// create a [`chrono::NaiveTime`] if possible
    pub fn into_naive(self) -> Option<chrono::NaiveTime> {
        chrono::NaiveTime::try_from(self).ok()
    }
}

impl TryFrom<crate::DateTime> for chrono::DateTime<chrono::FixedOffset> {
    type Error = ();

    fn try_from(dt: crate::DateTime) -> Result<Self, Self::Error> {
        match dt.time.offset {
            Some(o) => {
                let offset_seconds = if let Offset::Fixed {
                    hours,
                    minutes,
                    critical: _,
                } = o
                {
                    hours * 3600 + minutes * 60
                } else {
                    0
                };

                let offset = chrono::FixedOffset::east_opt(offset_seconds).ok_or(())?;

                let naive_time = chrono::NaiveTime::try_from(dt.time)?;
                let naive_date_time = chrono::NaiveDate::try_from(dt.date)?.and_time(naive_time);

                offset
                    .from_local_datetime(&naive_date_time)
                    .single()
                    .ok_or(())
            }
            None => {
                //TODO: don't panic!
                panic!("no offset, wrong type used")
            }
        }
    }
}

impl crate::DateTime {
    /// create a [`chrono::DateTime<chrono::FixedOffset>`] if possible
    pub fn into_fixed_offset(self) -> Option<chrono::DateTime<chrono::FixedOffset>> {
        chrono::DateTime::<chrono::FixedOffset>::try_from(self).ok()
    }

    /// create a [`chrono::NaiveDateTime`] if possible
    pub fn into_naive(self) -> Option<chrono::NaiveDateTime> {
        self.into_fixed_offset().map(|fxed| fxed.naive_local())
    }
}

#[cfg(test)]
mod test_datetime {
    use chrono::{Datelike, Offset, Timelike};
    use core::convert::TryFrom;

    #[test]
    fn datetime_from_iso_ymd_offset() {
        let dt = crate::DateTime {
            date: crate::Date::YMD {
                year: 2023,
                month: 2,
                day: 8,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                nanosecond: 0,
                offset: Some(crate::Offset::Fixed {
                    hours: 1,
                    minutes: 23,
                    critical: false,
                }),
                time_zone: None,
                calendar: None,
            },
        };
        let datetime = chrono::DateTime::try_from(dt).unwrap();

        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month(), 2);
        assert_eq!(datetime.day(), 8);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 40);
        assert_eq!(datetime.second(), 00);
        assert_eq!(datetime.offset().fix().local_minus_utc(), 4980);
    }

    #[test]
    fn time_keeps_fraction() {
        let iso = crate::Time {
            hour: 23,
            minute: 40,
            second: 0,
            nanosecond: 870_479_000,
            offset: Default::default(),
            time_zone: None,
            calendar: None,
        };
        let time = chrono::NaiveTime::try_from(iso).unwrap();
        assert_eq!(time.nanosecond(), 870_479_000);
    }

    #[test]
    fn datetime_from_iso_ymd_utc() {
        let dt = crate::DateTime {
            date: crate::Date::YMD {
                year: 2023,
                month: 2,
                day: 8,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                nanosecond: 0,
                offset: Some(crate::Offset::Fixed {
                    hours: 0,
                    minutes: 0,
                    critical: false,
                }),
                time_zone: None,
                calendar: None,
            },
        };
        let datetime = chrono::DateTime::try_from(dt).unwrap();

        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month(), 2);
        assert_eq!(datetime.day(), 8);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 40);
        assert_eq!(datetime.second(), 00);
        assert_eq!(datetime.offset().fix().local_minus_utc(), 0);
    }

    #[test]
    fn datetime_from_iso_ymd_no_offset() {
        let dt = crate::DateTime {
            date: crate::Date::YMD {
                year: 2023,
                month: 2,
                day: 8,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                nanosecond: 0,
                offset: Some(crate::Offset::Fixed {
                    hours: 0,
                    minutes: 0,
                    critical: false,
                }),
                time_zone: None,
                calendar: None,
            },
        };
        let datetime = chrono::DateTime::try_from(dt).unwrap();

        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month(), 2);
        assert_eq!(datetime.day(), 8);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 40);
        assert_eq!(datetime.second(), 00);
        assert_eq!(datetime.offset().fix().local_minus_utc(), 0);
    }

    #[test]
    fn datetime_from_iso_ywd() {
        let dt = crate::DateTime {
            date: crate::Date::Week {
                year: 2023,
                week: 6,
                day: 2,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                nanosecond: 0,
                offset: Some(crate::Offset::Fixed {
                    hours: 1,
                    minutes: 23,
                    critical: false,
                }),
                time_zone: None,
                calendar: None,
            },
        };
        let datetime = chrono::DateTime::try_from(dt).unwrap();

        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month(), 2);
        assert_eq!(datetime.day(), 7);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 40);
        assert_eq!(datetime.second(), 00);
        assert_eq!(datetime.offset().fix().local_minus_utc(), 4980);
    }

    #[test]
    fn date_from_iso_ywd_sunday() {
        let date = chrono::NaiveDate::try_from(crate::Date::Week {
            year: 2023,
            week: 6,
            day: 7,
        })
        .unwrap();
        assert_eq!(date, chrono::NaiveDate::from_ymd_opt(2023, 2, 12).unwrap());
    }
}
