//! The low-level parsers for date, datetime, duration and time.
//!
//! The top-level functions [`date()`][`crate::date()`], [`datetime()`][`crate::datetime()`],
//! [`duration()`][`crate::duration()`] and [`time()`][`crate::time()`]
//! provide convenient wrappers around the low-level parsers,
//! but throw away leftover input on success.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use crate::duration::Duration;
use core::str;
use winnow::combinator::opt;
use winnow::combinator::{alt, trace};
use winnow::combinator::{preceded, separated_pair, terminated};
use winnow::error::{ContextError, ErrMode};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::one_of;
use winnow::token::{literal, take_while};
use winnow::{seq, PResult, Parser, Partial};
use winnow_datetime::parsers::date_day;
use winnow_datetime::parsers::date_month;
use winnow_datetime::parsers::sign;
use winnow_datetime::parsers::take_digits;
use winnow_datetime::parsers::take_digits_in_range;
use winnow_datetime::parsers::{fraction_millisecond, time_hour, time_minute, time_second};
use winnow_datetime::{Date, DateTime, Offset, Time};

#[cfg(test)]
mod tests;

/// Type for holding partial data for parsers
pub type Stream<'i> = Partial<&'i [u8]>;

// DATE
/// Parses 2 digit week of the year within range 01-52
// WW
pub fn date_week<'i, Input>(i: &mut Input) -> PResult<u32>
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

fn unverified_date_week_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("unverified_date_week_day", move |input: &mut Input| {
        take_digits_in_range(input, 1, 0..=9)
    })
    .parse_next(i)
}

/// Parses 1 digit day of the week within range 1-7
pub fn date_week_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_week_day", move |input: &mut Input| {
        unverified_date_week_day(input)
    })
    .verify(|d| *d > 0 && *d <= 7)
    .parse_next(i)
}

/// Parses a date string as ISO 8601 week date.
// YYYY-"W"WW-D
pub fn date_iso_week<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        seq!((
            date_year,                               // y
            seq!((opt(literal("-")), literal("W"))), // [-]W
            date_week,                               // w
            opt(unverified_date_iso_week_day)
        ))
        .verify(|(_, _, _, d)| d.is_none() || d.unwrap() > 0 && d.unwrap() <= 7)
        .map(|(year, _, ww, d)| Date::Week {
            year,
            ww,
            d: d.unwrap_or(0),
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// [-]D - unverified
fn unverified_date_iso_week_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        seq!((
            _: opt(literal("-")),                       // [-]
            unverified_date_week_day,                           // d
        ))
        .map(|d| d.0)
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a year with +/- and will eventually support 6 digit year
// [+/-]YYYY
pub fn date_year<'i, Input>(i: &mut Input) -> PResult<i32>
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

// ordinal DDD
fn date_ord_day<'i, Input>(i: &mut Input) -> PResult<u32>
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
fn date_ordinal<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ordinal", move |input: &mut Input| {
        separated_pair(date_year, opt(literal("-")), date_ord_day)
            .map(|(year, ddd)| Date::Ordinal { year, ddd })
            .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a date string specificed as YYYYMMDD
pub fn date_ymd_numeric<'i, Input>(i: &mut Input) -> PResult<Date>
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

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub fn date<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_date", move |input: &mut Input| {
        alt((date_iso_week, date_ymd_numeric, date_ordinal, date_ymd)).parse_next(input)
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
    trace("parse_time", move |input: &mut Input| {
        seq!((
            _: opt(literal("T")),
            base_time
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
            seq!((
                preceded(opt(literal(":")), time_minute),
                opt(preceded(opt(literal(":")), time_second)),
                opt(preceded(one_of(b",."), fraction_millisecond))
            ))
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
    trace("parse_datetime", move |input: &mut Input| {
        separated_pair(date, literal("T"), base_time)
            .map(|(d, t)| DateTime { date: d, time: t })
            .parse_next(input)
    })
    .parse_next(i)
}

// DURATION

///    dur-year          = 1*DIGIT "Y" [dur-month]
fn duration_year<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_year", move |input: &mut Input| {
        (terminated(take_digits, literal("Y"))).parse_next(input)
    })
    .parse_next(i)
}

///    dur-month         = 1*DIGIT "M" [dur-day]
fn duration_month<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_month", move |input: &mut Input| {
        (terminated(take_digits, literal("M"))).parse_next(input)
    })
    .parse_next(i)
}

///    dur-week          = 1*DIGIT "W"
fn duration_week<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_week", move |input: &mut Input| {
        let d = take_digits(input)?;
        let _ = literal("W").parse_next(input)?;

        Ok(d)
    })
    .parse_next(i)
}

//    dur-day           = 1*DIGIT "D"
fn duration_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_day", move |input: &mut Input| {
        terminated(take_digits, literal("D")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
fn duration_hour<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_hour", move |input: &mut Input| {
        terminated(take_digits, literal("H")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-minute        = 1*DIGIT "M" [dur-second]
fn duration_minute<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        terminated(take_digits, literal("M")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-second        = 1*DIGIT "S"
fn duration_second<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_second", move |input: &mut Input| {
        terminated(take_digits, literal("S")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-second-ext    = 1*DIGIT (,|.) 1*DIGIT "S"
fn duration_second_and_millisecond<'i, Input>(i: &mut Input) -> PResult<(u32, u32)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace(
        "duration_second_and_millisecond",
        move |input: &mut Input| {
            alt((
                // no milliseconds
                duration_second.map(|m| (m, 0)),
                terminated(
                    // with milliseconds
                    separated_pair(take_digits, one_of(b",."), fraction_millisecond),
                    literal("S"),
                ),
            ))
            .parse_next(input)
        },
    )
    .parse_next(i)
}

fn duration_time<'i, Input>(i: &mut Input) -> PResult<(u32, u32, u32, u32)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_time", move |input: &mut Input| {
        seq!((
            literal("T"),
            opt(duration_hour),
            opt(duration_minute),
            opt(duration_second_and_millisecond),
        ))
        .map(|(_, h, m, s)| {
            let (s, ms) = s.unwrap_or((0, 0));

            (h.unwrap_or(0), m.unwrap_or(0), s, ms)
        })
        .parse_next(input)
    })
    .parse_next(i)
}

pub(crate) fn duration<'i, Input>(i: &mut Input) -> PResult<Duration>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        seq!((
            _: literal("P"),
            opt(duration_year),
            opt(duration_month),
            opt(duration_week),
            opt(duration_day),
            opt(duration_time),
        ))
        .verify(|(y, mo, w, d, time)| {
            if y.is_none() && mo.is_none() && w.is_none() && d.is_none() && time.is_none() {
                false
            } else {
                true
            }
        })
        .map(|(y, mo, w, d, time)| {
            // at least one element must be present for a valid duration representation

            let (h, mi, s, ms) = time.unwrap_or((0, 0, 0, 0));

            Duration {
                years: y.unwrap_or(0),
                months: mo.unwrap_or(0),
                weeks: w.unwrap_or(0),
                days: d.unwrap_or(0),
                hours: h,
                minutes: mi,
                seconds: s,
                milliseconds: ms,
            }
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
