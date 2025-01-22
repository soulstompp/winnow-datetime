use chrono::TimeZone;
use core::convert::TryFrom;
use num_traits::FromPrimitive;

// TODO: we already do validity checks on our own,
// would be nice if we could use the unsafe versions of these conversions
impl TryFrom<crate::Date> for chrono::NaiveDate {
    type Error = ();

    fn try_from(iso: crate::Date) -> Result<Self, Self::Error> {
        let maybe = match iso {
            crate::Date::YMD { year, month, day } => {
                chrono::NaiveDate::from_ymd_opt(year, month, day)
            }

            crate::Date::Week { year, week, day } => chrono::Weekday::from_u32(day)
                .and_then(|d| chrono::NaiveDate::from_isoywd_opt(year, week, d)),

            crate::Date::Ordinal { year, day } => chrono::NaiveDate::from_yo_opt(year, day),
        };
        maybe.ok_or(())
    }
}

impl crate::Date {
    /// create a [`chrono::NativeDate`] if possible
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
        let iso = crate::Date::YMD {
            year: 2023,
            month: 2,
            day: 8,
        };
        let naive = chrono::NaiveDate::try_from(iso).unwrap();
        assert_eq!(naive.year(), 2023);
        assert_eq!(naive.month(), 2);
        assert_eq!(naive.day(), 8);
    }

    #[test]
    fn naivedate_from_ywd() {
        let iso = Date::Week {
            year: 2023,
            week: 6,
            day: 2,
        };
        let naive = chrono::NaiveDate::try_from(iso).unwrap();
        assert_eq!(naive.year(), 2023);
        assert_eq!(naive.month(), 2);
        assert_eq!(naive.day(), 8);
    }

    #[test]
    fn naivedate_from_ordinal() {
        let iso = crate::Date::Ordinal {
            year: 2023,
            day: 39,
        };
        let naive = chrono::NaiveDate::try_from(iso).unwrap();
        assert_eq!(naive.year(), 2023);
        assert_eq!(naive.month(), 2);
        assert_eq!(naive.day(), 8);
    }
}

impl TryFrom<crate::Time> for chrono::NaiveTime {
    type Error = ();
    fn try_from(iso: crate::Time) -> Result<Self, Self::Error> {
        chrono::NaiveTime::from_hms_opt(iso.hour, iso.minute, iso.second).ok_or(())
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

    fn try_from(iso: crate::DateTime) -> Result<Self, Self::Error> {
        let offset = iso.time.offset.unwrap_or(crate::Offset {
            offset_hours: 0,
            offset_minutes: 0,
        });

        let offset_minutes = offset.offset_hours * 3600 + offset.offset_minutes;
        let offset = chrono::FixedOffset::east_opt(offset_minutes).ok_or(())?;

        let naive_time = chrono::NaiveTime::try_from(iso.time)?;
        let naive_date_time = chrono::NaiveDate::try_from(iso.date)?.and_time(naive_time);

        offset
            .from_local_datetime(&naive_date_time)
            .single()
            .ok_or(())
    }
}

impl crate::DateTime {
    /// create a [`chrono::DateTime<chrono::FixedOffset>`] if possible
    pub fn into_fixed_offset(self) -> Option<chrono::DateTime<chrono::FixedOffset>> {
        chrono::DateTime::<chrono::FixedOffset>::try_from(self).ok()
    }

    /// create a [`chrono::NativeDateTime`] if possible
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
        let iso = crate::DateTime {
            date: crate::Date::YMD {
                year: 2023,
                month: 2,
                day: 8,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                millisecond: 0,
                offset: Some(crate::Offset {
                    offset_hours: 1,
                    offset_minutes: 23,
                }),
            },
        };
        let datetime = chrono::DateTime::try_from(iso).unwrap();

        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month(), 2);
        assert_eq!(datetime.day(), 8);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 40);
        assert_eq!(datetime.second(), 00);
        assert_eq!(datetime.offset().fix().local_minus_utc(), 3623);
    }

    #[test]
    fn datetime_from_iso_ymd_utc() {
        let iso = crate::DateTime {
            date: crate::Date::YMD {
                year: 2023,
                month: 2,
                day: 8,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                millisecond: 0,
                offset: Some(crate::Offset {
                    offset_hours: 0,
                    offset_minutes: 0,
                }),
            },
        };
        let datetime = chrono::DateTime::try_from(iso).unwrap();

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
        let iso = crate::DateTime {
            date: crate::Date::YMD {
                year: 2023,
                month: 2,
                day: 8,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                millisecond: 0,
                offset: Some(crate::Offset {
                    offset_hours: 0,
                    offset_minutes: 0,
                }),
            },
        };
        let datetime = chrono::DateTime::try_from(iso).unwrap();

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
        let iso = crate::DateTime {
            date: crate::Date::Week {
                year: 2023,
                week: 6,
                day: 2,
            },
            time: crate::Time {
                hour: 23,
                minute: 40,
                second: 0,
                millisecond: 0,
                offset: Some(crate::Offset {
                    offset_hours: 1,
                    offset_minutes: 23,
                }),
            },
        };
        let datetime = chrono::DateTime::try_from(iso).unwrap();

        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month(), 2);
        assert_eq!(datetime.day(), 8);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 40);
        assert_eq!(datetime.second(), 00);
        assert_eq!(datetime.offset().fix().local_minus_utc(), 3623);
    }
}
