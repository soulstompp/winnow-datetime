use winnow_datetime::Time;
use crate::{FormatAssertion, FormatAssertionBuilder};
use crate::timezone::TimezoneAssertion;

#[derive(Debug)]
pub struct TimeAssertion {
    assertions: Vec<FormatAssertion<Time>>,
    timezone_assertions: TimezoneAssertion
}

impl FormatAssertionBuilder<Time> for TimeAssertion {
    fn base_assertions(&self) -> Vec<FormatAssertion<Time>> {
        self.assertions.clone()
    }

    fn assertions(&self) -> Vec<FormatAssertion<Time>> {
        let mut acc = vec![];

        acc.append(&mut self.base_assertions());

        for t in self.base_assertions() {
            for tz in self.timezone_assertions.assertions().iter() {
                let format = format!("{}{}", t.format, tz.format);
                let input = format!("{}{}", t.input, tz.input);

                let expected = match (t.expected.clone(), tz.expected.clone()) {
                    (Ok(t), Ok(tz)) => Ok(t.set_tz((tz.offset_hours, tz.offset_minutes))),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                };

                acc.push(FormatAssertion {
                    format,
                    input,
                    expected
                });
            }
        }

        acc
    }
}
pub fn assertions() -> TimeAssertion {
    TimeAssertion {
        assertions: vec![
            //%h:%m:%s                      | 07:42:55
            FormatAssertion {
                format: "%h:%m:%s".into(),
                input: "07:42:55".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //%h:%m:%.1s                    | 07:42:55.8
            FormatAssertion {
                format: "%h:%m:%.1s".into(),
                input: "07:42:55.8".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 800,
                    timezone: None,
                }),
            },
            //%h:%m:%.2s                    | 07:42:55.87
            FormatAssertion {
                format: "%h:%m:%.2s".into(),
                input: "07:42:55.87".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h:%m:%,3s                    | 07:42:55,870
            FormatAssertion {
                format: "%h:%m:%,3s".into(),
                input: "07:42:55,870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h:%m:%.3s                    | 07:42:55.870
            FormatAssertion {
                format: "%h:%m:%.3s".into(),
                input: "07:42:55.870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h:%m:%s,%u                   | 07:42:55,870479
            FormatAssertion {
                format: "%h:%m:%s,%u".into(),
                input: "07:42:55,870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h:%m:%s.%u                   | 07:42:55.870479
            FormatAssertion {
                format: "%h:%m:%s.%u".into(),
                input: "07:42:55.870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h                           | T07
            FormatAssertion {
                format: "T%h".into(),
                input: "T07".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 0,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //T%h:%m                        | T07:42
            FormatAssertion {
                format: "T%h:%m".into(),
                input: "T07:42".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //T%h:%,1m                      | T07:42,9
            FormatAssertion {
                format: "T%h:%,1m".into(),
                input: "T07:42,9".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 900,
                    timezone: None,
                }),
            },
            //T%h:%.1m                      | T07:42.9
            FormatAssertion {
                format: "T%h:%.1m".into(),
                input: "T07:42.9".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 900,
                    timezone: None,
                }),
            },
            //T%h:%m:%s                     | T07:42:55
            FormatAssertion {
                format: "T%h:%m:%s".into(),
                input: "T07:42:55".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //T%h:%m:%.1s                   | T07:42:55.8
            FormatAssertion {
                format: "T%h:%m:%.1s".into(),
                input: "T07:42:55.8".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 800,
                    timezone: None,
                }),
            },
            //T%h:%m:%.2s                   | T07:42:55.87
            FormatAssertion {
                format: "T%h:%m:%.2s".into(),
                input: "T07:42:55.87".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h:%m:%,3s                   | T07:42:55,870
            FormatAssertion {
                format: "T%h:%m:%,3s".into(),
                input: "T07:42:55,870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h:%m:%.3s                   | T07:42:55.870
            FormatAssertion {
                format: "T%h:%m:%.3s".into(),
                input: "T07:42:55.870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h:%m:%s,%u                  | T07:42:55,870479
            FormatAssertion {
                format: "T%h:%m:%s,%u".into(),
                input: "T07:42:55,870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h:%m:%s.%u                  | T07:42:55.870479
            FormatAssertion {
                format: "T%h:%m:%s.%u".into(),
                input: "T07:42:55.870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m                          | 0742
            FormatAssertion {
                format: "%h%m".into(),
                input: "0742".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //%h%m%s                        | 074255
            FormatAssertion {
                format: "%h%m%s".into(),
                input: "074255".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //%h%m%.1s                      | 074255.8
            FormatAssertion {
                format: "%h%m%.1s".into(),
                input: "074255.8".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 800,
                    timezone: None,
                }),
            },
            //%h%m%.2s                      | 074255.87
            FormatAssertion {
                format: "%h%m%.2s".into(),
                input: "074255.87".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m%,3s                      | 074255,870
            FormatAssertion {
                format: "%h%m%,3s".into(),
                input: "074255,870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m%.3s                      | 074255.870
            FormatAssertion {
                format: "%h%m%.3s".into(),
                input: "074255.870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m%s,%u                     | 074255,870479
            FormatAssertion {
                format: "%h%m%s,%u".into(),
                input: "074255,870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m%s.%u                     | 074255.870479
            FormatAssertion {
                format: "%h%m%s.%u".into(),
                input: "074255.870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h%m                         | T0742
            FormatAssertion {
                format: "T%h%m".into(),
                input: "T0742".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //T%h%,1m                       | T0742,9
            FormatAssertion {
                format: "T%h%,1m".into(),
                input: "T0742,9".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 900,
                    timezone: None,
                }),
            },
            //T%h%.1m                       | T0742.9
            FormatAssertion {
                format: "T%h%.1m".into(),
                input: "T0742.9".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 900,
                    timezone: None,
                }),
            },
            //T%h%m%s                       | T074255
            FormatAssertion {
                format: "T%h%m%s".into(),
                input: "T074255".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //T%h%m%.1s                     | T074255.8
            FormatAssertion {
                format: "T%h%m%.1s".into(),
                input: "T074255.8".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 800,
                    timezone: None,
                }),
            },
            //T%h%m%.2s                     | T074255.87
            FormatAssertion {
                format: "T%h%m%.2s".into(),
                input: "T074255.87".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h%m%,3s                     | T074255,870
            FormatAssertion {
                format: "T%h%m%,3s".into(),
                input: "T074255,870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h%m%.3s                     | T074255.870
            FormatAssertion {
                format: "T%h%m%.3s".into(),
                input: "T074255.870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h%m%s,%u                    | T074255,870479
            FormatAssertion {
                format: "T%h%m%s,%u".into(),
                input: "T074255,870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h%m%s.%u                    | T074255.870479
            FormatAssertion {
                format: "T%h%m%s.%u".into(),
                input: "T074255.870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h                            | 07
            FormatAssertion {
                format: "%h".into(),
                input: "07".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 0,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //%h:%m                         | 07:42
            FormatAssertion {
                format: "%h:%m".into(),
                input: "07:42".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //%h:%.1m                       | 07:42.9
            FormatAssertion {
                format: "%h:%.1m".into(),
                input: "07:42.9".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 900,
                    timezone: None,
                }),
            },
            //%h:%m:%s                      | 07:42:55
            FormatAssertion {
                format: "%h:%m:%s".into(),
                input: "07:42:55".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //%h:%m:%.1s                    | 07:42:55.8
            FormatAssertion {
                format: "%h:%m:%.1s".into(),
                input: "07:42:55.8".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 800,
                    timezone: None,
                }),
            },
            //%h:%m:%.2s                    | 07:42:55.87
            FormatAssertion {
                format: "%h:%m:%.2s".into(),
                input: "07:42:55.87".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h:%m:%,3s                    | 07:42:55,870
            FormatAssertion {
                format: "%h:%m:%,3s".into(),
                input: "07:42:55,870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h:%m:%.3s                    | 07:42:55.870
            FormatAssertion {
                format: "%h:%m:%.3s".into(),
                input: "07:42:55.870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h:%m:%s,%u                   | 07:42:55,870479
            FormatAssertion {
                format: "%h:%m:%s,%u".into(),
                input: "07:42:55,870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h:%m:%s.%u                   | 07:42:55.870479
            FormatAssertion {
                format: "%h:%m:%s.%u".into(),
                input: "07:42:55.870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h                           | T07
            FormatAssertion {
                format: "T%h".into(),
                input: "T07".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 0,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //T%h:%m                        | T07:42
            FormatAssertion {
                format: "T%h:%m".into(),
                input: "T07:42".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //T%h:%,1m                      | T07:42,9
            FormatAssertion {
                format: "T%h:%,1m".into(),
                input: "T07:42,9".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 900,
                    timezone: None,
                }),
            },
            //T%h:%.1m                      | T07:42.9
            FormatAssertion {
                format: "T%h:%.1m".into(),
                input: "T07:42.9".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 900,
                    timezone: None,
                }),
            },
            //T%h:%m:%s                     | T07:42:55
            FormatAssertion {
                format: "T%h:%m:%s".into(),
                input: "T07:42:55".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //T%h:%m:%.1s                   | T07:42:55.8
            FormatAssertion {
                format: "T%h:%m:%.1s".into(),
                input: "T07:42:55.8".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 800,
                    timezone: None,
                }),
            },
            //T%h:%m:%.2s                   | T07:42:55.87
            FormatAssertion {
                format: "T%h:%m:%.2s".into(),
                input: "T07:42:55.87".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h:%m:%,3s                   | T07:42:55,870
            FormatAssertion {
                format: "T%h:%m:%,3s".into(),
                input: "T07:42:55,870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h:%m:%.3s                   | T07:42:55.870
            FormatAssertion {
                format: "T%h:%m:%.3s".into(),
                input: "T07:42:55.870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h:%m:%s,%u                  | T07:42:55,870479
            FormatAssertion {
                format: "T%h:%m:%s,%u".into(),
                input: "T07:42:55,870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //T%h:%m:%s.%u                  | T07:42:55.870479
            FormatAssertion {
                format: "T%h:%m:%s.%u".into(),
                input: "T07:42:55.870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m                          | 0742
            FormatAssertion {
                format: "%h%m".into(),
                input: "0742".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 0,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //%h%m%s                        | 074255
            FormatAssertion {
                format: "%h%m%s".into(),
                input: "074255".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 0,
                    timezone: None,
                }),
            },
            //%h%m%.1s                      | 074255.8
            FormatAssertion {
                format: "%h%m%.1s".into(),
                input: "074255.8".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 800,
                    timezone: None,
                }),
            },
            //%h%m%.2s                      | 074255.87
            FormatAssertion {
                format: "%h%m%.2s".into(),
                input: "074255.87".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m%,3s                      | 074255,870
            FormatAssertion {
                format: "%h%m%,3s".into(),
                input: "074255,870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m%.3s                      | 074255.870
            FormatAssertion {
                format: "%h%m%.3s".into(),
                input: "074255.870".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m%s,%u                     | 074255,870479
            FormatAssertion {
                format: "%h%m%s,%u".into(),
                input: "074255,870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },
            //%h%m%s.%u                     | 074255.870479
            FormatAssertion {
                format: "%h%m%s.%u".into(),
                input: "074255.870479".into(),
                expected: Ok(Time {
                    hour: 7,
                    minute: 42,
                    second: 55,
                    millisecond: 870,
                    timezone: None,
                }),
            },

        ],
        timezone_assertions: crate::timezone::assertions(),
    }
}