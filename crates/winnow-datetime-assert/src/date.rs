use crate::{FormatAssertion, FormatAssertionBuilder};
use winnow_datetime::Date;

pub struct DateAssertion {
    assertions: Vec<FormatAssertion<Date>>,
}

impl FormatAssertionBuilder<Date> for DateAssertion {
    fn base_assertions(&self) -> Vec<FormatAssertion<Date>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Date>> {
        self.base_assertions()
    }
}

pub fn assertions() -> DateAssertion {
    DateAssertion {
        assertions: vec![
            FormatAssertion {
                format: "%Y-%M-%D".into(),
                input: "2024-12-22".into(),
                expected: Ok(Date::YMD {
                    year: 2024,
                    month: 12,
                    day: 22,
                }),
            },
            FormatAssertion {
                format: "%Y-%M".into(),
                input: "2024-12".into(),
                expected: Ok(Date::YMD {
                    year: 2024,
                    month: 12,
                    day: 1,
                }),
            },
            FormatAssertion {
                format: "%Y-%O".into(),
                input: "2024-357".into(),
                expected: Ok(Date::Ordinal {
                    year: 2024,
                    day: 357,
                }),
            },
            FormatAssertion {
                format: "%V-W%W".into(),
                input: "2024-W51".into(),
                expected: Ok(Date::Week {
                    year: 2024,
                    week: 51,
                    day: 1,
                }),
            },
            FormatAssertion {
                format: "%V-W%W-%w".into(),
                input: "2024-W51-7".into(),
                expected: Ok(Date::Week {
                    year: 2024,
                    week: 51,
                    day: 7,
                }),
            },
            FormatAssertion {
                format: "%Y%M%D".into(),
                input: "20241222".into(),
                expected: Ok(Date::YMD {
                    year: 2024,
                    month: 12,
                    day: 22,
                }),
            },
            FormatAssertion {
                format: "%Y%O".into(),
                input: "2024357".into(),
                expected: Ok(Date::Ordinal {
                    year: 2024,
                    day: 357,
                }),
            },
            FormatAssertion {
                format: "%VW%W".into(),
                input: "2024W51".into(),
                expected: Ok(Date::Week {
                    year: 2024,
                    week: 51,
                    day: 1,
                }),
            },
            FormatAssertion {
                format: "%VW%W%w".into(),
                input: "2024W517".into(),
                expected: Ok(Date::Week {
                    year: 2024,
                    week: 51,
                    day: 7,
                }),
            },
            FormatAssertion {
                format: "--%M-%D".into(),
                input: "--12-22".into(),
                expected: Ok(Date::YMD {
                    year: 0,
                    month: 12,
                    day: 22,
                }),
            },
            FormatAssertion {
                format: "%M-%D".into(),
                input: "12-22".into(),
                expected: Ok(Date::YMD {
                    year: 0,
                    month: 12,
                    day: 22,
                }),
            },
        ],
    }
}
