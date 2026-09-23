# Changelog

## 0.2.0 - 2026-09-23
* Bumped winnow version to 1.0
* Changes to support winnow-datetime 0.4.0 types, fractional seconds now parsed to nanoseconds
* Fixed `islamicc` calendar being parsed as `islamic`
* `chrono`, `jiff`, `serde` and `time` features now enable the matching winnow-datetime conversions

## 0.1.0 - 2025-05-01

Initial release

* Parsers and tests to cover the spec, essentially RFC3339 with timezone and calendar suffixes.