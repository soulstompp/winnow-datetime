use std::process::ExitCode;
use winnow::error::InputError;
use winnow_datetime_assert::{define_format_tests, DateAssertion};
use winnow_iso8601::date::date;

use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;

define_format_tests!(
    winnow_iso8601,
    "tests/data/coverages",
    DateAssertion,
    winnow_datetime::Date,
    DateCoverage,
    date
);
