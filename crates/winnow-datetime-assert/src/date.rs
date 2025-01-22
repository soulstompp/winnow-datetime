use crate::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};
use serde::Deserialize;
use winnow_datetime::Date;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub struct DateAssertion {
    assertions: Vec<FormatAssertion<Date>>,
}

impl FormatAssertionBuilder<Date> for DateAssertion {
    fn piece() -> &'static str {
        "date"
    }
    fn base_assertions(&self) -> Vec<FormatAssertion<Date>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Date>> {
        self.base_assertions()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DateCoverage {
    pub coverage: Vec<FormatCoverage<Date>>,
}

impl FormatCoverageBuilder<Date> for DateCoverage {
    fn piece() -> &'static str {
        "date"
    }

    fn base_coverage(&self) -> Vec<FormatCoverage<Date>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Date>> {
        vec![]
    }
}
