use winnow_datetime::Timezone;
use crate::{FormatAssertion, FormatAssertionBuilder};

#[derive(Debug)]
pub struct TimezoneAssertion {
    assertions: Vec<FormatAssertion<Timezone>>,
}

impl FormatAssertionBuilder<Timezone> for TimezoneAssertion {
    fn base_assertions(&self) -> Vec<FormatAssertion<Timezone>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Timezone>> {
        self.base_assertions()
    }
}

pub fn assertions() -> TimezoneAssertion {
    TimezoneAssertion {
        assertions: vec![
            FormatAssertion {
                format: "%Z:%z".into(),
                input: "-08:00".into(),
                expected: Ok(Timezone {
                    offset_hours: -8,
                    offset_minutes: 0,
                }),
            },
            FormatAssertion {
                format: "Z".into(),
                input: "Z".into(),
                expected: Ok(Timezone {
                    offset_hours: 0,
                    offset_minutes: 0,
                }),
            },
            FormatAssertion {
                format: "-00:00".into(),
                input: "-00:00".into(),
                expected: Ok(Timezone {
                        offset_hours: 0,
                        offset_minutes: 0,
                }),
            },
            FormatAssertion {
                format: "%Z".into(),
                input: "-08".into(),
                expected: Ok(Timezone {
                        offset_hours: -8,
                        offset_minutes: 0,
                }),
            },
            FormatAssertion {
                format: "%Z%z".into(),
                input: "-0800".into(),
                expected: Ok(Timezone {
                        offset_hours: -8,
                        offset_minutes: 0,
                }),
            },
        ]
    }
}
