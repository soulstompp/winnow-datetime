use std::process::ExitCode;
use winnow::error::InputError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow_datetime_assert::Exception;
use winnow_datetime_assert::{define_format_tests, OffsetAssertion};
use winnow_iso8601::offset::offset;

use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;

define_format_tests!(
    winnow_iso8601,
    "tests/data/coverages",
    OffsetAssertion,
    winnow_datetime::Offset,
    OffsetCoverage,
    offset
);
