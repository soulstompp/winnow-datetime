use crate::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};
use serde::Deserialize;
use winnow_datetime::Interval;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub struct IntervalAssertion {
    assertions: Vec<FormatAssertion<Interval>>,
}

impl FormatAssertionBuilder<Interval> for IntervalAssertion {
    fn piece() -> &'static str {
        "interval"
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<Interval>> {
        vec![]
    }

    fn assertions(&self) -> Vec<FormatAssertion<Interval>> {
        self.assertions.clone()
    }
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct IntervalCoverage {
    pub coverage: Vec<FormatCoverage<Interval>>,
}

impl FormatCoverageBuilder<Interval> for crate::IntervalCoverage {
    fn piece() -> &'static str {
        "interval"
    }

    fn base_coverage(&self) -> Vec<FormatCoverage<Interval>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Interval>> {
        vec![]
    }
}
