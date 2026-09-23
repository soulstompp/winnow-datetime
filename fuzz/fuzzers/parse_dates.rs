#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = winnow_iso8601::parse_date(data);
        let _ = winnow_iso8601::parse_time(data);
        let _ = winnow_iso8601::parse_datetime(data);
        let _ = winnow_iso8601::parse_duration(data);

        let _ = winnow_rfc3339::parse_date(data);
        let _ = winnow_rfc3339::parse_time(data);
        let _ = winnow_rfc3339::parse_datetime(data);

        let _ = winnow_rfc9557::parse_date(data);
        let _ = winnow_rfc9557::parse_time(data);
        let _ = winnow_rfc9557::parse_datetime(data);
    }
});
