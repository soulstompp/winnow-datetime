extern crate core;

mod clippy;
#[cfg(any(feature = "time", feature = "chrono"))]
pub mod convert;
mod macros;
pub mod parser;
pub mod types;
pub mod util;

pub use types::Calendar;
pub use types::Date;
pub use types::DateTime;
pub use types::Duration;
pub use types::FractionalDuration;
pub use types::Interval;
pub use types::NamedTimeZone;
pub use types::Offset;
pub use types::Time;
pub use types::TimeZone;

use winnow::Partial;

/// Type for holding partial data for parsers
pub type PartialInput<'i> = Partial<&'i [u8]>;
