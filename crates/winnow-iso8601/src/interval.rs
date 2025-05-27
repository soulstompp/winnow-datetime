use crate::duration::duration;
use crate::partial_datetime::{partial_datetime, partial_end_datetime};
use winnow::combinator::opt;
use winnow::combinator::trace;
use winnow::combinator::{alt, eof, terminated};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
use winnow_datetime::parser::take_digits;
use winnow_datetime::types::{Interval, IntervalRange};

/// Parses an interval
///
/// A string that optionally starts with `R` and contains a combination of partial date-times in the
/// following permissible formats:
///
pub fn parse_interval(mut i: &str) -> Result<Interval, InputError<&str>> {
    terminated(interval, eof).parse_next(&mut i)
}

/// Parses a interval string containing combinations of partial date-times and duration.
pub fn interval<'i, Input, Error>(input: &mut Input) -> Result<Interval, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("interval", move |input: &mut Input| {
        seq!(Interval {
            repetitions: opt(interval_repetitions),
            range: alt((
                interval_closed,
                interval_closed_end,
                interval_closed_start,
                interval_open
            )),
        })
        .parse_next(input)
    })
    .parse_next(input)
}

pub fn interval_repetitions<'i, Input, Error>(input: &mut Input) -> Result<Option<u32>, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("interval_repetitions", move |input: &mut Input| {
        seq!((literal("R"), opt(take_digits), literal("/")))
            .map(|(_, r, _)| r)
            .parse_next(input)
    })
    .parse_next(input)
}

fn interval_open<'i, Input, Error>(input: &mut Input) -> Result<IntervalRange, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("interval_open", move |input: &mut Input| {
        duration(input).map(|duration| IntervalRange::Open { duration })
    })
    .parse_next(input)
}

fn interval_closed<'i, Input, Error>(input: &mut Input) -> Result<IntervalRange, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("interval_closed", move |input: &mut Input| {
        let start = partial_datetime(input)?;
        let _ = literal("/").parse_next(input)?;
        let end = partial_end_datetime(input, &start)?;

        Ok(IntervalRange::Closed { start, end })
    })
    .parse_next(input)
}

fn interval_closed_end<'i, Input, Error>(input: &mut Input) -> Result<IntervalRange, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("interval_closed_end", move |input: &mut Input| {
        seq!(IntervalRange::ClosedEnd {
            duration: duration,
            _: literal("/"),
            end: partial_datetime,
        })
        .parse_next(input)
    })
    .parse_next(input)
}

fn interval_closed_start<'i, Input, Error>(input: &mut Input) -> Result<IntervalRange, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("interval_closed_start", move |input: &mut Input| {
        seq!( IntervalRange::ClosedStart {
            start: partial_datetime,
            _: literal("/"),
            duration: duration,
        })
        .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod parsers {
    use crate::interval::interval;
    use crate::partial_date::partial_end_date;
    use winnow::error::InputError;

    use winnow_datetime::types::{IntervalRange, PartialDate, PartialDateTime, PartialTime};
    use winnow_datetime::{Duration, Interval, Offset};

    #[test]
    fn interval_closed() {
        assert_eq!(
            interval::<_, InputError<_>>(&mut "2015-06-25/2015-06-26").unwrap(),
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
            interval::<_, InputError<_>>(&mut "2015-06-25 12:00:00Z/2015-06-26 12:00:00Z").unwrap(),
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
                            offset: Some(Offset::Fixed {
                                hours: 0,
                                minutes: 0,
                                critical: false,
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
                            offset: Some(Offset::Fixed {
                                hours: 0,
                                minutes: 0,
                                critical: false,
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
            interval::<_, InputError<_>>(&mut "2024-12-22/12-23").unwrap(),
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
            interval::<_, InputError<_>>(&mut "2024-12-22/23").unwrap(),
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
            interval::<_, InputError<_>>(&mut "2024-W51-7/2024-W52-1").unwrap(),
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
            interval::<_, InputError<_>>(&mut "2024-W51-7/W52-1").unwrap(),
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
            interval::<_, InputError<_>>(&mut "2024-W51-7/52-1").unwrap(),
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
            interval::<_, InputError<_>>(&mut "2024-W51-1/2").unwrap(),
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
        assert_eq!(
            interval::<_, InputError<_>>(&mut "2024-W51-7/1").unwrap(),
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
            partial_end_date::<_, InputError<_>>(
                &mut "1",
                &PartialDate::YWD {
                    year: Some(2024),
                    week: Some(51),
                    day: Some(7)
                }
            )
            .unwrap(),
            PartialDate::YWD {
                year: Some(2024),
                week: Some(51),
                day: Some(1)
            }
        );
    }

    #[test]
    fn interval_open() {
        assert_eq!(
            interval::<_, InputError<_>>(&mut "P1Y2M").unwrap(),
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
            interval::<_, InputError<_>>(&mut "2015-06-25/P1M").unwrap(),
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
            interval::<_, InputError<_>>(&mut "P1M/2015-06-25").unwrap(),
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
}
