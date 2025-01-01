use galvanic_test::test_suite;
use winnow_datetime_assert::generate_test_suite;

#[cfg(feature = "testing")]
use winnow_datetime_assert::FormatAssertion;

#[cfg(feature = "testing")]
generate_test_suite!(
    winnow_iso8601_date,
    winnow_iso8601,
    date,
    winnow_datetime::Date,
    DateCoverage,
    date
);
