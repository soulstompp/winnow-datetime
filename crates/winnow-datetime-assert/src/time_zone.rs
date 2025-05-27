use crate::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};
use serde::Deserialize;
use winnow_datetime::TimeZone;

#[derive(Debug, Deserialize)]
pub struct TimeZoneAssertion {
    assertions: Vec<FormatAssertion<TimeZone>>,
}

impl FormatAssertionBuilder<TimeZone> for TimeZoneAssertion {
    fn piece() -> &'static str {
        "time_zone"
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<TimeZone>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<TimeZone>> {
        self.base_assertions()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TimeZoneCoverage {
    pub coverage: Vec<FormatCoverage<TimeZone>>,
}

impl FormatCoverageBuilder<TimeZone> for TimeZoneCoverage {
    fn piece() -> &'static str {
        "time_zone"
    }
    fn base_coverage(&self) -> Vec<FormatCoverage<TimeZone>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<TimeZone>> {
        vec![]
    }
}
