# winnow-rfc3339, making parsing [RFC3339][iso] dates a breeze

[![crates.io](https://img.shields.io/crates/v/winnow-rfc3339?style=flat-square)](https://crates.io/crates/winnow-rfc3339)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/winnow-rfc3339)

[iso]: https://en.wikipedia.org/wiki/ISO_8601
[winnow]: https://github.com/winnow-rs/winnow
[winnow-datetime]: https://crates.io/crates/winnow-datetime

## About

This library contains parsers for parsing RFC3339 dates and their various components built off the
[winnow-datetime parsers][winnow-datetime]

### Parsing

#### Complete
If you have all the data you need, you can just pass along the input directly.

```rust,ignore
let datetime = opt(datetime)
    .parse_next(&mut "2015-06-26 16:43:23+0200"));

// the above will give you:
Some(DateTime {
    date: Date::YMD {
        year: 2015,
        month: 6,
        day: 26,
    },
    time: Time {
        hour: 16,
        minute: 43,
        second: 23,
        tz_offset_hours: 2,
        tz_offset_minutes: 0,
    },
});
```

#### Partial
For partial data the only difference is wrapping input in Partial and handling incomplete errors correctly,
which is documented in [winnow partial docs](https://docs.rs/winnow/latest/winnow/_topic/partial/index.html).
```rust,ignore
pub type Stream<'i> = Partial<&'i [u8]>;

let datetime = opt(datetime)
    .parse_next(&mut Stream::new("2015-06-26 16:43:23+0200").as_bytes()));

// the above will give you:
Some(DateTime {
    date: Date::YMD {
        year: 2015,
        month: 6,
        day: 26,
    },
    time: Time {
        hour: 16,
        minute: 43,
        second: 23,
        tz_offset_hours: 2,
        tz_offset_minutes: 0,
    },
});
```

# Contributors

winnow-rfc3339 is the fruit of the work of many contributors over the years, many thanks for your help!

# [Documentation][docs]

[Documentation][docs] is online.

# License

MIT Licensed. See [LICENSE](https://mit-license.org/)

[docs]: https://docs.rs/winnow_rfc3339/
