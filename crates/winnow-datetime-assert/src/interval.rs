use crate::{FormatAssertion, FormatAssertionBuilder};
use winnow_datetime::types::{IntervalRange, PartialDate, PartialDateTime, PartialTime};
use winnow_datetime::{Duration, Interval};

pub struct IntervalAssertion {
    assertions: Vec<FormatAssertion<Interval>>,
}

impl FormatAssertionBuilder<Interval> for IntervalAssertion {
    fn base_assertions(&self) -> Vec<FormatAssertion<Interval>> {
        vec![]
    }

    fn assertions(&self) -> Vec<FormatAssertion<Interval>> {
        self.assertions.clone()
    }
}

pub fn assertions() -> IntervalAssertion {
    IntervalAssertion {
        assertions: vec![
            FormatAssertion {
                format: "%Y-%M-%D/P1Y".into(),
                input: "2024-12-22/P1Y".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            years: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/P1M".into(),
                input: "2024-12-22/P1M".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            months: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/P1D".into(),
                input: "2024-12-22/P1D".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            days: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%w/P1Y".into(),
                input: "2024-W51-7/P1Y".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            years: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%w/P1M".into(),
                input: "2024-W51-7/P1M".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            months: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%w/P1D".into(),
                input: "2024-W51-7/P1D".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            days: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/P1Y".into(),
                input: "2024-357/P1Y".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            years: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/P1M".into(),
                input: "2024-357/P1M".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            months: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/P1D".into(),
                input: "2024-357/P1D".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        duration: Duration {
                            days: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/%Y-%M-%D".into(),
                input: "2024-12-22/2024-12-22".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/%V-W%W-%w".into(),
                input: "2024-12-22/2024-W51-7".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/%Y-%O".into(),
                input: "2024-12-22/2024-357".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%w/%Y-%M-%D".into(),
                input: "2024-W51-7/2024-12-22".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%w/%V-W%W-%w".into(),
                input: "2024-W51-7/2024-W51-7".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%w/%Y-%O".into(),
                input: "2024-W51-7/2024-357".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/%Y-%M-%D".into(),
                input: "2024-357/2024-12-22".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/%V-W%W-%w".into(),
                input: "2024-357/2024-W51-7".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/%Y-%O".into(),
                input: "2024-357/2024-357".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1Y/%Y-%M-%D".into(),
                input: "P1Y/2024-12-22".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            years: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1Y/%V-W%W-%w".into(),
                input: "P1Y/2024-W51-7".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            years: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1Y/%Y-%O".into(),
                input: "P1Y/2024-357".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            years: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1M/%Y-%M-%D".into(),
                input: "P1M/2024-12-22".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            months: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1M/%V-W%W-%w".into(),
                input: "P1M/2024-W51-7".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            months: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1M/%Y-%O".into(),
                input: "P1M/2024-357".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            months: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1D/%Y-%M-%D".into(),
                input: "P1D/2024-12-22".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1D/%V-W%W-%w".into(),
                input: "P1D/2024-W51-7".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1D/%Y-%O".into(),
                input: "P1D/2024-357".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%DT%h/P1DT1H".into(),
                input: "2024-12-22T07/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: None,
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%DT%h:%m/P1DT1H".into(),
                input: "2024-12-22T07:42/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%DT%h:%m:%s/P1DT1H".into(),
                input: "2024-12-22T07:42:55/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%DT%h:%m:%.3s/P1DT1H".into(),
                input: "2024-12-22T07:42:55.870/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: Some(870),
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%DT%h:%mZ/P1DT1H".into(),
                input: "2024-12-22T07:42Z/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: Some(Default::default()),
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%wT%h/P1DT1H".into(),
                input: "2024-W51-7T07/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: None,
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%wT%h:%m/P1DT1H".into(),
                input: "2024-W51-7T07:42/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%wT%h:%m:%s/P1DT1H".into(),
                input: "2024-W51-7T07:42:55/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%wT%h:%m:%.3s/P1DT1H".into(),
                input: "2024-W51-7T07:42:55.870/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: Some(870),
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%wT%h:%mZ/P1DT1H".into(),
                input: "2024-W51-7T07:42Z/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: Some(Default::default()),
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%OT%h/P1DT1H".into(),
                input: "2024-357T07/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: None,
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%OT%h:%m/P1DT1H".into(),
                input: "2024-357T07:42/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%OT%h:%m:%s/P1DT1H".into(),
                input: "2024-357T07:42:55/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%OT%h:%m:%.3s/P1DT1H".into(),
                input: "2024-357T07:42:55.870/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: Some(870),
                                offset: None,
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%OT%h:%mZ/P1DT1H".into(),
                input: "2024-357T07:42Z/P1DT1H".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedStart {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: Some(Default::default()),
                            }),
                        },
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%M-%DT%h".into(),
                input: "P1DT1H/2024-12-22T07".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: None,
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%M-%DT%h:%m".into(),
                input: "P1DT1H/2024-12-22T07:42".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%M-%DT%h:%m:%s".into(),
                input: "P1DT1H/2024-12-22T07:42:55".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%M-%DT%h:%m:%.3s".into(),
                input: "P1DT1H/2024-12-22T07:42:55.870".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: Some(870),
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%M-%DT%h:%mZ".into(),
                input: "P1DT1H/2024-12-22T15:42Z".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(15),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: Some(Default::default()),
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%V-W%W-%wT%h".into(),
                input: "P1DT1H/2024-W51-7T07".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: None,
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%V-W%W-%wT%h:%m".into(),
                input: "P1DT1H/2024-W51-7T07:42".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%V-W%W-%wT%h:%m:%s".into(),
                input: "P1DT1H/2024-W51-7T07:42:55".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%V-W%W-%wT%h:%m:%.3s".into(),
                input: "P1DT1H/2024-W51-7T07:42:55.870".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: Some(870),
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%V-W%W-%wT%h:%mZ".into(),
                input: "P1DT1H/2024-W51-7T15:42Z".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: Some(PartialTime {
                                hour: Some(15),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: Some(Default::default()),
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%OT%h".into(),
                input: "P1DT1H/2024-357T07".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: None,
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%OT%h:%m".into(),
                input: "P1DT1H/2024-357T07:42".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%OT%h:%m:%s".into(),
                input: "P1DT1H/2024-357T07:42:55".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%OT%h:%m:%.3s".into(),
                input: "P1DT1H/2024-357T07:42:55.870".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(7),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: Some(870),
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "P1DT1H/%Y-%OT%h:%mZ".into(),
                input: "P1DT1H/2024-357T15:42Z".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::ClosedEnd {
                        duration: Duration {
                            days: 1,
                            hours: 1,
                            ..Default::default()
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: Some(PartialTime {
                                hour: Some(15),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: Some(Default::default()),
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y/%Y".into(),
                input: "2024/2024".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::Year { year: Some(2024) }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::Year { year: Some(2024) }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M/%Y-%M".into(),
                input: "2024-12/2024-12".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: None,
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: None,
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/%Y-%M-%D".into(),
                input: "2024-12-22/2024-12-23".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(23),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/%M-%D".into(),
                input: "2024-12-22/12-23".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(23),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/%D".into(),
                input: "2024-12-22/23".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(23),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M/%Y-%M".into(),
                input: "2024-12/2024-12".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: None,
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: None,
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y/%Y".into(),
                input: "2024/2024".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::Year { year: Some(2024) }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::Year { year: Some(2024) }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/%Y-%M".into(),
                input: "2024-12-22/2024-12".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: None,
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D/%Y".into(),
                input: "2024-12-22/2024".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::Year { year: Some(2024) }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/%Y-%O".into(),
                input: "2024-357/2024-358".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(358),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/%O".into(),
                input: "2024-357/358".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(358),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%O/%Y".into(),
                input: "2024-357/2024".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YDDD {
                                year: Some(2024),
                                day: Some(357),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::Year { year: Some(2024) }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-W%W/%Y-W%W".into(),
                input: "2024-W51/2024-W52".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: None,
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(52),
                                day: None,
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-W%W-%w/%Y-W%W-%w".into(),
                input: "2024-W51-7/2024-W52-1".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(52),
                                day: Some(1),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-W%W/%W".into(),
                input: "2024-W51/52".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: None,
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(52),
                                day: None,
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-W%W-%w/%W-%w".into(),
                input: "2024-W51-7/52-1".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(52),
                                day: Some(1),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-W%W-%w/%w".into(),
                input: "2024-W51-7/1".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(7),
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: Some(1),
                            }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-W%W/%Y".into(),
                input: "2024-W51/2024".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YWD {
                                year: Some(2024),
                                week: Some(51),
                                day: None,
                            }),
                            time: None,
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::Year { year: Some(2024) }),
                            time: None,
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D %h:%m:%s/%Y-%M-%D %h:%m:%s".into(),
                input: "2024-12-22 15:42:55/2024-12-23 18:45:32".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(15),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(23),
                            }),
                            time: Some(PartialTime {
                                hour: Some(18),
                                minute: Some(45),
                                second: Some(32),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D %h:%m:%s/%M-%D %h:%m:%s".into(),
                input: "2024-12-22 15:42:55/12-23 18:45:32".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(15),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(23),
                            }),
                            time: Some(PartialTime {
                                hour: Some(18),
                                minute: Some(45),
                                second: Some(32),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D %h:%m:%s/%D %h:%m:%s".into(),
                input: "2024-12-22 15:42:55/23 18:45:32".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(15),
                                minute: Some(42),
                                second: Some(55),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(23),
                            }),
                            time: Some(PartialTime {
                                hour: Some(18),
                                minute: Some(45),
                                second: Some(32),
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D %h:%m/%Y-%M-%D %h:%m".into(),
                input: "2024-12-22 15:42/2024-12-23 18:45".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(15),
                                minute: Some(42),
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(23),
                            }),
                            time: Some(PartialTime {
                                hour: Some(18),
                                minute: Some(45),
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
            FormatAssertion {
                format: "%Y-%M-%D %h/%Y-%M-%D %h".into(),
                input: "2024-12-22 15/2024-12-23 18".into(),
                expected: Ok(Interval {
                    repetitions: None,
                    range: IntervalRange::Closed {
                        start: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(22),
                            }),
                            time: Some(PartialTime {
                                hour: Some(15),
                                minute: None,
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                        end: PartialDateTime {
                            date: Some(PartialDate::YMD {
                                year: Some(2024),
                                month: Some(12),
                                day: Some(23),
                            }),
                            time: Some(PartialTime {
                                hour: Some(18),
                                minute: None,
                                second: None,
                                millisecond: None,
                                offset: None,
                            }),
                        },
                    },
                }),
            },
        ],
    }
}
