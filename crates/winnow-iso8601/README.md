# winnow-iso8601, making parsing [ISO8601][iso] dates a breeze

[![crates.io](https://img.shields.io/crates/v/winnow-iso8601?style=flat-square)](https://crates.io/crates/winnow-iso8601)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/winnow-iso8601)

[iso]: https://en.wikipedia.org/wiki/ISO_8601
[winnow]: https://github.com/winnow-rs/winnow
[winnow-datetime]: https://crates.io/crates/winnow-datetime
[iso-crate]: https://crates.io/crates/iso8601

## About

This library contains parsers for parsing ISO8601 dates and their various components built off the [winnow-datetime parsers][winnow-datetime]

### Parsing

#### Complete
If you have all the data you need, you can just pass along the input directly.

```rust
use winnow_datetime::{Date, DateTime, Offset, Time};

let datetime = winnow_iso8601::parse_datetime("2015-06-26T16:43:23,870479+0200").unwrap();

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
use winnow_iso8601::datetime::datetime;

// the input stops part way through the time, so the parser asks for more rather than failing
let mut input = Partial::new("2015-06-26T16:43".as_bytes());
let result = datetime::<_, ErrMode<InputError<_>>>.parse_next(&mut input);

assert!(matches!(result, Err(ErrMode::Incomplete(_))));
```

# Contributors

winnow-iso8601 is the fruit of the work of many contributors over the years, many
thanks for your help! In particular, thanks to [badboy](https://github.com/badboy)
and [hoodie](https://github.com/hoodie) for the original [`iso8601` crate][iso-crate] and actually reading the standard.

# [Documentation][docs]

[Documentation][docs] is online.

# License

MIT Licensed. See [LICENSE](https://mit-license.org/)

[docs]: https://docs.rs/winnow_iso8601/