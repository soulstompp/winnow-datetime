use crate::partial_date::{partial_date, partial_end_date};
use crate::partial_time::{partial_end_base_time, partial_time};
use core::str;
use winnow::combinator::{alt, opt, preceded, trace};
use winnow::error::ParserError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
use winnow_datetime::types::PartialDateTime;

// partial date time
pub(crate) fn partial_datetime<'i, Input, Error>(
    input: &mut Input,
) -> Result<PartialDateTime, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
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
    .parse_next(input)
}

pub(crate) fn partial_end_datetime<'i, Input, Error>(
    input: &mut Input,
    start_datetime: &PartialDateTime,
) -> Result<PartialDateTime, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
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
                    return Err(ParserError::from_input(input));
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
    .parse_next(input)
}
#[cfg(test)]
mod parsers {
    use crate::partial_datetime::{partial_datetime, partial_end_datetime};
    use winnow::error::InputError;

    use winnow_datetime::types::{PartialDate, PartialDateTime};

    #[test]
    fn partial_datetime_parsing() {
        // Year
        assert_eq!(
            partial_datetime::<_, InputError<_>>(&mut "2015").unwrap(),
            PartialDateTime {
                date: Some(PartialDate::Year { year: Some(2015) }),
                time: None,
            }
        );
        // YMD
        assert_eq!(
            partial_datetime::<_, InputError<_>>(&mut "2015-06-26").unwrap(),
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
            partial_datetime::<_, InputError<_>>(&mut "2015-06").unwrap(),
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
            partial_datetime::<_, InputError<_>>(&mut "2015-W05-6").unwrap(),
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
            partial_datetime::<_, InputError<_>>(&mut "2015-W05-1").unwrap(),
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
            partial_datetime::<_, InputError<_>>(&mut "2015-W05").unwrap(),
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
            partial_datetime::<_, InputError<_>>(&mut "2015-156").unwrap(),
            PartialDateTime {
                date: Some(PartialDate::YDDD {
                    year: Some(2015),
                    day: Some(156)
                }),
                time: None,
            }
        );
        assert_eq!(
            partial_datetime::<_, InputError<_>>(&mut "2015-156").unwrap(),
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
    fn partial_ymd() {
        assert_eq!(
            partial_end_datetime::<_, InputError<_>>(
                &mut "2015-06-26",
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
            partial_end_datetime::<_, InputError<_>>(
                &mut "06-26",
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
            partial_end_datetime::<_, InputError<_>>(
                &mut "26",
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
    fn partial_ywd() {
        assert_eq!(
            partial_end_datetime::<_, InputError<_>>(
                &mut "2024-W51-4",
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
            partial_end_datetime::<_, InputError<_>>(
                &mut "W51-4",
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
            partial_end_datetime::<_, InputError<_>>(
                &mut "4",
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
    fn partial_yddd() {
        assert_eq!(
            partial_end_datetime::<_, InputError<_>>(
                &mut "083",
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
}
