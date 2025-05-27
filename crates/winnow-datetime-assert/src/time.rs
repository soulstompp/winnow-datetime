use crate::offset::OffsetAssertion;
use crate::Exception;
use crate::{
    FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder, OffsetCoverage,
};
use serde::Deserialize;
use winnow_datetime::{Offset, Time};

#[derive(Debug, Deserialize)]
pub struct TimeAssertion {
    assertions: Vec<FormatAssertion<Time>>,
}

impl FormatAssertionBuilder<Time> for TimeAssertion {
    fn piece() -> &'static str {
        "time"
    }

    fn base_assertions(&self) -> Vec<FormatAssertion<Time>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Time>> {
        let mut acc = vec![];

        let offset_assertions = OffsetAssertion::new();

        acc.append(&mut self.base_assertions());

        for t in self.base_assertions() {
            for o in offset_assertions.assertions().iter() {
                let format = format!("{}{}", t.format, o.format);
                let input = format!("{}{}", t.input, o.input);

                let expected = match (t.expected.clone(), o.expected.clone()) {
                    (mut t, o) => {
                        t.offset = Some(o);
                        t
                    }
                };

                acc.push(FormatAssertion {
                    format,
                    input,
                    expected,
                });
            }
        }

        acc
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TimeCoverage {
    pub coverage: Vec<FormatCoverage<Time>>,
    pub separators: Vec<Option<String>>,
}

impl FormatCoverageBuilder<Time> for TimeCoverage {
    fn piece() -> &'static str {
        "time"
    }
    fn base_coverage(&self) -> Vec<FormatCoverage<Time>> {
        self.coverage
            .clone()
            .iter()
            .filter(|c| c.complete)
            .cloned()
            .collect()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Time>> {
        let mut acc = vec![];

        let assertions = TimeAssertion::new().base_assertion_map();

        let offset_coverage = OffsetCoverage::new();

        for s in self.separators.iter() {
            for t in self.coverage.clone() {
                for offset in offset_coverage.coverage.iter() {
                    let format = format!(
                        "{}{}{}",
                        t.format.to_string(),
                        s.clone().unwrap_or("".into()),
                        offset.format
                    );
                    let exception = match (t.exception.clone(), offset.exception.clone()) {
                        (Exception::Unspecified, Exception::Unspecified) => Exception::Unspecified,
                        (Exception::Specific {value: mut t}, Exception::Specific { value: o }) => match o {
                            Offset::Fixed {
                                hours,
                                minutes,
                                critical: _,
                            } => {
                                t.offset = Some(Offset::Fixed {
                                    hours,
                                    minutes,
                                    critical: false,
                                });
                                Exception::Specific { value: t }
                            }
                            Offset::LocalUnknown { critical } => {
                                t.offset = Some(Offset::LocalUnknown { critical });
                                Exception::Specific { value: t }
                            }
                        },
                        (Exception::Specific { value: t }, Exception::Unspecified) => Exception::Specific { value: t },
                        (Exception::Unspecified, Exception::Specific { value: o }) => {
                            let mut default_t = assertions
                                .get(&t.format)
                                .expect(&format!("format not found: {}", &t.format))
                                .clone();

                            match o {
                                Offset::Fixed {
                                    hours,
                                    minutes,
                                    critical: _,
                                } => {
                                    default_t.offset = Some(Offset::Fixed {
                                        hours,
                                        minutes,
                                        critical: false,
                                    });
                                }
                                Offset::LocalUnknown { critical } => {
                                    default_t.offset = Some(Offset::LocalUnknown { critical });
                                }
                            }

                            Exception::Specific { value: default_t }
                        }
                    };

                    acc.push(FormatCoverage {
                        format,
                        exception,
                        complete: true,
                    });
                }
            }
        }

        acc
    }
}
