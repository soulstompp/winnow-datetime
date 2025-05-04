use winnow_datetime::Date;
use winnow_datetime::DateTime;
use winnow_datetime::Offset;
use winnow_datetime::Time;
use winnow_rfc3339::{parse_date, parse_datetime, parse_time};

#[test]
fn test_date() {
    assert_eq!(
        Ok(Date::YMD {
            year: 2015,
            month: 6,
            day: 26,
        }),
        parse_date("2015-06-26")
    );
    assert!(parse_date("-0333-07-11").is_err());
}

#[test]
fn test_millisecond() {
    let mut i = 0;
    while i < 1000 {
        //regression test for pull request 36.
        assert_eq!(
            Ok(Time {
                hour: 16,
                minute: 43,
                second: 0,
                millisecond: i,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0,
                }),
            }),
            parse_time(format!("16:43:00.{:0>3}Z", i).as_str())
        );
        i += 1;
    }

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 100,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:00.1Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 120,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:00.12Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 123,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:00.123Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 432,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:00.4321Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 11,
            millisecond: 432,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:11.4321Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 100,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:00,1Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 4,
            minute: 5,
            second: 6,
            millisecond: 123,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("04:05:06.12345Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 123,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:16.123Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 123,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:16.123+00:00")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 123,
            offset: Some(Offset {
                offset_hours: 5,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:16.123+05:00")
    );
}

#[test]
fn test_time() {
    assert!(parse_time("16:43:16").is_err());

    assert!(parse_time("pppp").is_err());
}

#[test]
fn test_time_set_tz() {
    let original = Time {
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
        offset: Default::default(),
    };
    let expected = Time {
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
        offset: Some(Offset {
            offset_hours: 2,
            offset_minutes: 30,
        }),
    };

    assert_eq!(expected, original.set_tz(Some((2, 30))));
}

#[test]
fn short_time7() {
    assert!(parse_time("16:48:00").is_err());
}

#[test]
fn test_time_with_offset() {
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:16Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time("16:43:16+00:00")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            offset: Some(Offset {
                offset_hours: 5,
                offset_minutes: 0,
            })
        }),
        parse_time("16:43:16+05:00")
    );

    assert!(parse_time("pppp").is_err());
}

#[test]
fn test_datetime_correct() {
    assert_eq!(
        parse_datetime("2010-01-01T12:00:00+01:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2010,
                month: 1,
                day: 1
            },
            time: Time {
                hour: 12,
                minute: 0,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 1,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2011-06-30T18:30:00+02:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2011,
                month: 6,
                day: 30
            },
            time: Time {
                hour: 18,
                minute: 30,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 2,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2015-06-26T16:43:16Z"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2015,
                month: 6,
                day: 26
            },
            time: Time {
                hour: 16,
                minute: 43,
                second: 16,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2015-06-26T16:43:16Z"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2015,
                month: 6,
                day: 26
            },
            time: Time {
                hour: 16,
                minute: 43,
                second: 16,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );
}

#[test]
fn lower_case_separators() {
    assert_eq!(
        parse_datetime("2011-06-30t18:30:00z"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2011,
                month: 6,
                day: 30
            },
            time: Time {
                hour: 18,
                minute: 30,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );
}
