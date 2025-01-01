use crate::{FormatAssertion, FormatAssertionBuilder};
use winnow_datetime::Offset;

#[derive(Debug)]
pub struct OffsetAssertion {
    assertions: Vec<FormatAssertion<Option<Offset>>>,
}

impl FormatAssertionBuilder<Option<Offset>> for OffsetAssertion {
    fn base_assertions(&self) -> Vec<FormatAssertion<Option<Offset>>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Option<Offset>>> {
        self.base_assertions()
    }
}

pub fn assertions() -> OffsetAssertion {
    OffsetAssertion {
        assertions: vec![
            FormatAssertion {
                format: "%Z:%z".into(),
                input: "-08:00".into(),
                expected: Ok(Some(Offset {
                    offset_hours: -8,
                    offset_minutes: 0,
                })),
            },
            FormatAssertion {
                format: "Z".into(),
                input: "Z".into(),
                expected: Ok(Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0,
                })),
            },
            FormatAssertion {
                format: "z".into(),
                input: "z".into(),
                expected: Ok(Some(Offset {
                    offset_hours: 0,
                    offset_minutes: 0,
                })),
            },
            FormatAssertion {
                format: "-00:00".into(),
                input: "-00:00".into(),
                expected: Ok(None),
            },
            FormatAssertion {
                format: "%Z".into(),
                input: "-08".into(),
                expected: Ok(Some(Offset {
                    offset_hours: -8,
                    offset_minutes: 0,
                })),
            },
            FormatAssertion {
                format: "%Z%z".into(),
                input: "-0800".into(),
                expected: Ok(Some(Offset {
                    offset_hours: -8,
                    offset_minutes: 0,
                })),
            },
        ],
    }
}
