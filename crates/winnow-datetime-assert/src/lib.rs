use winnow_datetime::{Date, Time, Timezone};

pub mod date;
pub mod time;
pub mod timezone;

use winnow::error::{ContextError, ErrMode};
#[derive(Clone, Debug, PartialEq)]
pub struct FormatAssertion<T> {
    pub format: String,
    pub input: String,
    pub expected: Result<T, ErrMode<ContextError>>,
}

pub trait FormatAssertionBuilder<T> {
    fn base_assertions(&self) -> Vec<FormatAssertion<T>>;
    fn assertions(&self) -> Vec<FormatAssertion<T>>;
}



#[derive(Clone, Debug, PartialEq)]
pub struct FormatCoverage<T> {
    pub format: String,
    pub exception: Result<Option<T>, ErrMode<ContextError>>,
}

pub trait FormatCoverageBuilder<T> {
    fn base_coverage(&self) -> Vec<FormatCoverage<T>>;
    fn coverage(&self) -> Vec<FormatCoverage<T>>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct DateCoverage {
    pub coverage: Vec<FormatCoverage<Date>>,
}

impl FormatCoverageBuilder<Date> for DateCoverage {
    fn base_coverage(&self) -> Vec<FormatCoverage<Date>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Date>> {
        self.base_coverage()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeCoverage {
    pub coverage: Vec<FormatCoverage<Time>>,
    pub separators: Vec<Option<&'static str>>,
    pub timezone_coverage: TimezoneCoverage
}

impl FormatCoverageBuilder<Time> for TimeCoverage {
    fn base_coverage(&self) -> Vec<FormatCoverage<Time>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Time>> {
        let mut acc = vec![];

        acc.append(&mut self.base_coverage());

        for s in self.separators.iter() {
            for t in self.base_coverage() {
                for tz in self.timezone_coverage.coverage.iter() {
                    let format = format!("{}{}{}", t.format, s.unwrap_or(""), tz.format);
                    let exception = match (t.exception.clone(), tz.exception.clone()) {
                        (Ok(None), Ok(None)) => Ok(None),
                        (Ok(Some(t)), Ok(Some(tz))) => Ok(Some(t.set_tz((tz.offset_hours, tz.offset_minutes)))),
                        (Err(e), _) => Err(e),
                        (_, Err(e)) => Err(e),
                        _ => panic!("Invalid exception combination")
                    };

                    acc.push(FormatCoverage {
                        format,
                        exception
                    });
                }
            }
        }

        acc
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimezoneCoverage {
    pub coverage: Vec<FormatCoverage<Timezone>>,
}

impl FormatCoverageBuilder<Timezone> for TimezoneCoverage {
    fn base_coverage(&self) -> Vec<FormatCoverage<Timezone>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Timezone>> {
        self.base_coverage().iter().map(|timezone| {
            FormatCoverage {
                format: timezone.format.clone(),
                exception: Ok(None)
            }
        }).collect()
    }
}

#[macro_export]
macro_rules! generate_test_suite {
    ($suite_name: ident, $format: ident, $piece: ident, $piece_type:ident, $coverage: ident, $parser: ident) => {
        test_suite! {
            name $suite_name;

            use winnow_datetime_assert::$piece::assertions;
            use winnow_datetime_assert::$coverage;
            use winnow_datetime_assert::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};

            fixture format_checks(assertion: crate::FormatAssertion<winnow_datetime::$piece_type>) -> crate::FormatAssertion<winnow_datetime::$piece_type> {
                params {
                    assertions().assertions().into_iter()
                }
                setup(&mut self) {
                    self.assertion.clone()
                }
            }

            // Iterate through the formats and create tests for each
            test format_assertions(format_checks) {
                let mut input = format_checks.params.assertion.input.clone();

                if $format::$piece::coverage().coverage().iter().any(|c| c.format == format_checks.params.assertion.format) {
                    let result = $format::parsers::$parser(&mut input.as_bytes());
                    assert_eq!(result, format_checks.params.assertion.expected);
                }
                else {
                    let result = $format::parsers::$parser(&mut input.as_bytes());
                    assert!(result.is_err(), "Uncovered format {} should not parse", format_checks.params.assertion.format);
                }

            }
        }
   }
}
