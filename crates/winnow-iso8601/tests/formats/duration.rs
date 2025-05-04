use std::process::ExitCode;
use winnow::error::InputError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;
use winnow_datetime_assert::{define_format_tests, DurationAssertion};
use winnow_iso8601::duration::duration;

//TODO: forgot fractional duration!
define_format_tests!(
    winnow_iso8601,
    "tests/data/coverages",
    DurationAssertion,
    winnow_datetime::Duration,
    DurationCoverage,
    duration
);
