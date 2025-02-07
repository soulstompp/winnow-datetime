use std::process::ExitCode;
use winnow_datetime_assert::{define_format_tests, OffsetAssertion};
use winnow_rfc3339::offset::offset;

use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;

define_format_tests!(
    winnow_rfc3339,
    "tests/data/coverages",
    OffsetAssertion,
    Option<winnow_datetime::Offset>,
    OffsetCoverage,
    offset
);
