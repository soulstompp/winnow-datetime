use std::process::ExitCode;
use winnow_datetime_assert::{define_format_tests, IntervalAssertion};

use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;

define_format_tests!(
    winnow_iso8601,
    "tests/data/coverages",
    IntervalAssertion,
    winnow_datetime::Interval,
    IntervalCoverage,
    interval
);
