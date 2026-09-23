# winnow-rfc3339, making parsing [RFC3339][iso] dates a breeze

[![crates.io](https://img.shields.io/crates/v/winnow-rfc3339?style=flat-square)](https://crates.io/crates/winnow-rfc3339)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/winnow-rfc3339)

[iso]: https://www.rfc-editor.org/rfc/rfc3339
[winnow]: https://github.com/winnow-rs/winnow
[winnow-datetime]: https://crates.io/crates/winnow-datetime

## About

This library contains parsers for parsing RFC3339 dates and their various components built off the
[winnow-datetime parsers][winnow-datetime]

### Parsing

#### Complete
If you have all the data you need, you can just pass along the input directly.

```rust
use winnow_datetime::{Date, DateTime, Offset, Time};

let datetime = winnow_rfc3339::parse_datetime("2015-06-26T16:43:23.870479+02:00").unwrap();

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
            time_zone: None,
            calendar: None,
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
use winnow_rfc3339::datetime::datetime;

// the input stops part way through the time, so the parser asks for more rather than failing
let mut input = Partial::new("2015-06-26T16:43".as_bytes());
let result = datetime::<_, ErrMode<InputError<_>>>.parse_next(&mut input);

assert!(matches!(result, Err(ErrMode::Incomplete(_))));
```

# Contributors

winnow-rfc3339 is the fruit of the work of many contributors over the years, many thanks for your help!

# [Documentation][docs]

[Documentation][docs] is online.

# License

MIT Licensed. See [LICENSE](https://mit-license.org/)

[docs]: https://docs.rs/winnow_rfc3339/
