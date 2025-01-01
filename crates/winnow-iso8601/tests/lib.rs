use crate::duration::Duration;
use winnow::combinator::{eof, terminated};
use winnow::Parser;
use winnow_datetime::Date;
use winnow_datetime::DateTime;
use winnow_datetime::Offset;
use winnow_datetime::Time;
use winnow_iso8601::parsers::time;
use winnow_iso8601::*;

#[test]
fn test_parse_date() {
    assert_eq!(
        Ok(Date::YMD {
            year: 2015,
            month: 6,
            day: 26,
        }),
        parse_date("2015-06-26")
    );
    assert_eq!(
        Ok(Date::YMD {
            year: -333,
            month: 7,
            day: 11,
        }),
        parse_date("-0333-07-11")
    );
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
                offset: Default::default(),
            }),
            parse_time(format!("16:43:00.{:0>3}", i).as_str())
        );
        i += 1;
    }
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 100,
            offset: Default::default(),
        }),
        parse_time("16:43:00.1")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 120,
            offset: Default::default(),
        }),
        parse_time("16:43:00.12")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 123,
            offset: Default::default(),
        }),
        parse_time("16:43:00.123")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 432,
            offset: Default::default(),
        }),
        parse_time("16:43:00.4321")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 432,
            offset: Default::default(),
        }),
        parse_time("16:43.4321")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 11,
            millisecond: 432,
            offset: Default::default(),
        }),
        parse_time("16:43:11.4321")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 100,
            offset: Default::default(),
        }),
        parse_time("16:43:00,1")
    );

    assert_eq!(
        Ok(Time {
            hour: 4,
            minute: 5,
            second: 6,
            millisecond: 123,
            offset: Default::default(),
        }),
        parse_time("04:05:06.12345")
    );

    assert_eq!(
        Ok(DateTime {
            date: Date::Week {
                year: 2001,
                ww: 5,
                d: 6
            },
            time: Time {
                hour: 4,
                minute: 5,
                second: 6,
                millisecond: 123,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        }),
        parse_datetime("2001-W05-6T04:05:06.12345Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 123,
            offset: Default::default(),
        }),
        parse_time("16:43:16.123")
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
    assert_eq!(
        parse_time("16:43:16"),
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            offset: Default::default(),
        })
    );
    assert_eq!(
        parse_time("16:43"),
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 0,
            offset: Default::default(),
        })
    );

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
fn short_time1() {
    assert_eq!(
        parse_time("1648"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            offset: Default::default(),
        })
    );
}
#[test]
fn short_time2() {
    assert_eq!(
        parse_time("16:48"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            offset: Default::default(),
        })
    );
}
#[test]
fn short_time3() {
    assert_eq!(
        parse_time("16:48Z"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        })
    );
}
#[test]
fn short_time4() {
    assert_eq!(
        parse_time("164800"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            offset: Default::default(),
        })
    );
}
#[test]
fn short_time5() {
    assert_eq!(
        parse_time("164800.1"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 100,
            offset: Default::default(),
        })
    );
}
#[test]
fn short_time6() {
    assert_eq!(
        parse_time("164800.1Z"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 100,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        })
    );
}
#[test]
fn short_time7() {
    assert_eq!(
        parse_time("16:48:00"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            offset: Default::default(),
        })
    );
}

#[test]
fn short_twtz1() {
    assert_eq!(
        parse_time("1648Z"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        })
    );
}
#[test]
fn short_twtz2() {
    assert_eq!(
        parse_time("16:48Z"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        })
    );
}

#[test]
fn short_dtim1() {
    assert_eq!(
        parse_datetime("20070831T1648"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 8,
                day: 31,
            },
            time: Time {
                hour: 16,
                minute: 48,
                second: 0,
                millisecond: 0,
                offset: Default::default(),
            }
        })
    );
}
#[test]
fn short_dtim2() {
    assert_eq!(
        parse_datetime("20070831T1648Z"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 8,
                day: 31,
            },
            time: Time {
                hour: 16,
                minute: 48,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0,
                }),
            },
        })
    );
}
#[test]
fn short_dtim3() {
    assert_eq!(
        parse_datetime("2008-12-24T18:21Z"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2008,
                month: 12,
                day: 24,
            },
            time: Time {
                hour: 18,
                minute: 21,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0,
                }),
            },
        })
    );
}

#[test]
fn test_time_with_offset() {
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            offset: Default::default(),
        }),
        parse_time("16:43:16")
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
fn test_iso_week_date() {
    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 5,
            d: 7,
        }),
        parse_date("2015-W05-7")
    );

    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 6,
            d: 6,
        }),
        parse_date("2015-W06-6")
    );

    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 6,
            d: 6,
        }),
        parse_date("2015-W066")
    );

    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 6,
            d: 6,
        }),
        parse_date("2015W066")
    );

    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 43,
            d: 6,
        }),
        parse_date("2015-W43-6")
    );

    assert!(parse_date("2015-W06-8").is_err());
    assert!(parse_date("2015-W068").is_err());
    assert!(parse_date("2015-W06-0").is_err());
    assert!(parse_date("2015-W00-2").is_err());
    assert!(parse_date("2015-W54-2").is_err());
    assert!(parse_date("2015-W542").is_err());
}

#[test]
fn test_ordinal_parse_date() {
    assert_eq!(
        Ok(Date::Ordinal {
            year: 2015,
            ddd: 057,
        }),
        parse_date("2015-057")
    );

    assert_eq!(
        Ok(Date::Ordinal {
            year: 2015,
            ddd: 358,
        }),
        parse_date("2015-358")
    );
    assert_eq!(
        Ok(Date::Ordinal {
            year: 2015,
            ddd: 366,
        }),
        parse_date("2015-366")
    );

    assert_eq!(
        Ok(Date::Ordinal { year: 2015, ddd: 1 }),
        parse_date("2015-001")
    );

    // not valid here either
    assert!(parse_date("2015-400").is_err());
}

#[test]
fn format_equivalence() {
    assert_eq!(
        parse_datetime("2001-02-03T04:05:06+07:00"),
        parse_datetime("20010203T040506+0700")
    );
    assert_eq!(
        parse_datetime("2001-02-03T04:05:06+07:00"),
        parse_datetime("20010203T04:05:06+0700")
    );
    assert_eq!(
        parse_datetime("2001-02-03T04:05:00+07:00"),
        parse_datetime("20010203T0405+0700")
    );
    assert_eq!(
        parse_datetime("20010203T0405+0700"),
        parse_datetime("2001-02-03T0405+0700")
    );
    assert_eq!(
        parse_datetime("20010203T040506+0700"),
        parse_datetime("2001-02-03T040506+0700")
    );
    assert_eq!(
        parse_datetime("20010203T040506+0000"),
        parse_datetime("20010203T040506Z")
    );
    assert_eq!(
        parse_datetime("2015W056T04:05:06+07:00"),
        parse_datetime("2015-W05-6T04:05:06+07:00")
    );
}

#[test]
fn test_datetime_correct() {
    assert_eq!(
        parse_datetime("20060831T16:44+00:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2006,
                month: 8,
                day: 31
            },
            time: Time {
                hour: 16,
                minute: 44,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2007-08-31T16:45+00:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 8,
                day: 31
            },
            time: Time {
                hour: 16,
                minute: 45,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("20070831T1646+00:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 8,
                day: 31
            },
            time: Time {
                hour: 16,
                minute: 46,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                }),
            }
        })
    );

    assert_eq!(
        parse_datetime("20070831T1647+0000"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 8,
                day: 31
            },
            time: Time {
                hour: 16,
                minute: 47,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                }),
            }
        })
    );

    assert_eq!(
        parse_datetime("2009-02-01T09:00:22+05"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2009,
                month: 2,
                day: 1
            },
            time: Time {
                hour: 9,
                minute: 0,
                second: 22,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 5,
                    offset_minutes: 0
                }),
            }
        })
    );

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
        parse_datetime("2015-06-29T23:07+02:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2015,
                month: 6,
                day: 29
            },
            time: Time {
                hour: 23,
                minute: 7,
                second: 0,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 2,
                    offset_minutes: 0
                }),
            }
        })
    );

    assert_eq!(
        parse_datetime("2015-06-26T16:43:16"),
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
                offset: Default::default(),
            }
        })
    );

    assert_eq!(
        parse_datetime("2015-06-26T16:43:16"),
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
                offset: Default::default(),
            }
        })
    );

    assert_eq!(
        parse_datetime("2015-W05-6T04:05:06+07:00"),
        Ok(DateTime {
            date: Date::Week {
                year: 2015,
                ww: 5,
                d: 6
            },
            time: Time {
                hour: 4,
                minute: 5,
                second: 6,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 7,
                    offset_minutes: 0
                })
            }
        })
    );
    assert_eq!(
        parse_datetime("2015W056T04:05:06+07:00"),
        Ok(DateTime {
            date: Date::Week {
                year: 2015,
                ww: 5,
                d: 6
            },
            time: Time {
                hour: 4,
                minute: 5,
                second: 6,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 7,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2015-056T04:05:06+07:00"),
        Ok(DateTime {
            date: Date::Ordinal {
                year: 2015,
                ddd: 56
            },
            time: Time {
                hour: 4,
                minute: 5,
                second: 6,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 7,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2015056T04:05:06+07:00"),
        Ok(DateTime {
            date: Date::Ordinal {
                year: 2015,
                ddd: 56
            },
            time: Time {
                hour: 4,
                minute: 5,
                second: 6,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 7,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2015-297T16:30:48Z"),
        Ok(DateTime {
            date: Date::Ordinal {
                year: 2015,
                ddd: 297
            },
            time: Time {
                hour: 16,
                minute: 30,
                second: 48,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2015-W43-6T16:30:48Z"),
        Ok(DateTime {
            date: Date::Week {
                year: 2015,
                ww: 43,
                d: 6
            },
            time: Time {
                hour: 16,
                minute: 30,
                second: 48,
                millisecond: 0,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2001-W05-6T04:05:06.1234Z"),
        Ok(DateTime {
            date: Date::Week {
                year: 2001,
                ww: 5,
                d: 6
            },
            time: Time {
                hour: 4,
                minute: 5,
                second: 6,
                millisecond: 123,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );

    assert_eq!(
        parse_datetime("2001-W05-6T04:05:06.12345Z"),
        Ok(DateTime {
            date: Date::Week {
                year: 2001,
                ww: 5,
                d: 6
            },
            time: Time {
                hour: 4,
                minute: 5,
                second: 6,
                millisecond: 123,
                offset: Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0
                })
            }
        })
    );
}

#[test]
fn issue12_regression_1() {
    let input = "164801.";

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 1,
            millisecond: 0,
            offset: Default::default(),
        }),
        parse_time(input)
    );
}

#[test]
fn issue12_regression_2() {
    let input = "04:05:06.1226001015632Z)*450";

    assert_eq!(
        Ok(Time {
            hour: 4,
            minute: 5,
            second: 6,
            millisecond: 122,
            offset: Some(Offset {
                offset_hours: 0,
                offset_minutes: 0,
            }),
        }),
        parse_time(input)
    );
}

#[test]
fn test_duration_ymdhms() {
    use core::time::Duration as StdDuration;

    // full YMDHMS
    let dur = parse_duration("P1Y2M3DT4H5M6S").unwrap();
    assert_eq!(
        Duration {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 5,
            seconds: 6,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(36993906, 0));

    // full YMDHMS with milliseconds dot delimiter
    let dur = parse_duration("P1Y2M3DT4H5M6.7S").unwrap();
    assert_eq!(
        Duration {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 5,
            seconds: 6,
            milliseconds: 700,
        },
        dur
    );
    assert_eq!(
        StdDuration::from(dur),
        StdDuration::new(36993906, 700000000)
    );

    // full YMDHMS with milliseconds comma delimiter
    let dur = parse_duration("P1Y2M3DT4H5M6,7S").unwrap();
    assert_eq!(
        Duration {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 5,
            seconds: 6,
            milliseconds: 700,
        },
        dur
    );
    assert_eq!(
        StdDuration::from(dur),
        StdDuration::new(36993906, 700000000)
    );

    // subset YM-HM-
    let dur = parse_duration("P1Y2MT4H5M").unwrap();
    assert_eq!(
        Duration {
            years: 1,
            months: 2,
            weeks: 0,
            days: 0,
            hours: 4,
            minutes: 5,
            seconds: 0,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(36734700, 0));

    // subset Y-----
    let dur = parse_duration("P1Y").unwrap();
    assert_eq!(
        Duration {
            years: 1,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(31536000, 0));

    // subset ---H--
    let dur = parse_duration("PT4H").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            days: 0,
            weeks: 0,
            hours: 4,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(14400, 0));

    // subset -----S with milliseconds dot delimiter
    let dur = parse_duration("PT6.7S").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 6,
            milliseconds: 700,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(6, 700000000));

    // subset -----S with milliseconds comma delimiter
    let dur = parse_duration("PT6,700S").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 6,
            milliseconds: 700,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(6, 700000000));

    // empty duration, using Y
    let dur = parse_duration("P0Y").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(0, 0));

    // empty duration, using S
    let dur = parse_duration("PT0S").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(0, 0));

    let dur = parse_duration("PT42M30S").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 42,
            seconds: 30,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(2550, 0));

    let dur = parse_duration("P0W").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(0, 0));

    let dur = parse_duration("P26W").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            weeks: 26,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(15724800, 0));

    let dur = parse_duration("P52W").unwrap();
    assert_eq!(
        Duration {
            years: 0,
            months: 0,
            weeks: 52,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        },
        dur
    );
    assert_eq!(StdDuration::from(dur), StdDuration::new(31449600, 0));
}

#[test]
fn test_z_times() {
    assert!(terminated(time, eof).parse_next(&mut "16:43:16z").is_err());

    assert!(terminated(time, eof).parse_next(&mut "07:42:55z").is_err());
}
