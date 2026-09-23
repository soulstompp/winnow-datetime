## 0.4.0 - 2026-09-23
* Bumped winnow version to 1.0
* Renamed `Time::millisecond` and `PartialTime::millisecond` to `nanosecond`, keeping up to nine fractional digits instead of three
* Renamed `parser::fraction_millisecond` to `parser::fraction_nanosecond`
* Fixed chrono and time conversions dropping the fractional second
* Fixed jiff conversion passing milliseconds as nanoseconds
* Fixed jiff conversions missing when `jiff` was the only conversion feature enabled
* Fixed chrono `DateTime` conversion reading offset minutes as seconds
* Fixed chrono conversion of week dates landing one day late
* Removed unused `paste` and `num-traits` dependencies

## 0.3.0 - 2025-05-30
* Addition of TimeZone enum to support new information from RFC9557
* Addition of Calendar struct to support new information from RFC9557
* Changed Offset to not rely on Option, which caused problems with test suite YAML
* Added critical value to Offset for RFC9557 support
* Added support for setting named time zones for jiff exports

## 0.2.3 - 2025-05-14
* Support for `Date::Ordinal` conversions to `jiff::civil::Date`

## 0.2.2 - 2025-05-14
* Added convert::jiff for jiff support of date and time

## 0.2.1 - 2025-05-11
* Made the convert modules public so that they can be used outside of the crate.
* Added TryInto for time::OffsetDateTime

## 0.2.0 - 2025-05-04
* Changed `Stream` helper type to `PartialInput` since this name conflicts with the winnow naming scheme and causes confusion.
* Bumped winnow version to 0.7 and changed parser singatures to match standard winnow parsers

## 0.1.0 - 2014-12-29 - Initial Release
