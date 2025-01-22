use crate::partial_date::{partial_date, partial_end_date};
use crate::partial_time::{partial_end_base_time, partial_time};
use core::str;
use winnow::combinator::{alt, opt, preceded, trace};
use winnow::error::ContextError;
use winnow::error::ErrMode;
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, PResult, Parser};
use winnow_datetime::types::PartialDateTime;

// partial date time
pub(crate) fn partial_datetime<'i, Input>(i: &mut Input) -> PResult<PartialDateTime>
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

pub(crate) fn partial_end_datetime<'i, Input>(
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
#[cfg(test)]
mod parsers {
    use crate::partial_datetime::{partial_datetime, partial_end_datetime};
    use winnow::stream::AsBStr;
    use winnow_datetime::types::{PartialDate, PartialDateTime};

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
    fn partial_ymd() {
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
    fn partial_ywd() {
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
    fn partial_yddd() {
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
}
