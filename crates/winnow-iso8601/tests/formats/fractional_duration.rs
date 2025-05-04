use std::process::ExitCode;
use winnow::error::InputError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;
use winnow_datetime_assert::{define_format_tests, FractionalDurationAssertion};
use winnow_iso8601::fractional_duration::fractional_duration;

define_format_tests!(
    winnow_iso8601,
    "tests/data/coverages",
    FractionalDurationAssertion,
    winnow_datetime::FractionalDuration,
    FractionalDurationCoverage,
    fractional_duration
);
