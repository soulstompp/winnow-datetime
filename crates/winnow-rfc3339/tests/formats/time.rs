use std::process::ExitCode;
use winnow_datetime_assert::{define_format_tests, TimeAssertion};
use winnow_rfc3339::time::time;

use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;

define_format_tests!(
    winnow_rfc3339,
    "tests/data/coverages",
    TimeAssertion,
    winnow_datetime::Time,
    TimeCoverage,
    time
);
