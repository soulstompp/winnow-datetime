use std::process::ExitCode;
use winnow::error::InputError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow_datetime_assert::{define_format_tests, TimeZoneAssertion};

use winnow_datetime_assert::Exception;
use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;
use winnow_rfc9557::time_zone::time_zone;

define_format_tests!(
    winnow_rfc9557,
    "tests/data/coverages",
    TimeZoneAssertion,
    winnow_datetime::TimeZone,
    TimeZoneCoverage,
    time_zone
);
