use super::*;
use crate::parsers::interval;
use winnow::combinator::eof;
use winnow_datetime::parsers::{date_day, date_month};
use winnow_datetime::Stream;

#[test]
fn test_date_year() {
    assert_eq!(date_year(&mut "2015".as_bstr()).unwrap(), 2015);
    assert_eq!(date_year(&mut "+2015".as_bstr()).unwrap(), 2015);
    assert!(date_year(&mut "-333".as_bstr()).is_err());
    assert_eq!(date_year(&mut "-0333".as_bstr()).unwrap(), -333);
    assert_eq!(date_year(&mut "2015-".as_bstr()).unwrap(), 2015);
    assert!(date_year(&mut Stream::new(b"abcd")).is_err());
    assert!(date_year(&mut Stream::new(b"2a03")).is_err());
}

#[test]
fn test_date_month() {
    assert_eq!(date_month(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(date_month(&mut "06".as_bstr()).unwrap(), 6);
    assert_eq!(date_month(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(date_month(&mut "12-".as_bstr()).unwrap(), 12);

    assert!(date_month(&mut Stream::new(b"13\n")).is_err());
    assert!(date_month(&mut Stream::new(b"00\n")).is_err());
}

#[test]
fn test_date_day() {
    assert_eq!(date_day(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(date_day(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(date_day(&mut "20".as_bstr()).unwrap(), 20);
    assert_eq!(date_day(&mut "28".as_bstr()).unwrap(), 28);
    assert_eq!(date_day(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(date_day(&mut "31".as_bstr()).unwrap(), 31);
    assert_eq!(date_day(&mut "31-".as_bstr()).unwrap(), 31);

    assert!(date_day(&mut Stream::new(b"00")).is_err());
    assert!(date_day(&mut Stream::new(b"32")).is_err());
}

#[test]
fn test_time_hour() {
    assert_eq!(time_hour(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_hour(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_hour(&mut "06".as_bstr()).unwrap(), 6);
    assert_eq!(time_hour(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(time_hour(&mut "13".as_bstr()).unwrap(), 13);
    assert_eq!(time_hour(&mut "20".as_bstr()).unwrap(), 20);

    assert!(time_hour(&mut "24".as_bstr()).is_err());
    assert!(time_hour(&mut "25".as_bstr()).is_err());
    assert!(time_hour(&mut "30".as_bstr()).is_err());
    assert!(time_hour(&mut "ab".as_bstr()).is_err());
}

#[test]
fn test_time_minute() {
    assert_eq!(time_minute(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_minute(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_minute(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(time_minute(&mut "59".as_bstr()).unwrap(), 59);

    assert!(time_minute(&mut Stream::new(b"60")).is_err());
    assert!(time_minute(&mut Stream::new(b"61")).is_err());
    assert!(time_minute(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_time_second() {
    assert_eq!(time_second(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_second(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_second(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(time_second(&mut "59".as_bstr()).unwrap(), 59);
    assert_eq!(time_second(&mut "60".as_bstr()).unwrap(), 60);

    assert!(time_second(&mut Stream::new(b"61")).is_err());
    assert!(time_second(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_date() {
    assert!(date(&mut Stream::new(b"201")).is_err());
    assert!(date(&mut Stream::new(b"2015p00p00")).is_err());
    assert!(date(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_time() {
    assert!(time(&mut Stream::new(b"20:")).is_err());
    assert!(time(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_time_with_timezone() {
    assert!(time(&mut Stream::new(b"20:")).is_err());
    assert!(time(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_date_iso_week_date() {
    assert!(terminated(date_ywd, eof)
        .parse_next(&mut Stream::new(b"2015-W06-8"))
        .is_err());
    assert!(terminated(date_ywd, eof)
        .parse_next(&mut Stream::new(b"2015-W068"))
        .is_err());
    assert!(terminated(date_ywd, eof)
        .parse_next(&mut Stream::new(b"2015-W06-0"))
        .is_err());
    assert!(terminated(date_ywd, eof)
        .parse_next(&mut Stream::new(b"2015-W00-2"))
        .is_err());
    assert!(terminated(date_ywd, eof)
        .parse_next(&mut Stream::new(b"2015-W54-2"))
        .is_err());
    assert!(terminated(date_ywd, eof)
        .parse_next(&mut Stream::new(b"2015-W542"))
        .is_err());
}

#[test]
fn test_date_ordinal_date() {
    // not valid here either
    assert!(date_yddd(&mut Stream::new(b"2015-400")).is_err());
}

#[test]
fn format_equivalence() {
    assert_eq!(
        datetime(&mut Stream::new(b"2001-02-03T04:05:06+07:00")),
        datetime(&mut Stream::new(b"20010203T040506+0700"))
    );
    assert_eq!(
        datetime(&mut Stream::new(b"2001-02-03T04:05:06+07:00")),
        datetime(&mut Stream::new(b"20010203T04:05:06+0700"))
    );
    assert_eq!(
        datetime(&mut Stream::new(b"2001-02-03T04:05:00+07:00")),
        datetime(&mut Stream::new(b"20010203T0405+0700"))
    );
    assert_eq!(
        datetime(&mut Stream::new(b"20010203T0405+0700")),
        datetime(&mut Stream::new(b"2001-02-03T0405+0700"))
    );
    assert_eq!(
        datetime(&mut Stream::new(b"20010203T040506+0700")),
        datetime(&mut Stream::new(b"2001-02-03T040506+0700"))
    );
    assert_eq!(
        datetime(&mut Stream::new(b"20010203T040506+0000")),
        datetime(&mut Stream::new(b"20010203T040506Z"))
    );
    assert_eq!(
        datetime(&mut Stream::new(b"2015W056T04:05:06+07:00")),
        datetime(&mut Stream::new(b"2015-W05-6T04:05:06+07:00"))
    );
}

#[test]
fn test_datetime_error() {
    let test_datetimes = vec!["ppp", "dumd-di-duTmd:iu:m"];

    for iso_string in test_datetimes {
        let res = datetime(&mut Stream::new(iso_string.as_bytes()));
        assert!(res.is_err());
    }
}

#[test]
fn disallows_notallowed() {
    assert!(time(&mut Stream::new(b"30:90:90")).is_err());
    assert!(date(&mut Stream::new(b"0000-20-40")).is_err());
    assert!(datetime(&mut Stream::new(b"2001-w05-6t04:05:06.123z")).is_err());
}

#[test]
fn test_duration_year() {
    assert_eq!(
        duration_part_year(&mut "2019Y".as_bstr()).unwrap(),
        (DurationPart { whole: 2019, frac: None })
    );
    assert_eq!(
        duration_part_year(&mut "0Y".as_bstr()).unwrap(),
        (DurationPart { whole: 0, frac: None })
    );
    assert_eq!(
        duration_part_year(&mut "10000Y".as_bstr()).unwrap(),
        (DurationPart { whole: 10000, frac: None })
    );
    assert!(duration_part_year(&mut Stream::new(b"abcd")).is_err());
    assert!(duration_part_year(&mut Stream::new(b"-1")).is_err());
}

#[test]
fn test_duration_month() {
    assert_eq!(
        duration_part_month(&mut "6M".as_bstr()).unwrap(),
        (DurationPart { whole: 6, frac: None })
    );
    assert_eq!(
        duration_part_month(&mut "0M".as_bstr()).unwrap(),
        (DurationPart { whole: 0, frac: None })
    );
    assert_eq!(
        duration_part_month(&mut "12M".as_bstr()).unwrap(),
        (DurationPart { whole: 12, frac: None })
    );

    assert!(duration_part_month(&mut Stream::new(b"ab")).is_err());
    assert!(duration_part_month(&mut Stream::new(b"-1")).is_err());
    assert!(duration_part_month(&mut Stream::new(b"13")).is_err());
}

#[test]
fn test_duration_week() {
    assert_eq!(
        duration_part_week(&mut "26W".as_bstr()).unwrap(),
        DurationPart { whole: 26, frac: None }
    );
    assert_eq!(
        duration_part_week(&mut "0W".as_bstr()).unwrap(),
        DurationPart { whole: 0, frac: None }
    );
    assert_eq!(
        duration_part_week(&mut "52W".as_bstr()).unwrap(),
        DurationPart { whole: 52, frac: None }
    );
    assert!(duration_part_week(&mut Stream::new(b"ab")).is_err());
    assert!(duration_part_week(&mut Stream::new(b"-1")).is_err());
    assert!(duration_part_week(&mut Stream::new(b"53")).is_err());
}

#[test]
fn test_duration_day() {
    assert_eq!(
        duration_part_day(&mut "16D".as_bstr()).unwrap(),
        DurationPart { whole: 16, frac: None }
    );
    assert_eq!(
        duration_part_day(&mut "0D".as_bstr()).unwrap(),
        DurationPart { whole: 0, frac: None }
    );
    assert_eq!(
        duration_part_day(&mut "31D".as_bstr()).unwrap(),
        DurationPart { whole: 31, frac: None }
    );
    assert!(duration_part_day(&mut Stream::new(b"ab")).is_err());
    assert!(duration_part_day(&mut Stream::new(b"-1")).is_err());
    assert!(duration_part_day(&mut Stream::new(b"32")).is_err());
}

#[test]
fn test_duration_hour() {
    assert_eq!(
        duration_part_hour(&mut "12H".as_bstr()).unwrap(),
        DurationPart { whole: 12, frac: None }
    );
    assert_eq!(
        duration_part_hour(&mut "0H".as_bstr()).unwrap(),
        DurationPart { whole: 0, frac: None }
    );
    assert_eq!(
        duration_part_hour(&mut "24H".as_bstr()).unwrap(),
        DurationPart { whole: 24, frac: None }
    );
    assert!(duration_part_hour(&mut Stream::new(b"ab")).is_err());
    assert!(duration_part_hour(&mut Stream::new(b"-1")).is_err());
    assert!(duration_part_hour(&mut Stream::new(b"25")).is_err());
}

#[test]
fn test_duration_minute() {
    assert_eq!(
        duration_part_minute(&mut "30M".as_bstr()).unwrap(),
        DurationPart { whole: 30, frac: None }
    );
    assert_eq!(
        duration_part_minute(&mut "0M".as_bstr()).unwrap(),
        DurationPart { whole: 0, frac: None }
    );
    assert_eq!(
        duration_part_minute(&mut "60M".as_bstr()).unwrap(),
        DurationPart { whole: 60, frac: None }
    );
    assert!(duration_part_minute(&mut Stream::new(b"ab")).is_err());
    assert!(duration_part_minute(&mut Stream::new(b"-1")).is_err());
    assert!(duration_part_minute(&mut Stream::new(b"61")).is_err());
}

#[test]
fn test_duration_second_and_millisecond1() {
    assert_eq!(
        duration_part_second(&mut "30S".as_bstr()).unwrap(),
        DurationPart{ whole: 30, frac: None }
    );
    assert_eq!(
        duration_part_second(&mut "0S".as_bstr()).unwrap(),
        DurationPart{ whole: 0, frac: None }
    );
    assert_eq!(
        duration_part_second(&mut "60S".as_bstr()).unwrap(),
        DurationPart{ whole: 60, frac: None }
    );
    assert_eq!(
        duration_part_second(&mut "1,23S".as_bstr()).unwrap(),
        DurationPart{ whole: 1, frac: Some(0.23) }
    );
    assert_eq!(
        duration_part_second(&mut "2.34S".as_bstr()).unwrap(),
        DurationPart{ whole: 2, frac: Some(0.34) }
    );
    assert!(duration_part_second(&mut Stream::new(b"abS")).is_err());
    assert!(duration_part_second(&mut Stream::new(b"-1S")).is_err());
}

#[test]
fn test_duration_time() {
    assert_eq!(
        duration_part_time(&mut "T1H2M3S".as_bstr()).unwrap(),
        (
            Some(DurationPart { whole: 1, frac: None }),
            Some(DurationPart { whole: 2, frac: None }),
            Some(DurationPart { whole: 3, frac: None })
        )
    );
    assert_eq!(
        duration_part_time(&mut "T10H12M30S".as_bstr()).unwrap(),
        (
            Some(DurationPart { whole: 10, frac: None }),
            Some(DurationPart { whole: 12, frac: None }),
            Some(DurationPart { whole: 30, frac: None })
        )
    );
    assert_eq!(
        duration_part_time(&mut "T1H3S".as_bstr()).unwrap(),
        (
            Some(DurationPart { whole: 1, frac: None }),
            None,
            Some(DurationPart { whole: 3, frac: None })
        )
    );

    assert_eq!(
        duration_part_time(&mut "T2M".as_bstr()).unwrap(),
        (None, Some(DurationPart { whole: 2, frac: None }), None)
    );
    assert_eq!(
        duration_part_time(&mut "T1H2M3,4S".as_bstr()).unwrap(),
        (
            Some(DurationPart { whole: 1, frac: None }),
            Some(DurationPart { whole: 2, frac: None }),
            Some(DurationPart { whole: 3, frac: Some(0.4) })
        )
    );
    assert_eq!(
        duration_part_time(&mut "T1H23.4S".as_bstr()).unwrap(),
        (
            Some(DurationPart { whole: 1, frac: None }),
            None,
            Some(DurationPart { whole: 23, frac: Some(0.4) })
        )
    );
    assert_eq!(
        duration_part_time(&mut "T0,123S".as_bstr()).unwrap(),
        (
            None,
            None,
            Some(
                DurationPart { whole: 0, frac: Some(0.123) }
            )
        )
    );
    assert_eq!(
        duration_part_time(&mut "T0123S".as_bstr()).unwrap(),
        (
            None,
            None,
            Some(
                DurationPart { whole: 123, frac: None },
            )
        )
    );
}

#[test]
fn test_duration_ymdhms_error() {
    assert!(duration(&mut Stream::new(b"")).is_err());
    assert!(duration(&mut Stream::new(b"P")).is_err()); // empty duration is not 0 seconds
    assert!(duration(&mut Stream::new(b"1Y2M3DT4H5M6S")).is_err()); // missing P at start
    assert!(duration(&mut Stream::new(b"T4H5M6S")).is_err()); // missing P,
}

#[test]
fn test_duration_weeks_error() {
    assert!(duration(&mut Stream::new(b"")).is_err());
    assert!(duration(&mut Stream::new(b"P")).is_err()); // empty duration is not 0 seconds
    assert!(duration(&mut Stream::new(b"P1")).is_err()); // missing W after number
    assert!(duration(&mut Stream::new(b"PW")).is_err()); // missing number
}

#[test]
fn test_duration_second() {
    assert_eq!(
        duration(&mut "PT30S".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 30,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "PT30.123S".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 30,
            milliseconds: Some(0.123)
        }
    );
    assert_eq!(
        duration(&mut "P2021Y11M16DT23H26M59.123S".as_bstr()).unwrap(),
        Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 16,
            hours: 23,
            minutes: 26,
            seconds: 59,
            milliseconds: Some(0.123)
        }
    );
}

#[test]
fn duration_roundtrip() {
    assert_eq!(
        duration(&mut "P0W".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P2021Y11M16DT23H26M59S".as_bstr()).unwrap(),
        Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 16,
            hours: 23,
            minutes: 26,
            seconds: 59,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P2021Y11M16DT23H26M".as_bstr()).unwrap(),
        Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 16,
            hours: 23,
            minutes: 26,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P2021Y11M16DT23H".as_bstr()).unwrap(),
        Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 16,
            hours: 23,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P2021Y11M16D".as_bstr()).unwrap(),
        Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 16,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P2021Y11M16DT1S".as_bstr()).unwrap(),
        Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 16,
            hours: 0,
            minutes: 0,
            seconds: 1,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P2021Y11M16DT0.471S".as_bstr()).unwrap(),
        Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 16,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: Some(0.471)
        }
    );
    assert_eq!(
        duration(&mut "P2021Y11M".as_bstr()).unwrap(),
        Duration {
            years: 2021,
            months: 11,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P11M".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 11,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P16D".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 16,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "P0D".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
}

#[test]
fn duration_multi_digit_hour() {
    assert_eq!(
        duration(&mut "PT12H".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 12,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "PT8760H".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 365 * 24,
            minutes: 0,
            seconds: 0,
            milliseconds: None
        }
    );
}

#[test]
fn duration_multi_digit_minute() {
    assert_eq!(
        duration(&mut "PT15M".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 15,
            seconds: 0,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "PT600M".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 600,
            seconds: 0,
            milliseconds: None
        }
    );
}

#[test]
fn duration_multi_digit_second() {
    assert_eq!(
        duration(&mut "PT16S".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 16,
            milliseconds: None
        }
    );
    assert_eq!(
        duration(&mut "PT900S".as_bstr()).unwrap(),
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 900,
            milliseconds: None
        }
    );
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_day() {
    assert_eq!(duration(&mut "P365D".as_bstr()).unwrap(),
Duration { years: 0,
months: 0,
weeks: 0,
days: 365,
hours: 0,
minutes: 0,
seconds: 0,
milliseconds: None });
    assert_eq!(duration(&mut "P36500D".as_bstr()).unwrap(),
Duration { years: 0,
months: 0,
weeks: 0,
days: 36500,
hours: 0,
minutes: 0,
seconds: 0,
milliseconds: None });
}

#[test]
fn partial_date_parsing() {
    // Year
    assert_eq!(
        partial_date(&mut "2015".as_bstr()).unwrap(),
        PartialDate::Year { year: Some(2015) }
    );
    // YMD
    assert_eq!(
        partial_date(&mut "2015-06-26".as_bstr()).unwrap(),
        PartialDate::YMD {
            year: Some(2015),
            month: Some(6),
            day: Some(26)
        }
    );
    assert_eq!(
        partial_date(&mut "2015-06".as_bstr()).unwrap(),
        PartialDate::YMD {
            year: Some(2015),
            month: Some(6),
            day: None
        }
    );
    // YWD
    assert_eq!(
        partial_date(&mut "2015-W05-6".as_bstr()).unwrap(),
        PartialDate::YWD {
            year: Some(2015),
            week: Some(5),
            day: Some(6)
        }
    );
    assert_eq!(
        partial_date(&mut "2015-W05".as_bstr()).unwrap(),
        PartialDate::YWD {
            year: Some(2015),
            week: Some(5),
            day: None
        }
    );
    //Ordinal
    assert_eq!(
        partial_date(&mut "2015-156".as_bstr()).unwrap(),
        PartialDate::YDDD {
            year: Some(2015),
            day: Some(156)
        }
    );
    assert_eq!(
        partial_date(&mut "2015-156".as_bstr()).unwrap(),
        PartialDate::YDDD {
            year: Some(2015),
            day: Some(156)
        }
    );
}

#[test]
fn partial_datetime_parsing() {
    // Year
    assert_eq!(
        partial_datetime(&mut "2015".as_bstr()).unwrap(),
        PartialDateTime {
            date: Some(PartialDate::Year { year: Some(2015) }),
            time: None,
        }
    );
    // YMD
    assert_eq!(
        partial_datetime(&mut "2015-06-26".as_bstr()).unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: Some(26)
            }),
            time: None,
        }
    );
    assert_eq!(
        partial_datetime(&mut "2015-06".as_bstr()).unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: None
            }),
            time: None,
        }
    );
    // YWD
    assert_eq!(
        partial_datetime(&mut "2015-W05-6".as_bstr()).unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YWD {
                year: Some(2015),
                week: Some(5),
                day: Some(6)
            }),
            time: None,
        }
    );
    assert_eq!(
        partial_datetime(&mut "2015-W05-1".as_bstr()).unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YWD {
                year: Some(2015),
                week: Some(5),
                day: Some(1)
            }),
            time: None,
        }
    );
    assert_eq!(
        partial_datetime(&mut "2015-W05".as_bstr()).unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YWD {
                year: Some(2015),
                week: Some(5),
                day: None
            }),
            time: None,
        }
    );
    //Ordinal
    assert_eq!(
        partial_datetime(&mut "2015-156".as_bstr()).unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YDDD {
                year: Some(2015),
                day: Some(156)
            }),
            time: None,
        }
    );
    assert_eq!(
        partial_datetime(&mut "2015-156".as_bstr()).unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YDDD {
                year: Some(2015),
                day: Some(156)
            }),
            time: None,
        }
    );
}

#[test]
fn partial_end_dates_ymd() {
    assert_eq!(
        partial_end_date(
            &mut "2015-06-26".as_bstr(),
            &PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: Some(25)
            }
        )
        .unwrap(),
        PartialDate::YMD {
            year: Some(2015),
            month: Some(6),
            day: Some(26)
        }
    );
    assert_eq!(
        partial_end_datetime(
            &mut "2015-06-26".as_bstr(),
            &PartialDateTime {
                date: Some(PartialDate::YMD {
                    year: Some(2015),
                    month: Some(6),
                    day: Some(25)
                }),
                time: None,
            }
        )
        .unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: Some(26)
            }),
            time: None,
        }
    );
    assert_eq!(
        partial_end_date(
            &mut "06-26".as_bstr(),
            &PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: Some(25)
            }
        )
        .unwrap(),
        PartialDate::YMD {
            year: Some(2015),
            month: Some(6),
            day: Some(26)
        }
    );
    assert_eq!(
        partial_end_datetime(
            &mut "06-26".as_bstr(),
            &PartialDateTime {
                date: Some(PartialDate::YMD {
                    year: Some(2015),
                    month: Some(6),
                    day: Some(25)
                }),
                time: None,
            }
        )
        .unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: Some(26)
            }),
            time: None,
        }
    );
    assert_eq!(
        partial_end_date(
            &mut "26".as_bstr(),
            &PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: Some(25)
            }
        )
        .unwrap(),
        PartialDate::YMD {
            year: Some(2015),
            month: Some(6),
            day: Some(26)
        }
    );
    assert_eq!(
        partial_end_datetime(
            &mut "26".as_bstr(),
            &PartialDateTime {
                date: Some(PartialDate::YMD {
                    year: Some(2015),
                    month: Some(6),
                    day: Some(25)
                }),
                time: None,
            }
        )
        .unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: Some(26)
            }),
            time: None,
        }
    );
}

#[test]
fn partial_end_dates_ywd() {
    assert_eq!(
        partial_end_date(
            &mut "2024-W51-4".as_bstr(),
            &PartialDate::YWD {
                year: Some(2024),
                week: Some(51),
                day: Some(3)
            }
        )
        .unwrap(),
        PartialDate::YWD {
            year: Some(2024),
            week: Some(51),
            day: Some(4)
        }
    );
    assert_eq!(
        partial_end_datetime(
            &mut "2024-W51-4".as_bstr(),
            &PartialDateTime {
                date: Some(PartialDate::YWD {
                    year: Some(2024),
                    week: Some(51),
                    day: Some(3)
                }),
                time: None,
            }
        )
        .unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YWD {
                year: Some(2024),
                week: Some(51),
                day: Some(4)
            }),
            time: None,
        }
    );
    assert_eq!(
        partial_end_date(
            &mut "W51-4".as_bstr(),
            &PartialDate::YWD {
                year: Some(2024),
                week: Some(51),
                day: Some(3)
            }
        )
        .unwrap(),
        PartialDate::YWD {
            year: Some(2024),
            week: Some(51),
            day: Some(4)
        }
    );
    assert_eq!(
        partial_end_datetime(
            &mut "W51-4".as_bstr(),
            &PartialDateTime {
                date: Some(PartialDate::YWD {
                    year: Some(2024),
                    week: Some(51),
                    day: Some(3)
                }),
                time: None,
            }
        )
        .unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YWD {
                year: Some(2024),
                week: Some(51),
                day: Some(4)
            }),
            time: None,
        }
    );
    assert_eq!(
        partial_end_date(
            &mut "4".as_bstr(),
            &PartialDate::YWD {
                year: Some(2024),
                week: Some(51),
                day: Some(3)
            }
        )
        .unwrap(),
        PartialDate::YWD {
            year: Some(2024),
            week: Some(51),
            day: Some(4)
        }
    );
    assert_eq!(
        partial_end_datetime(
            &mut "4".as_bstr(),
            &PartialDateTime {
                date: Some(PartialDate::YWD {
                    year: Some(2024),
                    week: Some(51),
                    day: Some(3)
                }),
                time: None,
            }
        )
        .unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YWD {
                year: Some(2024),
                week: Some(51),
                day: Some(4)
            }),
            time: None,
        }
    );
}

#[test]
fn partial_end_dates_yddd() {
    assert_eq!(
        partial_end_date(
            &mut "2025-083".as_bstr(),
            &PartialDate::YDDD {
                year: Some(2025),
                day: Some(82)
            }
        )
        .unwrap(),
        PartialDate::YDDD {
            year: Some(2025),
            day: Some(83)
        }
    );
    assert_eq!(
        partial_end_datetime(
            &mut "083".as_bstr(),
            &PartialDateTime {
                date: Some(PartialDate::YDDD {
                    year: Some(2025),
                    day: Some(82)
                }),
                time: None,
            }
        )
        .unwrap(),
        PartialDateTime {
            date: Some(PartialDate::YDDD {
                year: Some(2025),
                day: Some(83)
            }),
            time: None,
        }
    );
}

#[test]
fn partial_time_parsing() {
    assert_eq!(
        partial_time(&mut "12:01:30".as_bstr()).unwrap(),
        PartialTime {
            hour: Some(12),
            minute: Some(1),
            second: Some(30),
            millisecond: None,
            offset: None,
        }
    );
    assert_eq!(
        partial_time(&mut "12:01".as_bstr()).unwrap(),
        PartialTime {
            hour: Some(12),
            minute: Some(1),
            second: None,
            millisecond: None,
            offset: None,
        }
    );
    assert_eq!(
        partial_time(&mut "12:01:30.123".as_bstr()).unwrap(),
        PartialTime {
            hour: Some(12),
            minute: Some(1),
            second: Some(30),
            millisecond: Some(123),
            offset: None,
        }
    );
}

#[test]
fn partial_end_time_parsing() {
    assert_eq!(
        partial_end_time(
            &mut "12:01:30".as_bstr(),
            &PartialTime {
                hour: Some(12),
                minute: Some(1),
                second: Some(29),
                millisecond: None,
                offset: None,
            }
        )
        .unwrap(),
        PartialTime {
            hour: Some(12),
            minute: Some(1),
            second: Some(30),
            millisecond: None,
            offset: None,
        }
    );
    assert_eq!(
        partial_end_time(
            &mut "12:01".as_bstr(),
            &PartialTime {
                hour: Some(12),
                minute: Some(0),
                second: None,
                millisecond: None,
                offset: None,
            }
        )
        .unwrap(),
        PartialTime {
            hour: Some(12),
            minute: Some(1),
            second: None,
            millisecond: None,
            offset: None,
        }
    );
    assert_eq!(
        partial_end_time(
            &mut "12:01:30.123".as_bstr(),
            &PartialTime {
                hour: Some(12),
                minute: Some(1),
                second: Some(30),
                millisecond: Some(122),
                offset: None,
            }
        )
        .unwrap(),
        PartialTime {
            hour: Some(12),
            minute: Some(1),
            second: Some(30),
            millisecond: Some(123),
            offset: None,
        }
    );
}

#[test]
fn interval_closed() {
    assert_eq!(
        interval(&mut "2015-06-25/2015-06-26".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2015),
                        month: Some(6),
                        day: Some(25)
                    }),
                    time: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2015),
                        month: Some(6),
                        day: Some(26)
                    }),
                    time: None,
                },
            },
        }
    );

    assert_eq!(
        interval(&mut "2015-06-25 12:00:00Z/2015-06-26 12:00:00Z".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2015),
                        month: Some(6),
                        day: Some(25)
                    }),
                    time: Some(PartialTime {
                        hour: Some(12),
                        minute: Some(0),
                        second: Some(0),
                        millisecond: None,
                        offset: Some(Offset {
                            offset_hours: 0,
                            offset_minutes: 0,
                        })
                    }),
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2015),
                        month: Some(6),
                        day: Some(26)
                    }),
                    time: Some(PartialTime {
                        hour: Some(12),
                        minute: Some(0),
                        second: Some(0),
                        millisecond: None,
                        offset: Some(Offset {
                            offset_hours: 0,
                            offset_minutes: 0,
                        })
                    }),
                },
            },
        }
    );
}

#[test]
fn interval_closed_partial_ymd_end_date() {
    // Partial end: 2024-12-22/12-23
    assert_eq!(
        interval(&mut "2024-12-22/12-23".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2024),
                        month: Some(12),
                        day: Some(22)
                    }),
                    time: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2024),
                        month: Some(12),
                        day: Some(23)
                    }),
                    time: None,
                },
            },
        }
    );

    // Partial start: 2024-12-22/12-23
    assert_eq!(
        interval(&mut "2024-12-22/23".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2024),
                        month: Some(12),
                        day: Some(22)
                    }),
                    time: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2024),
                        month: Some(12),
                        day: Some(23)
                    }),
                    time: None,
                },
            },
        }
    );
}

#[test]
fn interval_closed_partial_ywd_end_date() {
    // Partial end: 2024-12-22/12-23
    assert_eq!(
        interval(&mut "2024-W51-7/2024-W52-1".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(51),
                        day: Some(7)
                    }),
                    time: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(52),
                        day: Some(1)
                    }),
                    time: None,
                },
            },
        }
    );
    assert_eq!(
        interval(&mut "2024-W51-7/W52-1".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(51),
                        day: Some(7)
                    }),
                    time: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(52),
                        day: Some(1)
                    }),
                    time: None,
                },
            },
        }
    );
    assert_eq!(
        interval(&mut "2024-W51-7/52-1".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(51),
                        day: Some(7)
                    }),
                    time: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(52),
                        day: Some(1)
                    }),
                    time: None,
                },
            },
        }
    );
    assert_eq!(
        interval(&mut "2024-W51-1/2".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(51),
                        day: Some(1)
                    }),
                    time: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(51),
                        day: Some(2)
                    }),
                    time: None,
                },
            },
        }
    );
    //TODO: this should fail, can't go backward
    assert_eq!(
        interval(&mut "2024-W51-7/1".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Closed {
                start: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(51),
                        day: Some(7)
                    }),
                    time: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YWD {
                        year: Some(2024),
                        week: Some(51),
                        day: Some(1)
                    }),
                    time: None,
                },
            },
        }
    );

    // %Y-W%W/%W
}

#[test]
fn test_partial_end_date_ywd() {
    assert_eq!(
        partial_end_date(
            &mut "1".as_bstr(),
            &PartialDate::YWD {
                year: Some(2024),
                week: Some(51),
                day: Some(7)
            }
        )
        .unwrap(),
        PartialDate::YWD {
            year: Some(2024),
            //TODO: should be 52
            week: Some(51),
            day: Some(1)
        }
    );
}

#[test]
fn interval_open() {
    assert_eq!(
        interval(&mut "P1Y2M".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::Open {
                duration: Duration {
                    years: 1,
                    months: 2,
                    weeks: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                    milliseconds: None,
                },
            },
        }
    )
}

#[test]
fn interval_closed_start() {
    assert_eq!(
        interval(&mut "2015-06-25/P1M".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::ClosedStart {
                start: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2015),
                        month: Some(6),
                        day: Some(25),
                    }),
                    time: None,
                },
                duration: Duration {
                    years: 0,
                    months: 1,
                    weeks: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                    milliseconds: None,
                },
            },
        }
    )
}

#[test]
fn interval_closed_end() {
    assert_eq!(
        interval(&mut "P1M/2015-06-25".as_bstr()).unwrap(),
        Interval {
            repetitions: None,
            range: IntervalRange::ClosedEnd {
                duration: Duration {
                    years: 0,
                    months: 1,
                    weeks: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                    milliseconds: None,
                },
                end: PartialDateTime {
                    date: Some(PartialDate::YMD {
                        year: Some(2015),
                        month: Some(6),
                        day: Some(25),
                    }),
                    time: None,
                },
            },
        }
    )
}

#[test]
fn test_day_of_week() {
    assert_eq!(day_of_week(&mut "1".as_bstr()).unwrap(), 1);
    assert_eq!(day_of_week(&mut "7".as_bstr()).unwrap(), 7);
    assert!(day_of_week(&mut "8".as_bstr()).is_err()); // Invalid day
}

// #[test]
// fn corner_cases() {
//    // how to deal with left overs?
//    assert!(parse_datetime((b"2015-06-26T22:57:09Z00:00").is_done());
//    assert!(date("2015-06-26T22:57:09Z00:00").is_err());
//
//    assert!(parse_datetime((b"2015-06-26T22:57:09Z+00:00").is_done());
//    assert!(datetime("2015-06-26T22:57:09Z+00:00").is_err());
//    assert!(parse_datetime((b"2001-W05-6T04:05:06.123455Z").is_err());
//    assert!(parse_datetime((b"2015-06-26TZ").is_err());
// }

#[test]
#[ignore]
/// a few things we don't parse correctly yet
/// see <https://ijmacd.github.io/rfc3339-iso8601/>
fn iso8601_vs_rfc3339() {
    // "+002023-02-18".parse::<Date>().unwrap();  // six digit years
    // "+002023-02".parse::<Date>().unwrap(); // six digit years
    // "+002023".parse::<Date>().unwrap(); // six digit years
    // "+2023".parse::<Date>().unwrap(); // missing months etc
    // "2023-02-18 18:29:24+01:00".parse::<DateTime>().unwrap();
    // "2023-02-18_17:29:49.278Z".parse::<DateTime>().unwrap();
    // "2021-208T22:20:32.332320+08".parse::<DateTime>().unwrap();
}
