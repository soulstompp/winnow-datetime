use galvanic_test::test_suite;
use winnow_datetime_assert::generate_test_suite;

#[cfg(feature = "testing")]
use winnow_datetime_assert::FormatAssertion;

#[cfg(feature = "testing")]
generate_test_suite!(
    winnow_iso8601_interval,
    winnow_iso8601,
    interval,
    winnow_datetime::Interval,
    IntervalCoverage,
    interval
);
