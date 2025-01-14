//! The low-level parsers for date, datetime, duration and time.
//!
//! The top-level functions [`date()`][`crate::date()`], [`datetime()`][`crate::datetime()`],
//! [`duration()`][`crate::duration()`] and [`time()`][`crate::time()`]
//! provide convenient wrappers around the low-level parsers,
//! but throw away leftover input on success.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use core::str;
use winnow::combinator::{alt, trace};
use winnow::combinator::{empty, fail, opt};
use winnow::combinator::{preceded, separated_pair, terminated};
use winnow::error::{ContextError, ErrMode};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::one_of;
use winnow::token::{literal, take_while};
use winnow::{seq, PResult, Parser};
use winnow_datetime::parsers::date_day;
use winnow_datetime::parsers::date_month;
use winnow_datetime::parsers::sign;
use winnow_datetime::parsers::take_digits;
use winnow_datetime::parsers::take_digits_in_range;
use winnow_datetime::parsers::{fraction_millisecond, time_hour, time_minute, time_second};
use winnow_datetime::types::{
    Duration, DurationPart, Interval, IntervalRange, PartialDate, PartialDateTime, PartialTime,
};
use winnow_datetime::{date_yddd_seq, date_ymd_seq, date_ywd_seq, duration_part_seq, time_seq, Date, DateTime, FractionalDuration, Offset, Time};

#[cfg(test)]
mod tests;

// DATE

/// Parses 2 digit week of the year within range 01-52
// WW
fn date_week<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_week", move |input: &mut Input| {
        let _ = preceded(opt(literal("-")), literal("W")).parse_next(input)?; // [-]Ww

        week_of_year(input)
    })
    .parse_next(i)
}

/// Parses 2 digit week of the year within range 01-52
// WW
fn week_of_year<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_week", move |input: &mut Input| {
        take_digits_in_range(input, 2, 1..=52)
    })
    .parse_next(i)
}

/// Parses 2 digit week of the year within range 01-7
fn date_day_of_week<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_week_day", move |input: &mut Input| {
        preceded(opt(literal("-")), day_of_week).parse_next(input)
    })
    .parse_next(i)
}

/// Parses 2 digit week of the year within range 01-7
fn day_of_week<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("day_of_week", move |input: &mut Input| {
        take_digits_in_range(input, 1, 1..=7)
    })
    .parse_next(i)
}

/// Parses 2 digit week of the year within range 01-52
/// Parses a date string as ISO 8601 week date.
// YYYY-"W"WW-D
fn date_ywd<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        seq!((
            date_year, // y
            date_week, // w
            opt(date_day_of_week)
        ))
        .map(|(year, week, day)| Date::Week {
            year,
            week,
            day: day.unwrap_or(1),
        })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses 2 digit week of the year within range 01-52
/// Parses a date string as ISO 8601 week date.
// YYYY-"W"WW-D
fn partial_date_ywd<'i, Input>(i: &mut Input) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

// [-]D - unverified
/// Parses a year with +/- and will eventually support 6 digit year
// [+/-]YYYY
fn date_year<'i, Input>(i: &mut Input) -> PResult<i32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_year", move |input: &mut Input| {
        // The sign is optional, but defaults to `+`
        let sign = opt(sign).parse_next(input)?.unwrap_or(1);

        let y = take_while(4, |c: <Input as InputStream>::Token| {
            c.as_char().is_digit(10)
        })
        .parse_next(input)?;
        let year: i32 = str::from_utf8(y.as_bstr()).unwrap().parse().unwrap();

        if year >= 100 && year < 10000 {
            Ok(sign * year)
        } else {
            Err(ErrMode::Backtrack(ContextError::new()))
        }
    })
    .parse_next(i)
}

// YYYY-MM-DD
fn date_ymd<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ymd", move |input: &mut Input| {
        seq!(Date::YMD {
            year: date_year,      // YYYY
            _: opt(literal("-")), // -
            month: date_month,     // MM
            day: opt(preceded(opt(literal("-")), date_day)).map(|d| d.unwrap_or(1)),       //DD
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// YYYY-MM-DD
fn partial_date_ymd<'i, Input>(i: &mut Input) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}
// ordinal DDD
fn date_day_of_year<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ord_day", move |input: &mut Input| {
        take_digits_in_range(input, 3, 1..=366)
    })
    .parse_next(i)
}

// YYYY-DDD
fn date_yddd<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_yddd", move |input: &mut Input| {
        separated_pair(date_year, opt(literal("-")), date_day_of_year)
            .map(|(year, day)| Date::Ordinal { year, day })
            .parse_next(input)
    })
    .parse_next(i)
}

// YYYY-DDD
fn partial_date_yddd<'i, Input>(i: &mut Input) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

/// Parses a date string specificed as YYYYMMDD
fn date_ymd_numeric<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ymd_numeric", move |input: &mut Input| {
        seq!(Date::YMD {
            year: date_year,   // YYYY
            month: date_month, // MM
            day: date_day,     //DD
        })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a date string specificed as YYYYMMDD
fn partial_date_ymd_numeric<'i, Input>(i: &mut Input) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

/// Parses a date string specificed as YYYYMMDD
pub fn partial_date_y<'i, Input>(i: &mut Input) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_date_year_only", move |input: &mut Input| {
        date_year(input).map(|d| PartialDate::Year {
            year: Some(d), // YYYY
        })
    })
    .parse_next(i)
}

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub fn date<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date", move |input: &mut Input| {
        alt((date_ywd, date_ymd_numeric, date_yddd, date_ymd)).parse_next(input)
    })
    .parse_next(i)
}

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
fn partial_date<'i, Input>(i: &mut Input) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

// TIME

/// Parses a time string with an optional preceding 'T'.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn time<'i, Input>(i: &mut Input) -> PResult<Time>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("time", move |input: &mut Input| {
        seq!((
            _: opt(literal("T")),
            base_time
        ))
        .map(|r| r.0)
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a partial time string with an optional preceding 'T'.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
fn partial_time<'i, Input>(i: &mut Input) -> PResult<PartialTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_time", move |input: &mut Input| {
        seq!((
            _: opt(alt((literal(" "), literal("T")))),
            partial_base_time
        ))
        .map(|r| r.0)
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn base_time<'i, Input>(i: &mut Input) -> PResult<Time>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("base_time", move |input: &mut Input| {
        let hour = time_hour(input)?;

        let msms = opt(time_minute_second_millisecond).parse_next(input)?; // MM:[SS][.(m*)]

        let offset = opt(offset).parse_next(input)?;

        let (minute, second, millisecond) = msms.unwrap_or((0, None, None));

        Ok(Time {
            hour,                                  // HH
            minute,                                // MM
            second: second.unwrap_or(0),           // [SS]
            millisecond: millisecond.unwrap_or(0), // [.(m*)]
            offset: offset.unwrap_or(None),        // [(Z|+...|-...)]
        })
    })
    .parse_next(i)
}

/// Parses a partial time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
fn partial_base_time<'i, Input>(i: &mut Input) -> PResult<PartialTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_partial_time", move |input: &mut Input| {
        seq!(PartialTime {
            hour: time_hour.map(Some),                        // HH
            minute: opt(preceded(literal(":"), time_minute)), // MM
            second: opt(preceded(literal(":"), time_second)), // SS
            millisecond: opt(preceded(
                alt((literal("."), literal(","))),
                fraction_millisecond
            )), // .mmm
            offset: opt(offset).map(|o| o.unwrap_or(None)),   // [(Z|+...|-...)]
        })
        .parse_next(input)
    })
    .parse_next(i)
}

/// a partial time string which can be truncated depending on a partial start time
fn partial_end_base_time<'i, Input>(i: &mut Input, start_time: &PartialTime) -> PResult<PartialTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_end_base_time", move |input: &mut Input| {
        match [
            start_time.hour.is_some(),
            start_time.minute.is_some(),
            start_time.second.is_some(),
            start_time.millisecond.is_some(),
        ] {
            // Case 1: Full precision (%H:%M:%S.%ms)
            [true, true, true, true] => alt((
                time_seq!(PartialTime {
                    hour: time_hour.map(Some),
                    minute: preceded(literal(":"), time_minute).map(Some),
                    second: preceded(literal(":"), time_second).map(Some),
                    millisecond: opt(preceded(one_of(['.', ',']), fraction_millisecond)),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: time_minute.map(Some),
                    second: preceded(literal(":"), time_second).map(Some),
                    millisecond: opt(preceded(one_of(['.', ',']), fraction_millisecond)),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: opt(empty).map(|_| start_time.minute),
                    second: time_second.map(Some),
                    millisecond: opt(preceded(one_of(['.', ',']), fraction_millisecond)),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: opt(empty).map(|_| start_time.minute),
                    second: opt(empty).map(|_| start_time.second),
                    millisecond: opt(preceded(one_of(['.', ',']), fraction_millisecond)),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
            ))
            .parse_next(input),
            // Case 2: HH:MM:SS (no milliseconds)
            [true, true, true, false] => alt((
                time_seq!(PartialTime {
                    hour: time_hour.map(Some),
                    minute: preceded(literal(":"), time_minute).map(Some),
                    second: preceded(literal(":"), time_second).map(Some),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: time_minute.map(Some),
                    second: preceded(literal(":"), time_second).map(Some),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: opt(empty).map(|_| start_time.minute),
                    second: time_second.map(Some),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
            ))
            .parse_next(input),

            // Case 3: HH:MM (no seconds or milliseconds)
            [true, true, false, false] => alt((
                time_seq!(PartialTime {
                    hour: time_hour.map(Some),
                    minute: preceded(literal(":"), time_minute).map(Some),
                    second: opt(empty).map(|_| None),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: time_minute.map(Some),
                    second: opt(empty).map(|_| None),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
            ))
            .parse_next(input),

            // Case 4: HH only (no minutes, seconds, or milliseconds)
            [true, false, false, false] => time_seq!(PartialTime {
                hour: time_hour.map(Some),
                minute: opt(empty).map(|_| None),
                second: opt(empty).map(|_| None),
                millisecond: opt(empty).map(|_| None),
                offset: opt(offset).map(|o| o.unwrap_or(None)),
            })
            .parse_next(input),

            // Case 5: Invalid (no hour provided)
            [_, _, _, _] => fail.parse_next(input),
        }
    })
    .parse_next(i)
}

/// Parses secondary portion of a time string.
fn time_minute_second_millisecond<'i, Input>(
    i: &mut Input,
) -> PResult<(u32, Option<u32>, Option<u32>)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace(
        "time_minute_second_millisecond",
        move |input: &mut Input| {
            seq!(
                preceded(opt(literal(":")), time_minute),
                opt(preceded(opt(literal(":")), time_second)),
                opt(preceded(one_of(b",."), fraction_millisecond))
            )
            .parse_next(input)
        },
    )
    .parse_next(i)
}

/// Parses a offset offset string.
///
/// See [`offset()`][`crate::offset()`] for the supported formats.
// (Z|+...|-...)
pub fn offset<'i, Input>(i: &mut Input) -> PResult<Option<Offset>>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("offset_hour", move |input: &mut Input| {
        alt((offset_hour, offset_zulu)).parse_next(input)
    })
    .parse_next(i)
}

// Z
fn offset_zulu<'i, Input>(i: &mut Input) -> PResult<Option<Offset>>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("offset_zulu", move |input: &mut Input| {
        literal("Z")
            .map(|_| Some(Offset::default()))
            .parse_next(input)
    })
    .parse_next(i)
}

// DATETIME

/// Parses a datetime string.
///
/// See [`datetime()`][`crate::datetime()`] for supported formats.
// Full ISO8601 datetime
pub fn datetime<'i, Input>(i: &mut Input) -> PResult<DateTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("datetime", move |input: &mut Input| {
        separated_pair(date, literal("T"), base_time)
            .map(|(d, t)| DateTime { date: d, time: t })
            .parse_next(input)
    })
    .parse_next(i)
}

// partial date time
fn partial_datetime<'i, Input>(i: &mut Input) -> PResult<PartialDateTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_datetime", move |input: &mut Input| {
        seq!((
            opt(partial_date),
            opt(preceded(alt((literal(" "), literal("T"))), partial_time))
        ))
        .verify(|(d, t)| d.is_some() || t.is_some())
        .map(|(d, t)| PartialDateTime { date: d, time: t })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a possibly trunctated partial datetime string based on a partial start date
fn partial_end_date<'i, Input>(i: &mut Input, start_date: &PartialDate) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_end_date", move |input: &mut Input| {
        match start_date {
            PartialDate::Year { year: start_year } => {
                // Parse the year or use fallback only when parsing fails
                let year = match date_year.parse_next(input) {
                    Ok(parsed) => Some(parsed),
                    Err(ErrMode::Backtrack(_)) => *start_year,
                    Err(e) => return Err(e), // Propagate other errors
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
    .parse_next(i)
}

/// Sifts through portions of end_date parses for a Date::YDDD start_date
fn partial_end_date_yddd<'i, Input>(i: &mut Input, start_date: &PartialDate) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

/// Sifts through portions of end_date based on a Date::YMD start_date
fn partial_end_date_ymd<'i, Input>(i: &mut Input, start_date: &PartialDate) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
            _ => return Err(ErrMode::Backtrack(ContextError::new())),
        }
    })
    .parse_next(i)
}

/// Sifts through portions of end_date based on a Date::YMD start_date
fn partial_end_date_ywd<'i, Input>(i: &mut Input, start_date: &PartialDate) -> PResult<PartialDate>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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

fn partial_end_time<'i, Input>(i: &mut Input, start_time: &PartialTime) -> PResult<PartialTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_end_time", move |input: &mut Input| {
        let _ = opt(alt((literal(" "), literal("T")))).parse_next(input)?;

        partial_end_base_time(input, start_time)
    })
    .parse_next(i)
}

fn partial_end_datetime<'i, Input>(
    i: &mut Input,
    start_datetime: &PartialDateTime,
) -> PResult<PartialDateTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace(
        "partial_end_datetime",
        move |input: &mut Input| match start_datetime {
            PartialDateTime {
                date: start_date,
                time: start_time,
            } => {
                let mut end_date = None;
                let mut end_time = None;

                if start_date.is_none() && start_date.is_none() {
                    return Err(ErrMode::Backtrack(ContextError::new()));
                }

                if let Some(d) = start_date {
                    end_date = partial_end_date(input, d).map(Some)?;
                }

                if let Some(t) = start_time {
                    _ = literal(" ").parse_next(input)?;

                    end_time = partial_end_base_time(input, t).map(Some)?;
                }

                Ok(PartialDateTime {
                    date: end_date,
                    time: end_time,
                })
            }
        },
    )
    .parse_next(i)
}

// DURATION

///    dur-year          = 1*DIGIT "Y" [dur-month]
fn duration_part_year<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_year", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("Y")
        })
        .parse_next(input)
    })
    .parse_next(i)
}

///    dur-month         = 1*DIGIT "M" [dur-day]
fn duration_part_month<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_month", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("M")
        })
        .parse_next(input)
    })
    .parse_next(i)
}

///    dur-week          = 1*DIGIT "W"
fn duration_part_week<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_part_week", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("W")
        })
        .parse_next(input)
    })
    .parse_next(i)
}

//    dur-day           = 1*DIGIT "D"
fn duration_part_day<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_day", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("D")
        })
        .parse_next(input)
    })
    .parse_next(i)
}

///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
fn duration_part_hour<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_hour", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("H")
        })
        .parse_next(input)
    })
    .parse_next(i)
}

///    dur-minute        = 1*DIGIT "M" [dur-second]
fn duration_part_minute<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("M")
        })
        .parse_next(input)
    })
    .parse_next(i)
}

///    dur-second        = 1*DIGIT "S"
fn duration_part_second<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_second", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("S")
        })
        .parse_next(input)
    })
    .parse_next(i)
}

fn duration_part_time<'i, Input>(i: &mut Input) -> PResult<(Option<DurationPart>, Option<DurationPart>, Option<DurationPart>)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_time", move |input: &mut Input| {
        preceded(opt(literal("T")), duration_base_time).parse_next(input)
    })
    .parse_next(i)
}

fn duration_base_time<'i, Input>(i: &mut Input) -> PResult<(Option<DurationPart>, Option<DurationPart>, Option<DurationPart>)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_base_time", move |input: &mut Input| {
        seq!((
            opt(duration_part_hour),
            opt(duration_part_minute),
            opt(duration_part_second),
        ))
        .verify(|(h, m, s)| h.is_some() || m.is_some() || s.is_some())
        .map(|(h, m, s)| (h, m, s))
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a duration string with the format P%dY%dM%dDT%dH%dM%dS
pub fn duration<'i, Input>(i: &mut Input) -> PResult<Duration>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration", move |input: &mut Input| {
        seq!((
            _: literal("P"),
            opt(duration_part_year),
            opt(duration_part_month),
            opt(duration_part_week),
            opt(duration_part_day),
            opt(preceded(opt(literal("T")), duration_base_time)),
        ))
        .verify(|(y, mo, w, d, time)| {
            let (h, m, s) = time.unwrap_or((None, None, None));

            let p = [y, mo, w, d, &h, &m];

            (p.iter().any(|x| x.is_some() || s.is_some()))
            && p.iter().all(|x| x.is_none() || x.unwrap().frac.is_none())
        })
        .map(|(y, mo, w, d, time)| {
            let time = time.unwrap_or((None, None, None));
            Duration {
                years: y.map(|p| p.whole).unwrap_or(0),
                months: mo.map(|p| p.whole).unwrap_or(0),
                weeks: w.map(|p| p.whole).unwrap_or(0),
                days: d.map(|p| p.whole).unwrap_or(0),
                hours: time.0.map(|p| p.whole).unwrap_or(0),
                minutes: time.1.map(|p| p.whole).unwrap_or(0),
                seconds: time.2.map(|p| p.whole).unwrap_or(0),
                milliseconds: time.2.map(|p| p.frac).unwrap_or(None),
            }
            // at least one element must be present for a valid duration representation
        })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a duration string with the format P%dY%dM%dDT%dH%dM%dS
pub fn fractional_duration<'i, Input>(i: &mut Input) -> PResult<FractionalDuration>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("fractional_duration", move |input: &mut Input| {
        seq!((
            _: literal("P"),
            opt(duration_part_year),
            opt(duration_part_month),
            opt(duration_part_week),
            opt(duration_part_day),
            opt(preceded(opt(literal("T")), duration_base_time)),
        ))
            .verify(|(y, mo, w, d, time)| {
                if y.is_none() && mo.is_none() && w.is_none() && d.is_none() && time.is_none() {
                    false
                } else {
                    true
                }
            })
            .map(|(y, mo, w, d, time)| {
                let time = time.unwrap_or((None, None, None));

                FractionalDuration {
                    years: y.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                    months: mo.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                    weeks: w.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                    days: d.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                    hours: time.0.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                    minutes: time.1.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                    seconds: time.2.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                }
                // at least one element must be present for a valid duration representation
            })
            .parse_next(input)
    })
        .parse_next(i)
}

/// Parses a interval string containing combinations of partial date-times and duration.
pub fn interval<'i, Input>(i: &mut Input) -> PResult<Interval>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

fn interval_repetitions<'i, Input>(i: &mut Input) -> PResult<Option<u32>>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("interval_repetitions", move |input: &mut Input| {
        seq!((literal("R"), opt(take_digits), literal("/")))
            .map(|(_, r, _)| r)
            .parse_next(input)
    })
    .parse_next(i)
}

fn interval_open<'i, Input>(i: &mut Input) -> PResult<IntervalRange>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("interval_closed", move |input: &mut Input| {
        duration(input).map(|duration| IntervalRange::Open { duration })
    })
    .parse_next(i)
}

fn interval_closed<'i, Input>(i: &mut Input) -> PResult<IntervalRange>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("interval_closed", move |input: &mut Input| {
        let start = partial_datetime(input)?;
        let _ = literal("/").parse_next(input)?;
        let end = partial_end_datetime(input, &start)?;

        Ok(IntervalRange::Closed { start, end })
    })
    .parse_next(i)
}

fn interval_closed_end<'i, Input>(i: &mut Input) -> PResult<IntervalRange>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("interval_closed_end", move |input: &mut Input| {
        seq!(IntervalRange::ClosedEnd {
            duration: duration,
            _: literal("/"),
            end: partial_datetime,
        })
        .parse_next(input)
    })
    .parse_next(i)
}

fn interval_closed_start<'i, Input>(i: &mut Input) -> PResult<IntervalRange>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("interval_closed_start", move |input: &mut Input| {
        seq!( IntervalRange::ClosedStart {
            start: partial_datetime,
            _: literal("/"),
            duration: duration,
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// (+...|-...)
fn offset_hour<'i, Input>(i: &mut Input) -> PResult<Option<Offset>>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("offset_hour", move |input: &mut Input| {
        seq!((
            sign,
            time_hour,
            opt(preceded(opt(literal(":")), time_minute))
        ))
        .verify(|(s, h, m)| !(*s == -1 && h * 1 == 0 && (m.is_none() || m.unwrap() * 1 == 0)))
        .map(|(s, h, m)| {
            Some(Offset {
                offset_hours: s * (h as i32),
                offset_minutes: s * (m.unwrap_or(0) as i32),
            })
        })
        .parse_next(input)
    })
    .parse_next(i)
}
