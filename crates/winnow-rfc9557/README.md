# winnow-rfc9557, making parsing [RFC9557][iso] dates a breeze

[![crates.io](https://img.shields.io/crates/v/winnow-rfc9557?style=flat-square)](https://crates.io/crates/winnow-rfc9557)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/winnow-rfc9557)

[iso]: https://datatracker.ietf.org/doc/html/rfc9557
[winnow]: https://github.com/winnow-rs/winnow
[winnow-datetime]: https://crates.io/crates/winnow-datetime

## About

This library contains parsers for parsing RFC9557 dates and their various components built off the
[winnow-datetime parsers][winnow-datetime]

### Parsing

#### Complete
If you have all the data you need, you can just pass along the input directly.

```rust
use winnow_datetime::{Date, DateTime, Offset, Time, Calendar, NamedTimeZone, TimeZone};

let datetime = winnow_rfc9557::parse_datetime("2015-06-26T16:43:23.870479+02:00[Europe/Paris][u-ca=gregory]").unwrap();

assert_eq!(
    datetime,
    DateTime {
        date: Date::YMD { year: 2015, month: 6, day: 26 },
        time: Time {
            hour: 16,
            minute: 43,
            second: 23,
            nanosecond: 870_479_000,
            offset: Some(Offset::Fixed { hours: 2, minutes: 0, critical: false }),
            time_zone: Some(TimeZone::Named {
                zone: NamedTimeZone { identifier: "Europe/Paris".into(), critical: false },
            }),
            calendar: Some(Calendar { identifier: "gregory".into(), critical: false }),
        },
    }
);
```

#### Partial
For partial data the only difference is wrapping input in `Partial` and handling incomplete errors correctly,
which is documented in [winnow partial docs](https://docs.rs/winnow/latest/winnow/_topic/partial/index.html).

```rust
use winnow::error::{ErrMode, InputError};
use winnow::{Parser, Partial};
use winnow_rfc9557::datetime::datetime;

// the input stops part way through the time, so the parser asks for more rather than failing
let mut input = Partial::new("2015-06-26T16:43".as_bytes());
let result = datetime::<_, ErrMode<InputError<_>>>.parse_next(&mut input);

assert!(matches!(result, Err(ErrMode::Incomplete(_))));
```

# Caveats
## Timezone Suffixes
The critical flag for suffixes are parsed according to the spec and saved in the AST but don't have an effect on
exports to jiff (the only crate that currently handles timezones correctly). At the moment, if the offset or timezone
was set in the suffix it simply takes precedence.

### Calendar Suffixes
Per the spec, multiple calendar suffixes are allowed, but only the first one is parsed and used. The rest are ignored
even if the critical flag is set.

# Contributors

winnow-rfc9557 is the fruit of the work of many contributors over the years, many thanks for your help!

# [Documentation][docs]

[Documentation][docs] is online.

# License

MIT Licensed. See [LICENSE](https://mit-license.org/)

[docs]: https://docs.rs/winnow_rfc9557/
