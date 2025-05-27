use std::process::ExitCode;
use winnow::error::InputError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow_datetime_assert::{define_format_tests, DateAssertion};
use winnow_rfc3339::date::date;

use winnow_datetime_assert::Exception;
use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;

define_format_tests!(
    winnow_rfc3339,
    "tests/data/coverages",
    DateAssertion,
    winnow_datetime::Date,
    DateCoverage,
    date
);
