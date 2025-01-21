use crate::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};
use serde::Deserialize;
use winnow_datetime::Offset;

#[derive(Debug, Deserialize)]
pub struct OffsetAssertion {
    assertions: Vec<FormatAssertion<Option<Offset>>>,
}

impl FormatAssertionBuilder<Option<Offset>> for OffsetAssertion {
    fn piece() -> &'static str {
        "offset"
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<Option<Offset>>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Option<Offset>>> {
        self.base_assertions()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OffsetCoverage {
    pub coverage: Vec<FormatCoverage<Option<Offset>>>,
}

impl FormatCoverageBuilder<Option<Offset>> for OffsetCoverage {
    fn piece() -> &'static str {
        "offset"
    }
    fn base_coverage(&self) -> Vec<FormatCoverage<Option<Offset>>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Option<Offset>>> {
        vec![]
    }
}
