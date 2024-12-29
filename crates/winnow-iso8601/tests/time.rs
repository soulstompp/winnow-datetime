use winnow_datetime_assert::generate_test_suite;
use galvanic_test::test_suite;

#[cfg(feature = "testing")]
use winnow_datetime_assert::FormatAssertion;

#[cfg(feature = "testing")]
generate_test_suite!(winnow_iso8601_time, winnow_iso8601, time, Time, TimeCoverage, parse_time);
