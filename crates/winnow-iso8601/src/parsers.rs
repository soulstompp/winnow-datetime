//! The low-level parsers for date, datetime, duration and time.
//!
//! The top-level functions [`date()`][`crate::date()`], [`datetime()`][`crate::datetime()`],
//! [`duration()`][`crate::duration()`] and [`time()`][`crate::time()`]
//! provide convenient wrappers around the low-level parsers,
//! but throw away leftover input on success.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use crate::{DateTime, Duration, Time, Timezone};
use core::str;
use winnow::combinator::opt;
use winnow::combinator::{alt, trace};
use winnow::combinator::{preceded, separated_pair, terminated};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::literal;
use winnow::token::one_of;
use winnow::{seq, PResult, Parser, Partial};
use winnow_datetime::parsers::sign;
use winnow_datetime::parsers::take_digits;
use winnow_datetime::parsers::take_digits_in_range;
use winnow_datetime::parsers::{date_year, fraction_millisecond, time_hour, time_minute};
use winnow_datetime::parsers::{date_ymd, time_second};

#[cfg(test)]
mod tests;

/// Type for holding partial data for parsers
pub type Stream<'i> = Partial<&'i [u8]>;

// DATE
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
fn date_ordinal<'i, Input>(i: &mut Input) -> PResult<winnow_datetime::Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ordinal", move |input: &mut Input| {
        separated_pair(date_year, opt(literal("-")), date_ord_day)
            .map(|(year, ddd)| winnow_datetime::Date::Ordinal { year, ddd })
            .parse_next(input)
    })
    .parse_next(i)
}

// YYYY-"W"WW-D
fn date_iso_week<'i, Input>(i: &mut Input) -> PResult<winnow_datetime::Date>
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
            opt(literal("-")),                       // [-]
            date_week_day,                           // d
        ))
        .map(|(year, _, ww, _, d)| winnow_datetime::Date::Week { year, ww, d })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub fn parse_date<'i, Input>(i: &mut Input) -> PResult<winnow_datetime::Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_date", move |input: &mut Input| {
        alt((date_ymd, date_iso_week, date_ordinal)).parse_next(input)
    })
    .parse_next(i)
}

// WW
fn date_week<'i, Input>(i: &mut Input) -> PResult<u32>
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

fn date_week_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_week_day", move |input: &mut Input| {
        take_digits_in_range(input, 1, 1..=7)
    })
    .parse_next(i)
}

// TIME

/// Parses a time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn parse_time<'i, Input>(i: &mut Input) -> PResult<Time>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_time", move |input: &mut Input| {
        let t = seq! {Time {
            hour: time_hour,                                         // HH
            _: opt(literal(":")),                                    // :
            minute: time_minute,                                       // MM
            second: opt(preceded(opt(literal(":")), time_second)).map(|d| d.unwrap_or(0)),        // [SS]
            millisecond: opt(preceded(one_of(b",."), fraction_millisecond)).map(|d| d.unwrap_or(0)), // [.(m*)]
            timezone: opt(parse_timezone).map(|tz| tz.unwrap_or(Default::default())),           // [(Z|+...|-...)]
        }}
        .parse_next(input)?;
        Ok(t)
    })
    .parse_next(i)
}

/// Parses a timezone offset string.
///
/// See [`timezone()`][`crate::timezone()`] for the supported formats.
// (Z|+...|-...)
pub fn parse_timezone<'i, Input>(i: &mut Input) -> PResult<Timezone>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("timezone_hour", move |input: &mut Input| {
        alt((timezone_hour, timezone_utc)).parse_next(input)
    })
    .parse_next(i)
}

// Z
fn timezone_utc<'i, Input>(i: &mut Input) -> PResult<Timezone>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("timezone_utc", move |input: &mut Input| {
        literal("Z").map(|_| Timezone::default()).parse_next(input)
    })
    .parse_next(i)
}

/// Parses a datetime string.
///
/// See [`datetime()`][`crate::datetime()`] for supported formats.
// Full ISO8601 datetime
pub fn parse_datetime<'i, Input>(i: &mut Input) -> PResult<DateTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_datetime", move |input: &mut Input| {
        separated_pair(parse_date, literal("T"), parse_time)
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
fn timezone_hour<'i, Input>(i: &mut Input) -> PResult<Timezone>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("timezone_hour", move |input: &mut Input| {
        seq!((
            sign,
            time_hour,
            opt(preceded(opt(literal(":")), time_minute))
        ))
        .map(|(s, h, m)| Timezone {
            offset_hours: s * (h as i32),
            offset_minutes: s * (m.unwrap_or(0) as i32),
        })
        .parse_next(input)
    })
    .parse_next(i)
}
