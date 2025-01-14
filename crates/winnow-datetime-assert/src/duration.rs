use crate::{FormatAssertion, FormatAssertionBuilder};
use winnow_datetime::Duration;

pub struct DurationAssertion {
    assertions: Vec<FormatAssertion<Duration>>,
}

impl FormatAssertionBuilder<Duration> for DurationAssertion {
    fn base_assertions(&self) -> Vec<FormatAssertion<Duration>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Duration>> {
        self.base_assertions()
    }
}

pub fn assertions() -> DurationAssertion {
    DurationAssertion {
        assertions: vec![
            // Format
            // P1Y
            FormatAssertion {
                format: "P1Y".into(),
                input: "P1Y".into(),
                expected: Ok(Duration {
                    years: 1,
                    ..Default::default()
                }),
            },
            // P1M
            FormatAssertion {
                format: "P1M".into(),
                input: "P1M".into(),
                expected: Ok(Duration {
                    months: 1,
                    ..Default::default()
                }),
            },
            // P1W
            FormatAssertion {
                format: "P1W".into(),
                input: "P1W".into(),
                expected: Ok(Duration {
                    weeks: 1,
                    ..Default::default()
                }),
            },
            // P1D
            FormatAssertion {
                format: "P1D".into(),
                input: "P1D".into(),
                expected: Ok(Duration {
                    days: 1,
                    ..Default::default()
                }),
            },
            // PT1H
            FormatAssertion {
                format: "PT1H".into(),
                input: "PT1H".into(),
                expected: Ok(Duration {
                    hours: 1,
                    ..Default::default()
                }),
            },
            // P1H
            FormatAssertion {
                format: "P1H".into(),
                input: "P1H".into(),
                expected: Ok(Duration {
                    hours: 1,
                    ..Default::default()
                }),
            },
            // PT1M
            FormatAssertion {
                format: "PT1M".into(),
                input: "PT1M".into(),
                expected: Ok(Duration {
                    minutes: 1,
                    ..Default::default()
                }),
            },
            // PT1S
            FormatAssertion {
                format: "PT1S".into(),
                input: "PT1S".into(),
                expected: Ok(Duration {
                    seconds: 1,
                    ..Default::default()
                }),
            },
            // P1S
            FormatAssertion {
                format: "P1S".into(),
                input: "P1S".into(),
                expected: Ok(Duration {
                    seconds: 1,
                    ..Default::default()
                }),
            },
            // P1Y1M
            FormatAssertion {
                format: "P1Y1M".into(),
                input: "P1Y1M".into(),
                expected: Ok(Duration {
                    years: 1,
                    months: 1,
                    ..Default::default()
                }),
            },
            // P1Y1D
            FormatAssertion {
                format: "P1Y1D".into(),
                input: "P1Y1D".into(),
                expected: Ok(Duration {
                    years: 1,
                    days: 1,
                    ..Default::default()
                }),
            },
            // P1Y1M1D
            FormatAssertion {
                format: "P1Y1M1D".into(),
                input: "P1Y1M1D".into(),
                expected: Ok(Duration {
                    years: 1,
                    months: 1,
                    days: 1,
                    ..Default::default()
                }),
            },
            // P1Y1M1DT1H1M1S
            FormatAssertion {
                format: "P1Y1M1DT1H1M1S".into(),
                input: "P1Y1M1DT1H1M1S".into(),
                expected: Ok(Duration {
                    years: 1,
                    months: 1,
                    days: 1,
                    hours: 1,
                    minutes: 1,
                    seconds: 1,
                    ..Default::default()
                }),
            },
            // P1DT1H
            FormatAssertion {
                format: "P1DT1H".into(),
                input: "P1DT1H".into(),
                expected: Ok(Duration {
                    days: 1,
                    hours: 1,
                    ..Default::default()
                }),
            },
            // P1MT1M
            FormatAssertion {
                format: "P1MT1M".into(),
                input: "P1MT1M".into(),
                expected: Ok(Duration {
                    months: 1,
                    minutes: 1,
                    ..Default::default()
                }),
            },
            // P1DT1M
            FormatAssertion {
                format: "P1DT1M".into(),
                input: "P1DT1M".into(),
                expected: Ok(Duration {
                    days: 1,
                    minutes: 1,
                    ..Default::default()
                }),
            },
            // 1W1M1S
            FormatAssertion {
                format: "1W1M1S".into(),
                input: "1W1M1S".into(),
                expected: Ok(Duration {
                    days: 7,
                    minutes: 1,
                    seconds: 1,
                    ..Default::default()
                }),
            },
            // 1S1M1H1W
            FormatAssertion {
                format: "1S1M1H1W".into(),
                input: "1S1M1H1W".into(),
                expected: Ok(Duration {
                    days: 7,
                    hours: 1,
                    minutes: 1,
                    seconds: 1,
                    ..Default::default()
                }),
            },
            // 1 W
            FormatAssertion {
                format: "1 W".into(),
                input: "1 W".into(),
                expected: Ok(Duration {
                    days: 7,
                    ..Default::default()
                }),
            },
            // 1 D 1 W
            FormatAssertion {
                format: "1 D 1 W".into(),
                input: "1 D 1 W".into(),
                expected: Ok(Duration {
                    days: 8,
                    ..Default::default()
                }),
            },
            // 1H 15 M
            FormatAssertion {
                format: "1H 15 M".into(),
                input: "1H 15 M".into(),
                expected: Ok(Duration {
                    hours: 1,
                    minutes: 15,
                    ..Default::default()
                }),
            },
        ],
    }
}
