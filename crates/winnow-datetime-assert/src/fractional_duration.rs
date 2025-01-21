use crate::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};
use serde::Deserialize;
use winnow_datetime::FractionalDuration;

#[derive(Debug, serde::Deserialize)]
pub struct FractionalDurationAssertion {
    assertions: Vec<FormatAssertion<FractionalDuration>>,
}

impl FormatAssertionBuilder<FractionalDuration> for FractionalDurationAssertion {
    fn piece() -> &'static str {
        "fractional_duration"
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<FractionalDuration>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<FractionalDuration>> {
        self.base_assertions()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct FractionalDurationCoverage {
    pub coverage: Vec<FormatCoverage<FractionalDuration>>,
}

impl FormatCoverageBuilder<FractionalDuration> for FractionalDurationCoverage {
    fn piece() -> &'static str {
        "fractional_duration"
    }

    fn base_coverage(&self) -> Vec<FormatCoverage<FractionalDuration>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<FractionalDuration>> {
        vec![]
    }
}
