use core::convert::TryFrom;

impl TryFrom<crate::Time> for time::Time {
    type Error = ();

    fn try_from(iso: crate::Time) -> Result<Self, Self::Error> {
        time::Time::from_hms(
            iso.hour.try_into().unwrap(),
            iso.minute.try_into().unwrap(),
            iso.second.try_into().unwrap(),
        )
        .or(Err(()))
    }
}

impl crate::Time {
    /// create a [`time::Time`] if possible
    pub fn into_time(self) -> Option<time::Time> {
        time::Time::try_from(self).ok()
    }
}

impl TryFrom<crate::Date> for time::Date {
    type Error = ();

    fn try_from(iso: crate::Date) -> Result<Self, Self::Error> {
        match iso {
            crate::Date::YMD { year, month, day } => time::Date::from_calendar_date(
                year,
                time::Month::try_from(u8::try_from(month).unwrap()).unwrap(),
                day.try_into().unwrap(),
            ),

            crate::Date::Week { year, ww, d } => {
                let wd = time::Weekday::from(match d {
                    1 => time::Weekday::Monday,
                    2 => time::Weekday::Tuesday,
                    3 => time::Weekday::Wednesday,
                    4 => time::Weekday::Thursday,
                    5 => time::Weekday::Friday,
                    6 => time::Weekday::Saturday,
                    7 => time::Weekday::Sunday,
                    _ => panic!("Invalid day of week"),
                });

                time::Date::from_iso_week_date(year, ww.try_into().unwrap(), wd)
            }

            crate::Date::Ordinal { year, ddd } => {
                time::Date::from_ordinal_date(year, ddd.try_into().unwrap())
            }
        }
        .or(Err(()))
    }
}

impl crate::Date {
    /// create a [`time::Date`] if possible
    pub fn into_date(&self) -> Option<time::Date> {
        time::Date::try_from(*self).ok()
    }
}

impl TryFrom<crate::DateTime> for time::PrimitiveDateTime {
    type Error = ();

    fn try_from(iso: crate::DateTime) -> Result<Self, Self::Error> {
        let naive_date = time::Date::try_from(iso.date)?;
        let naive_time = time::Time::try_from(iso.time)?;
        Ok(naive_date.with_time(naive_time))
    }
}

impl crate::DateTime {
    /// create a [`time::PrimitiveDateTime`] if possible
    pub fn into_primitive(self) -> Option<time::PrimitiveDateTime> {
        time::PrimitiveDateTime::try_from(self).ok()
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
        };
        let time = time::Time::try_from(iso).unwrap();
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

        let date = time::Date::try_from(iso).unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month() as u8, 2);
        assert_eq!(date.day(), 8);
    }

    #[test]
    fn datetime_from_iso() {
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
                offset: Default::default(),
            },
        };

        let datetime = time::PrimitiveDateTime::try_from(iso).unwrap();
        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month() as u8, 2);
        assert_eq!(datetime.day(), 8);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 40);
        assert_eq!(datetime.second(), 0);
    }
}
