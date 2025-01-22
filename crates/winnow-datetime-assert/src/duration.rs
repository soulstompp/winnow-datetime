use crate::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};
use serde::Deserialize;
use winnow_datetime::Duration;

#[derive(Debug, Deserialize)]
pub struct DurationAssertion {
    assertions: Vec<FormatAssertion<Duration>>,
}

impl FormatAssertionBuilder<Duration> for DurationAssertion {
    fn piece() -> &'static str {
        "duration"
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<Duration>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Duration>> {
        self.base_assertions()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DurationCoverage {
    pub coverage: Vec<FormatCoverage<Duration>>,
}

impl FormatCoverageBuilder<Duration> for crate::DurationCoverage {
    fn piece() -> &'static str {
        "duration"
    }

    fn base_coverage(&self) -> Vec<FormatCoverage<Duration>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Duration>> {
        vec![]
    }
}
