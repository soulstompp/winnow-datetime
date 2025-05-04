use serde::Deserialize;
use std::path::PathBuf;

mod clippy;

pub mod date;
pub use date::{DateAssertion, DateCoverage};

pub mod duration;
pub use duration::{DurationAssertion, DurationCoverage};

pub mod interval;
pub use interval::{IntervalAssertion, IntervalCoverage};

pub mod fractional_duration;
pub use fractional_duration::{FractionalDurationAssertion, FractionalDurationCoverage};

pub mod offset;
pub use offset::{OffsetAssertion, OffsetCoverage};

pub mod time;
pub use time::{TimeAssertion, TimeCoverage};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct FormatAssertion<T> {
    pub format: String,
    pub input: String,
    pub expected: T,
}

pub trait FormatAssertionBuilder<T> {
    fn new() -> Self
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        serde_yaml::from_reader(std::fs::File::open(Self::path()).unwrap()).unwrap()
    }

    fn piece() -> &'static str;

    fn path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Path to the crate's `Cargo.toml`
        path.push("data/assertions"); // Adjust the relative path to t
        path.join(Self::piece()).with_extension("yaml")
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<T>>;
    fn assertions(&self) -> Vec<FormatAssertion<T>>;
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct FormatCoverage<T> {
    pub format: String,
    pub exception: Option<T>,
    pub complete: bool,
}

pub trait FormatCoverageBuilder<T> {
    fn new() -> Self
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        serde_yaml::from_reader(std::fs::File::open(Self::path()).unwrap()).unwrap()
    }

    fn piece() -> &'static str;

    fn path() -> PathBuf {
        let data_dir = std::env::var("COVERAGE_PATH")
            .expect("COVERAGE_PATH should be set by test script calling it.");
        PathBuf::from(data_dir)
            .join(Self::piece())
            .with_extension("yaml")
    }

    fn base_coverage(&self) -> Vec<FormatCoverage<T>>;
    fn coverage(&self) -> Vec<FormatCoverage<T>>;
}

#[macro_export]
macro_rules! define_format_tests {
    ($format:ident, $coverage_path: expr, $assertion_type: ident, $piece_type:path, $coverage:ident, $parser:ident) => {
        fn main() -> ExitCode {
            use std::env;

            use libtest_mimic::{Arguments, Failed, Trial};
            use winnow::combinator::{eof, terminated};
            use winnow::Parser;
            use winnow_datetime_assert::$coverage;
            use winnow_datetime_assert::{FormatAssertion, FormatCoverage};

            if env::var("COVERAGE_PATH").is_err() {
                env::set_var("COVERAGE_PATH", $coverage_path);
            }

            let format_assertions = $assertion_type::new();
            let format_coverage = $coverage::new();

            let assertions = format_assertions
                .base_assertions()
                .into_iter()
                .chain(format_assertions.assertions().into_iter())
                .collect::<Vec<_>>();

            let coverages = format_coverage
                .base_coverage()
                .into_iter()
                .chain(format_coverage.coverage().into_iter())
                .collect::<Vec<_>>();

            let args = Arguments::from_args();
            let mut trials = Vec::new();

            let (covered, uncovered) =
                assertions
                    .into_iter()
                    .fold((vec![], vec![]), |(mut covered, mut uncovered), a| {
                        if let Some(c) = coverages.iter().find(|f| f.format == a.format) {
                            if let Some(e) = c.exception {
                                let mut a = a.clone();
                                a.expected = e;
                                covered.push(a);
                            } else {
                                covered.push(a);
                            }
                        } else {
                            uncovered.push(a);
                        }

                        (covered, uncovered)
                    });

            fn parse_input<'i, Input>(input: &mut Input) -> Result<$piece_type, String>
            where
                Input: StreamIsPartial
                    + Stream
                    + Compare<&'i str>
                    + AsBStr
                    + Clone
                    + std::fmt::Display,
                <Input as Stream>::Slice: AsBStr,
                <Input as Stream>::Token: AsChar + Clone,
            {
                let o = $parser::<Input, InputError<Input>>(input).map_err(|e| {
                    format!(
                        "Failed to parse datetime: {}: {}",
                        String::from_utf8_lossy(input.as_bstr()),
                        e.to_string()
                    )
                })?;
                let _ = eof::<Input, InputError<Input>>(input).map_err(|e| {
                    format!(
                        "Remaining input parsing datetime: {}: {}",
                        String::from_utf8_lossy(input.as_bstr()),
                        e.to_string()
                    )
                })?;

                Ok(o)
            }

            // Generate a trial for each assertion
            for assertion in covered {
                let name = format!("parses - {}", assertion.format);
                trials.push(Trial::test(name, move || {
                    // Parse the input
                    let result = parse_input(&mut assertion.input.as_str());

                    // If covered, the result must match the expected value
                    if result != Ok(assertion.expected) {
                        return Err(Failed::from(format!(
                            "Covered format mismatch: {}\nExpected: {:?}\nGot: {:?}",
                            assertion.format, assertion.expected, result
                        )));
                    }

                    Ok(())
                }));
            }

            for assertion in uncovered {
                let name = format!("rejects - {}", assertion.format);
                trials.push(Trial::test(name, move || {
                    // Parse the input
                    let result = parse_input(&mut assertion.input.as_str());

                    // If not covered, the result must be an error
                    if result.is_ok() {
                        return Err(Failed::from(format!(
                            "Uncovered format should not parse: {}\nInput: {}",
                            assertion.format, assertion.input
                        )));
                    }

                    Ok(())
                }));
            }

            // Run the trials and return the exit code
            libtest_mimic::run(&args, trials).exit_code()
        }
    };
}
