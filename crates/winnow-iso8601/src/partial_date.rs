use crate::date::{date_day_of_year, date_year, day_of_week, week_of_year};
use core::str;
use winnow::combinator::{alt, empty, fail, opt, preceded, trace};
use winnow::error::ParserError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
use winnow_datetime::parser::{date_day, date_month};
use winnow_datetime::types::PartialDate;
use winnow_datetime::{date_yddd_seq, date_ymd_seq, date_ywd_seq};

/// Parses 2 digit week of the year within range 01-52
/// Parses a date string as ISO 8601 week date.
// YYYY-"W"WW-D
fn partial_date_ywd<'i, Input, Error>(input: &mut Input) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_date_ywd", move |input: &mut Input| {
        seq!(
            date_year.map(Some), // y
            opt(preceded(
                preceded(opt(literal("-")), literal("W")),
                week_of_year
            )), // w
            opt(preceded(opt(literal("-")), day_of_week))
        )
        .verify(|(_, w, d)| (w.is_some() || d.is_some()) && !(w.is_none() && d.is_some()))
        .map(|(year, week, day)| PartialDate::YWD { year, week, day })
        .parse_next(input)
    })
    .parse_next(input)
}

// YYYY-MM-DD
fn partial_date_ymd<'i, Input, Error>(input: &mut Input) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_date_ymd", move |input: &mut Input| {
        seq!((
            date_year,                                    // YYYY
            opt(preceded(opt(literal("-")), date_month)), //DD
            opt(preceded(opt(literal("-")), date_day)),   //DD
        ))
        .verify(|(_, m, d)| m.is_some() || d.is_some())
        .map(|(year, month, day)| PartialDate::YMD {
            year: Some(year),
            month,
            day,
        })
        .parse_next(input)
    })
    .parse_next(input)
}

// YYYY-DDD
fn partial_date_yddd<'i, Input, Error>(input: &mut Input) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_date_yddd", move |input: &mut Input| {
        seq!((
            opt(date_year),
            preceded(opt(literal("-")), opt(date_day_of_year))
        ))
        .verify(|(_, day)| day.is_some())
        .map(|(year, day)| PartialDate::YDDD { year, day })
        .parse_next(input)
    })
    .parse_next(input)
}

/// Parses a date string specificed as YYYYMMDD
fn partial_date_ymd_numeric<'i, Input, Error>(input: &mut Input) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_date_ymd_numeric", move |input: &mut Input| {
        seq!((
            opt(date_year),  // YYYY
            opt(date_month), // MM
            opt(date_day),   //DD
        ))
        .verify(|(_, m, d)| m.is_some() || d.is_some())
        .map(|(y, m, d)| PartialDate::YMD {
            year: y,
            month: m,
            day: d,
        })
        .parse_next(input)
    })
    .parse_next(input)
}

/// Parses a date string specificed as YYYYMMDD
fn partial_date_y<'i, Input, Error>(input: &mut Input) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_date_year_only", move |input: &mut Input| {
        date_year(input).map(|d| PartialDate::Year {
            year: Some(d), // YYYY
        })
    })
    .parse_next(input)
}

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub(crate) fn partial_date<'i, Input, Error>(input: &mut Input) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_date", move |input: &mut Input| {
        alt((
            partial_date_ywd,
            partial_date_yddd,
            partial_date_ymd_numeric,
            partial_date_ymd,
            partial_date_y,
        ))
        .parse_next(input)
    })
    .parse_next(input)
}

/// Parses a possibly trunctated partial datetime string based on a partial start date
pub(crate) fn partial_end_date<'i, Input, Error>(
    input: &mut Input,
    start_date: &PartialDate,
) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_end_date", move |input: &mut Input| {
        match start_date {
            PartialDate::Year { year: start_year } => {
                // Parse the year or use fallback only when parsing fails
                let year = match date_year::<Input, Error>.parse_next(input) {
                    Ok(parsed) => Some(parsed),
                    Err(e) => {
                        if !e.is_incomplete() {
                            *start_year
                        } else {
                            return Err(e);
                        }
                    }
                };

                Ok(PartialDate::Year { year })
            }
            PartialDate::YWD {
                year: _,
                week: _,
                day: _,
            } => partial_end_date_ywd(input, start_date),
            PartialDate::YDDD { year: _, day: _ } => partial_end_date_yddd(input, start_date),
            PartialDate::YMD {
                year: _,
                month: _,
                day: _,
            } => partial_end_date_ymd(input, start_date),
        }
    })
    .parse_next(input)
}

/// Sifts through portions of end_date parses for a Date::YDDD start_date
pub(crate) fn partial_end_date_yddd<'i, Input, Error>(
    input: &mut Input,
    start_date: &PartialDate,
) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_end_date_ydd", move |input: &mut Input| {
        match start_date {
            PartialDate::YDDD {
                year: start_year,
                day: start_day,
            } => {
                match [start_year.is_some(), start_day.is_some()] {
                    // Case 1: Full Year-Month-Day context available
                    [true, true] => alt((
                        date_yddd_seq!(PartialDate::YDDD {
                            year: date_year.map(Some),
                            day: preceded(literal("-"), date_day_of_year).map(Some),
                        }),
                        date_yddd_seq!(PartialDate::YDDD {
                            year: opt(empty).map(|_| *start_year),
                            day: date_day_of_year.map(Some),
                        }),
                    ))
                    .parse_next(input),
                    // Case 2: Partial Year-Month (no day)
                    [true, false] => date_yddd_seq!(PartialDate::YDDD {
                        year: opt(empty).map(|_| *start_year),
                        day: date_day_of_year.map(Some),
                    })
                    .parse_next(input),
                    _ => fail.parse_next(input),
                }
            }
            _ => fail.parse_next(input),
        }
    })
    .parse_next(input)
}

/// Sifts through portions of end_date based on a Date::YMD start_date
fn partial_end_date_ymd<'i, Input, Error>(
    input: &mut Input,
    start_date: &PartialDate,
) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_end_date_ymd", move |input: &mut Input| {
        match start_date {
            PartialDate::YMD {
                year: start_year,
                month: start_month,
                day: start_day,
            } => {
                match [
                    start_year.is_some(),
                    start_month.is_some(),
                    start_day.is_some(),
                ] {
                    // Case 1: Full Year-Month-Day context available
                    [true, true, true] => alt((
                        // Fully specified end date
                        date_ymd_seq!(PartialDate::YMD {
                            year: date_year.map(Some),
                            month: preceded(literal("-"), date_month).map(Some),
                            day: preceded(literal("-"), date_day).map(Some),
                        }),
                        // Default year and parse month-day
                        date_ymd_seq!(PartialDate::YMD {
                            year: opt(empty).map(|_| *start_year),
                            month: date_month.map(Some),
                            day: preceded(literal("-"), date_day).map(Some),
                        }),
                        // Default year and month, parse only day
                        date_ymd_seq!(PartialDate::YMD {
                            year: opt(empty).map(|_| *start_year),
                            month: opt(empty).map(|_| *start_month),
                            day: date_day.map(Some),
                        }),
                    ))
                    .parse_next(input),

                    // Case 2: Partial Year-Month (no day)
                    [true, true, false] => alt((
                        // Fully specified year-month
                        date_ymd_seq!(PartialDate::YMD {
                            year: date_year.map(Some),
                            month: preceded(literal("-"), date_month).map(Some),
                            day: opt(empty).map(|_| None),
                        }),
                        // Default year, parse month
                        date_ymd_seq!(PartialDate::YMD {
                            year: opt(empty).map(|_| *start_year),
                            month: date_month.map(Some),
                            day: opt(empty).map(|_| None),
                        }),
                    ))
                    .parse_next(input),

                    // Case 3: Year only (no month or day)
                    [true, false, false] => date_ymd_seq!(PartialDate::YMD {
                        year: date_year.map(Some),
                        month: opt(empty).map(|_| None),
                        day: opt(empty).map(|_| None),
                    })
                    .parse_next(input),

                    // Case 4: Invalid (no year provided)
                    [_, _, _] => fail.parse_next(input),
                }
            }
            _ => return Err(ParserError::from_input(input)),
        }
    })
    .parse_next(input)
}

/// Sifts through portions of end_date based on a Date::YMD start_date
pub(crate) fn partial_end_date_ywd<'i, Input, Error>(
    i: &mut Input,
    start_date: &PartialDate,
) -> Result<PartialDate, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("partial_end_date_ywd", move |input: &mut Input| {
        match start_date {
            PartialDate::YWD {
                year: start_year,
                week: start_week,
                day: start_day,
            } => {
                match [
                    start_year.is_some(),
                    start_week.is_some(),
                    start_day.is_some(),
                ] {
                    // Case 1: Full start context (YWD)
                    [true, true, true] => alt((
                        date_ywd_seq!(PartialDate::YWD {
                            year: date_year.map(Some),
                            week: preceded(literal("-W"), week_of_year).map(Some),
                            day: preceded(literal("-"), day_of_week).map(Some),
                        }),
                        date_ywd_seq!(PartialDate::YWD {
                            year: opt(empty).map(|_| *start_year),
                            week: preceded(opt(literal("W")), week_of_year).map(Some),
                            day: preceded(literal("-"), day_of_week).map(Some),
                        }),
                        date_ywd_seq!(PartialDate::YWD {
                            year: opt(empty).map(|_| *start_year),
                            week: opt(empty).map(|_| *start_week),
                            day: day_of_week.map(Some),
                        }),
                    ))
                    .parse_next(input),

                    // Case 2: Partial start (YW)
                    [true, true, false] => {
                        alt((
                            date_ywd_seq!(PartialDate::YWD {
                                year: date_year.map(Some),
                                week: preceded(literal("-W"), week_of_year).map(Some),
                                day: opt(empty).map(|_| None),
                            }),
                            date_ywd_seq!(PartialDate::YWD {
                                year: opt(empty).map(|_| *start_year),
                                week: preceded(opt(literal("W")), week_of_year).map(Some),
                                day: opt(empty).map(|_| None),
                            }),
                        ))
                    }
                    .parse_next(input),

                    // Case 3: Year only
                    [true, false, false] => {
                        date_ywd_seq!(PartialDate::YWD {
                            year: date_year.map(Some),
                            week: opt(empty).map(|_| None),
                            day: opt(empty).map(|_| None),
                        })
                    }
                    .parse_next(input),

                    // Case 4: Invalid
                    [_, _, _] => fail.parse_next(input),
                }
            }
            _ => fail.parse_next(input),
        }
    })
    .parse_next(i)
}

#[cfg(test)]
mod parsers {
    use crate::partial_date::{partial_date, partial_end_date};
    use winnow::error::InputError;

    use winnow_datetime::types::PartialDate;

    #[test]
    fn partial_date_parsing() {
        // Year
        assert_eq!(
            partial_date::<_, InputError<_>>(&mut "2015").unwrap(),
            PartialDate::Year { year: Some(2015) }
        );
        // YMD
        assert_eq!(
            partial_date::<_, InputError<_>>(&mut "2015-06-26").unwrap(),
            PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: Some(26)
            }
        );
        assert_eq!(
            partial_date::<_, InputError<_>>(&mut "2015-06").unwrap(),
            PartialDate::YMD {
                year: Some(2015),
                month: Some(6),
                day: None
            }
        );
        // YWD
        assert_eq!(
            partial_date::<_, InputError<_>>(&mut "2015-W05-6").unwrap(),
            PartialDate::YWD {
                year: Some(2015),
                week: Some(5),
                day: Some(6)
            }
        );
        assert_eq!(
            partial_date::<_, InputError<_>>(&mut "2015-W05").unwrap(),
            PartialDate::YWD {
                year: Some(2015),
                week: Some(5),
                day: None
            }
        );
        //Ordinal
        assert_eq!(
            partial_date::<_, InputError<_>>(&mut "2015-156").unwrap(),
            PartialDate::YDDD {
                year: Some(2015),
                day: Some(156)
            }
        );
        assert_eq!(
            partial_date::<_, InputError<_>>(&mut "2015-156").unwrap(),
            PartialDate::YDDD {
                year: Some(2015),
                day: Some(156)
            }
        );
    }

    #[test]
    fn partial_ymd() {
        assert_eq!(
            partial_end_date::<_, InputError<_>>(
                &mut "2015-06-26",
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
            partial_end_date::<_, InputError<_>>(
                &mut "06-26",
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
            partial_end_date::<_, InputError<_>>(
                &mut "26",
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
    }

    #[test]
    fn partial_ywd() {
        assert_eq!(
            partial_end_date::<_, InputError<_>>(
                &mut "2024-W51-4",
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
            partial_end_date::<_, InputError<_>>(
                &mut "W51-4",
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
            partial_end_date::<_, InputError<_>>(
                &mut "4",
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
    }

    #[test]
    fn partial_yddd() {
        assert_eq!(
            partial_end_date::<_, InputError<_>>(
                &mut "2025-083",
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
    }
}
