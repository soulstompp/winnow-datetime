use std::process::ExitCode;
use winnow::error::InputError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow_datetime_assert::calendar::CalendarAssertion;
use winnow_datetime_assert::define_format_tests;
use winnow_datetime_assert::Exception;
use winnow_datetime_assert::FormatAssertionBuilder;
use winnow_datetime_assert::FormatCoverageBuilder;
use winnow_rfc9557::calendar::calendar;

define_format_tests!(
    winnow_rfc9557,
    "tests/data/coverages",
    CalendarAssertion,
    winnow_datetime::Calendar,
    CalendarCoverage,
    calendar
);
