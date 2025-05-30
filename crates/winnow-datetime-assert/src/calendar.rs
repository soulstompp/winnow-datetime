use crate::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};
use serde::Deserialize;
use winnow_datetime::types::Calendar;

#[derive(Debug, Deserialize)]
pub struct CalendarAssertion {
    assertions: Vec<FormatAssertion<Calendar>>,
}

impl FormatAssertionBuilder<Calendar> for CalendarAssertion {
    fn piece() -> &'static str {
        "calendar"
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<Calendar>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Calendar>> {
        self.base_assertions()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CalendarCoverage {
    pub coverage: Vec<FormatCoverage<Calendar>>,
}

impl FormatCoverageBuilder<Calendar> for CalendarCoverage {
    fn piece() -> &'static str {
        "calendar"
    }
    fn base_coverage(&self) -> Vec<FormatCoverage<Calendar>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Calendar>> {
        vec![]
    }
}
