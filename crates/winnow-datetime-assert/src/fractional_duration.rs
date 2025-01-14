use crate::{FormatAssertion, FormatAssertionBuilder};
use winnow_datetime::FractionalDuration;

pub struct FractionalDurationAssertion {
    assertions: Vec<FormatAssertion<FractionalDuration>>,
}

impl FormatAssertionBuilder<FractionalDuration> for FractionalDurationAssertion {
    fn base_assertions(&self) -> Vec<FormatAssertion<FractionalDuration>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<FractionalDuration>> {
        self.base_assertions()
    }
}

pub fn assertions() -> FractionalDurationAssertion {
    FractionalDurationAssertion {
        assertions: vec![
            // Format
            // P1Y
            FormatAssertion {
                format: "P1Y".into(),
                input: "P1Y".into(),
                expected: Ok(FractionalDuration {
                    years: (1, None),
                    ..Default::default()
                }),
            },
            // P1,5Y
            FormatAssertion {
                format: "P1,5Y".into(),
                input: "P1,5Y".into(),
                expected: Ok(FractionalDuration {
                    years: (1, None),
                    months: (6, None),
                    ..Default::default()
                }),
            },
            // P1.5Y
            FormatAssertion {
                format: "P1.5Y".into(),
                input: "P1.5Y".into(),
                expected: Ok(FractionalDuration {
                    years: (1, None),
                    months: (6, None),
                    ..Default::default()
                }),
            },
            // P1M
            FormatAssertion {
                format: "P1M".into(),
                input: "P1M".into(),
                expected: Ok(FractionalDuration {
                    months: (1, None),
                    ..Default::default()
                }),
            },
            // P1W
            FormatAssertion {
                format: "P1W".into(),
                input: "P1W".into(),
                expected: Ok(FractionalDuration {
                    weeks: (1, None),
                    ..Default::default()
                }),
            },
            // P1D
            FormatAssertion {
                format: "P1D".into(),
                input: "P1D".into(),
                expected: Ok(FractionalDuration {
                    days: (1, None),
                    ..Default::default()
                }),
            },
            // PT1H
            FormatAssertion {
                format: "PT1H".into(),
                input: "PT1H".into(),
                expected: Ok(FractionalDuration {
                    hours: (1, None),
                    ..Default::default()
                }),
            },
            // P1H
            FormatAssertion {
                format: "P1H".into(),
                input: "P1H".into(),
                expected: Ok(FractionalDuration {
                    hours: (1, None),
                    ..Default::default()
                }),
            },
            // PT1M
            FormatAssertion {
                format: "PT1M".into(),
                input: "PT1M".into(),
                expected: Ok(FractionalDuration {
                    minutes: (1, None),
                    ..Default::default()
                }),
            },
            // PT1S
            FormatAssertion {
                format: "PT1S".into(),
                input: "PT1S".into(),
                expected: Ok(FractionalDuration {
                    seconds: (1, None),
                    ..Default::default()
                }),
            },
            // P1S
            FormatAssertion {
                format: "P1S".into(),
                input: "P1S".into(),
                expected: Ok(FractionalDuration {
                    seconds: (1, None),
                    ..Default::default()
                }),
            },
            // PT1,5S
            FormatAssertion {
                format: "PT1,5S".into(),
                input: "PT1,5S".into(),
                expected: Ok(FractionalDuration {
                    seconds: (1, Some(0.5)),
                    ..Default::default()
                }),
            },
            // PT1.5S
            FormatAssertion {
                format: "PT1.5S".into(),
                input: "PT1.5S".into(),
                expected: Ok(FractionalDuration {
                    seconds: (1, Some(0.5)),
                    ..Default::default()
                }),
            },
            // P1Y1M
            FormatAssertion {
                format: "P1Y1M".into(),
                input: "P1Y1M".into(),
                expected: Ok(FractionalDuration {
                    years: (1, None),
                    months: (1, None),
                    ..Default::default()
                }),
            },
            // P1Y1D
            FormatAssertion {
                format: "P1Y1D".into(),
                input: "P1Y1D".into(),
                expected: Ok(FractionalDuration {
                    years: (1, None),
                    days: (1, None),
                    ..Default::default()
                }),
            },
            // P1Y1M1D
            FormatAssertion {
                format: "P1Y1M1D".into(),
                input: "P1Y1M1D".into(),
                expected: Ok(FractionalDuration {
                    years: (1, None),
                    months: (1, None),
                    days: (1, None),
                    ..Default::default()
                }),
            },
            // P1Y1M1DT1H1M1S
            FormatAssertion {
                format: "P1Y1M1DT1H1M1S".into(),
                input: "P1Y1M1DT1H1M1S".into(),
                expected: Ok(FractionalDuration {
                    years: (1, None),
                    months: (1, None),
                    days: (1, None),
                    hours: (1, None),
                    minutes: (1, None),
                    seconds: (1, None),
                    ..Default::default()
                }),
            },
            // P1DT1H
            FormatAssertion {
                format: "P1DT1H".into(),
                input: "P1DT1H".into(),
                expected: Ok(FractionalDuration {
                    days: (1, None),
                    hours: (1, None),
                    ..Default::default()
                }),
            },
            // P1MT1M
            FormatAssertion {
                format: "P1MT1M".into(),
                input: "P1MT1M".into(),
                expected: Ok(FractionalDuration {
                    months: (1, None),
                    minutes: (1, None),
                    ..Default::default()
                }),
            },
            // P1DT1M
            FormatAssertion {
                format: "P1DT1M".into(),
                input: "P1DT1M".into(),
                expected: Ok(FractionalDuration {
                    days: (1, None),
                    minutes: (1, None),
                    ..Default::default()
                }),
            },
            // P1.5W
            FormatAssertion {
                format: "P1.5W".into(),
                input: "P1.5W".into(),
                expected: Ok(FractionalDuration {
                    weeks: (1, None),
                    days: (3, None),
                    hours: (12, None),
                    ..Default::default()
                }),
            },
            // P1,5W
            FormatAssertion {
                format: "P1,5W".into(),
                input: "P1,5W".into(),
                expected: Ok(FractionalDuration {
                    weeks: (1, None),
                    days: (3, None),
                    hours: (12, None),
                    ..Default::default()
                }),
            },
            // P1DT1.000S
            FormatAssertion {
                format: "P1DT1.000S".into(),
                input: "P1DT1.000S".into(),
                expected: Ok(FractionalDuration {
                    days: (1, None),
                    seconds: (1, None),
                    ..Default::default()
                }),
            },
            // P1DT1.00000S
            FormatAssertion {
                format: "P1DT1.00000S".into(),
                input: "P1DT1.00000S".into(),
                expected: Ok(FractionalDuration {
                    days: (1, None),
                    seconds: (1, None),
                    ..Default::default()
                }),
            },
            // P1DT1H1M1.1S
            FormatAssertion {
                format: "P1DT1H1M1.1S".into(),
                input: "P1DT1H1M1.1S".into(),
                expected: Ok(FractionalDuration {
                    days: (1, None),
                    hours: (1, None),
                    minutes: (1, None),
                    seconds: (1, Some(0.1)),
                    ..Default::default()
                }),
            },
            // P1H1M1.1S
            FormatAssertion {
                format: "P1H1M1.1S".into(),
                input: "P1H1M1.1S".into(),
                expected: Ok(FractionalDuration {
                    hours: (1, None),
                    minutes: (1, None),
                    seconds: (1, Some(0.1)),
                    ..Default::default()
                }),
            },
            // 1W1M1S
            FormatAssertion {
                format: "1W1M1S".into(),
                input: "1W1M1S".into(),
                expected: Ok(FractionalDuration {
                    days: (7, None),
                    minutes: (1, None),
                    seconds: (1, None),
                    ..Default::default()
                }),
            },
            // 1S1M1H1W
            FormatAssertion {
                format: "1S1M1H1W".into(),
                input: "1S1M1H1W".into(),
                expected: Ok(FractionalDuration {
                    days: (7, None),
                    hours: (1, None),
                    minutes: (1, None),
                    seconds: (1, None),
                    ..Default::default()
                }),
            },
            // 1 W
            FormatAssertion {
                format: "1 W".into(),
                input: "1 W".into(),
                expected: Ok(FractionalDuration {
                    days: (7, None),
                    ..Default::default()
                }),
            },
            // 1.5W
            FormatAssertion {
                format: "1.5W".into(),
                input: "1.5W".into(),
                expected: Ok(FractionalDuration {
                    days: (10, None),
                    ..Default::default()
                }),
            },
            // 1 D 1 W
            FormatAssertion {
                format: "1 D 1 W".into(),
                input: "1 D 1 W".into(),
                expected: Ok(FractionalDuration {
                    days: (8, None),
                    ..Default::default()
                }),
            },
            // 1.5 S 1.5 M
            FormatAssertion {
                format: "1.5 S 1.5 M".into(),
                input: "1.5 S 1.5 M".into(),
                expected: Ok(FractionalDuration {
                    minutes: (1, None),
                    seconds: (1, None),
                    ..Default::default()
                }),
            },
            // 1H 15 M
            FormatAssertion {
                format: "1H 15 M".into(),
                input: "1H 15 M".into(),
                expected: Ok(FractionalDuration {
                    hours: (1, None),
                    minutes: (15, None),
                    ..Default::default()
                }),
            },
        ],
    }
}
