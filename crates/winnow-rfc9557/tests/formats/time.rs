use std::process::ExitCode;
use winnow::error::InputError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow_datetime_assert::Exception;
use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;
use winnow_datetime_assert::{define_format_tests, TimeAssertion};
use winnow_rfc9557::time::time;

define_format_tests!(
    winnow_rfc9557,
    "tests/data/coverages",
    TimeAssertion,
    winnow_datetime::Time,
    TimeCoverage,
    time
);
