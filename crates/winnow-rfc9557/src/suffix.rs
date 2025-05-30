use crate::calendar::calendar;
use crate::time_zone::time_zone;
use winnow::combinator::opt;
use winnow::combinator::trace;
use winnow::error::ParserError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
use winnow_datetime::types::Calendar;
use winnow_datetime::types::TimeZone;
use winnow_datetime::Offset;

pub fn suffix_start<'a, Input, Error>(input: &mut Input) -> Result<bool, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("suffix_start", move |input: &mut Input| {
        seq!(literal("["), opt(literal("!")),)
            .map(|(_, critical)| critical.is_some())
            .parse_next(input)
    })
    .parse_next(input)
}

pub fn suffix_end<'a, Input, Error>(input: &mut Input) -> Result<(), Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("suffix_start", move |input: &mut Input| {
        literal("]").map(|_| ()).parse_next(input)
    })
    .parse_next(input)
}

pub fn suffix_time_zone<'a, Input, Error>(input: &mut Input) -> Result<TimeZone, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("suffix_time_zone", move |input: &mut Input| {
        seq!(
            suffix_start,
            time_zone,
            _: suffix_end,
        )
        .map(|(c, tz): (bool, TimeZone)| match tz {
            TimeZone::Named { zone: mut tz } => {
                tz.critical = c;
                TimeZone::Named { zone: tz }
            }
            TimeZone::Fixed { offset: o } => match o {
                Offset::Fixed {
                    hours,
                    minutes,
                    critical: _,
                } => TimeZone::Fixed {
                    offset: Offset::Fixed {
                        hours,
                        minutes,
                        critical: c,
                    },
                },
                Offset::LocalUnknown { critical: _ } => TimeZone::Fixed {
                    offset: Offset::LocalUnknown { critical: c },
                },
            },
        })
        .parse_next(input)
    })
    .parse_next(input)
}

pub fn suffix_calendar<'a, Input, Error>(input: &mut Input) -> Result<Calendar, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("suffix_calendar", move |input: &mut Input| {
        seq!(
            suffix_start,
            _: literal("u-ca="),
            calendar,
            _: suffix_end,
        )
        .map(|(critical, mut c): (bool, Calendar)| {
            c.critical = critical;
            c
        })
        .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod test {
    use crate::parse_time;
    use winnow_datetime::types::NamedTimeZone;
    use winnow_datetime::{Calendar, Offset, Time, TimeZone};

    #[test]
    fn test_suffix_critical_named_timezone() {
        let result = parse_time("16:43:16Z[!America/New_York]").unwrap();
        assert_eq!(
            result,
            Time {
                hour: 16,
                minute: 43,
                second: 16,
                millisecond: 0,
                offset: Some(Offset::LocalUnknown { critical: false }),
                time_zone: Some(TimeZone::Named {
                    zone: NamedTimeZone {
                        identifier: "America/New_York".to_string(),
                        critical: true
                    }
                }),
                calendar: None,
            }
        );
    }

    #[test]
    fn test_suffix_critical_named_timezone_and_cal() {
        let result = parse_time("16:43:16Z[!America/New_York][!u-ca=gregory]").unwrap();
        assert_eq!(
            result,
            Time {
                hour: 16,
                minute: 43,
                second: 16,
                millisecond: 0,
                offset: Some(Offset::LocalUnknown { critical: false }),
                time_zone: Some(TimeZone::Named {
                    zone: NamedTimeZone {
                        identifier: "America/New_York".to_string(),
                        critical: true
                    }
                }),
                calendar: Some(Calendar {
                    identifier: "gregory".to_string(),
                    critical: true,
                }),
            }
        );
    }

    #[test]
    fn test_suffix_critical_named_timezone_and_mutl_cal() {
        let result = parse_time(
            "16:43:16Z[!America/New_York][!u-ca=gregory][u-ca=iso8601][u-ca=islamic-umalqura]",
        )
        .unwrap();
        assert_eq!(
            result,
            Time {
                hour: 16,
                minute: 43,
                second: 16,
                millisecond: 0,
                offset: Some(Offset::LocalUnknown { critical: false }),
                time_zone: Some(TimeZone::Named {
                    zone: NamedTimeZone {
                        identifier: "America/New_York".to_string(),
                        critical: true
                    }
                }),
                calendar: Some(Calendar {
                    identifier: "gregory".to_string(),
                    critical: true,
                }),
            }
        );
    }
}
