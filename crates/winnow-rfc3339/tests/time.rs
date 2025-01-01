use galvanic_test::test_suite;
use winnow_datetime_assert::generate_test_suite;

#[cfg(feature = "testing")]
use winnow_datetime_assert::FormatAssertion;

#[cfg(feature = "testing")]
generate_test_suite!(
    winnow_rfc3339_time,
    winnow_rfc3339,
    time,
    winnow_datetime::Time,
    TimeCoverage,
    time
);
