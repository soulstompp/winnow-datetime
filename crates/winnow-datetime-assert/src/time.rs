use crate::offset::OffsetAssertion;
use crate::{
    FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder, OffsetCoverage,
};
use serde::Deserialize;
use winnow_datetime::Time;

#[derive(Debug, Deserialize)]
pub struct TimeAssertion {
    assertions: Vec<FormatAssertion<Time>>,
}

impl FormatAssertionBuilder<Time> for TimeAssertion {
    fn piece() -> &'static str {
        "time"
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<Time>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Time>> {
        let mut acc = vec![];

        let offset_assertions = OffsetAssertion::new();

        acc.append(&mut self.base_assertions());

        for t in self.base_assertions() {
            for tz in offset_assertions.assertions().iter() {
                let format = format!("{}{}", t.format, tz.format);
                let input = format!("{}{}", t.input, tz.input);

                let expected = match (t.expected.clone(), tz.expected.clone()) {
                    (t, Some(tz)) => t.set_tz(Some((tz.offset_hours, tz.offset_minutes))),
                    (t, None) => t.set_tz(None),
                };

                acc.push(FormatAssertion {
                    format,
                    input,
                    expected,
                });
            }
        }

        acc
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TimeCoverage {
    pub coverage: Vec<FormatCoverage<Time>>,
    pub separators: Vec<Option<String>>,
}

impl FormatCoverageBuilder<Time> for TimeCoverage {
    fn piece() -> &'static str {
        "time"
    }
    fn base_coverage(&self) -> Vec<FormatCoverage<Time>> {
        self.coverage
            .clone()
            .iter()
            .filter(|c| c.complete)
            .cloned()
            .collect()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Time>> {
        let mut acc = vec![];

        let offset_coverage = OffsetCoverage::new();

        for s in self.separators.iter() {
            for t in self.coverage.clone() {
                for offset in offset_coverage.coverage.iter() {
                    let format = format!(
                        "{}{}{}",
                        t.format,
                        s.clone().unwrap_or("".into()),
                        offset.format
                    );
                    let exception = match (t.exception.clone(), offset.exception.clone()) {
                        (None, None) => None,
                        (Some(t), Some(o)) => match o {
                            Some(o) => Some(t.set_tz(Some((o.offset_hours, o.offset_minutes)))),
                            None => Some(t.set_tz(None)),
                        },
                        _ => panic!("Invalid exception combination"),
                    };

                    acc.push(FormatCoverage {
                        format,
                        exception,
                        complete: true,
                    });
                }
            }
        }

        acc
    }
}
