extern crate core;

mod clippy;
#[cfg(any(feature = "time", feature = "chrono"))]
mod convert;
mod macros;
pub mod parser;
pub mod types;
pub mod util;

pub use types::Date;
pub use types::DateTime;
pub use types::Duration;
pub use types::FractionalDuration;
pub use types::Interval;
pub use types::Offset;
pub use types::Time;
use winnow::Partial;

/// Type for holding partial data for parsers
pub type Stream<'i> = Partial<&'i [u8]>;
