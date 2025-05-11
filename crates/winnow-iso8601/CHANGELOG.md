# Changelog

## 0.5.1 - 2024-05-11
* Dependency on winnow-datetime 0.2 instead of a more specific version
* Dropped feature "serde" for winnow-datetime since it isn't needed.

## 0.5.0 - 2024-05-04
* Changed parser signatures to meet winnow 0.7 standards

## 0.4.0 - 2014-12-29
* Moved Date, Time, and DateTime, Timezone to the `winnow-datetime` so that upcoming format crates
  can re-use them. This crate will no longer export these directly, since this caused several type
  misinterpretations by the compiler.
* Moved several base parsers to the `winnow-datetime` crate. Duration and Period parsers are still
  here, since not many other formats cover these concepts.
* Addition of the `Datetime::Century` and `Datetime::Decade` variants, which are new additions to 
  ISO 8601-1:2019.
* Fixes to some situations where year-week dates were not being parsed correctly.
* Test generation methods and improved testing from the workspace's new crate
  `winnow-datetime-assert`
* Added support for 'T' preceding the time in the time parser.
* Using new test builder from the new winnow-datetime-assert crate in this workspace. Old tests
  should be almost entirely redundant but currently remain to protect against regression as these
  tests are made less verbose in the near future.
* Timezone/timezone parsers were renamed to offset, since upcoming versions will have named timezones
  and offsets is what is used in the specs.
* timezone_utc parser renamed offset_zulu
* the helper methods for getting these objects from an str are now called parser_(year|date|time|offset)
  and the corresponding parsers no longer have the parser_ suffix so that they are uniform with the rest of
  the parser names.

 
## 0.3.0 - 2024-12-17
* Conversion traits for time-rs module and moved both time-rs and chrono to convert module.
* Minor clean-up of date and sign parsers
* Fixes several inherited issues with parsing durations

## 0.2.1 - 2024-12-15
* Accept &str for all parsers and stop passing bytes for string parser helper functions.

## 0.2.0 - 2024-11-16
* Added Timezone struct so offsets are
* Support for complete values in parsers

## 0.1.0 - 2024-11-15

Initial release/Fork from nom-iso8601 @ 6c59a4a7365bbe0

* Changes to parsers in order to work with winnow 0.6.20 parser combinator.
* Accept partial data for all parsers
* Added trace to all parsers
* Updated README and LICENSE
* Removal of tutorial
