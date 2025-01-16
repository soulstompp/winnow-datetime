use super::*;
use winnow_datetime::parser::{date_day, date_month};

#[test]
fn test_date_month() {
    assert_eq!(date_month(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(date_month(&mut "06".as_bstr()).unwrap(), 6);
    assert_eq!(date_month(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(date_month(&mut "12-".as_bstr()).unwrap(), 12);

    assert!(date_month(&mut Stream::new(b"13\n")).is_err());
    assert!(date_month(&mut Stream::new(b"00\n")).is_err());
}

#[test]
fn test_date_day() {
    assert_eq!(date_day(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(date_day(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(date_day(&mut "20".as_bstr()).unwrap(), 20);
    assert_eq!(date_day(&mut "28".as_bstr()).unwrap(), 28);
    assert_eq!(date_day(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(date_day(&mut "31".as_bstr()).unwrap(), 31);
    assert_eq!(date_day(&mut "31-".as_bstr()).unwrap(), 31);

    assert!(date_day(&mut Stream::new(b"00")).is_err());
    assert!(date_day(&mut Stream::new(b"32")).is_err());
}

#[test]
fn test_time_hour() {
    assert_eq!(time_hour(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_hour(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_hour(&mut "06".as_bstr()).unwrap(), 6);
    assert_eq!(time_hour(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(time_hour(&mut "13".as_bstr()).unwrap(), 13);
    assert_eq!(time_hour(&mut "20".as_bstr()).unwrap(), 20);

    assert!(time_hour(&mut "24".as_bstr()).is_err());
    assert!(time_hour(&mut "25".as_bstr()).is_err());
    assert!(time_hour(&mut "30".as_bstr()).is_err());
    assert!(time_hour(&mut "ab".as_bstr()).is_err());
}

#[test]
fn test_time_minute() {
    assert_eq!(time_minute(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_minute(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_minute(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(time_minute(&mut "59".as_bstr()).unwrap(), 59);

    assert!(time_minute(&mut Stream::new(b"60")).is_err());
    assert!(time_minute(&mut Stream::new(b"61")).is_err());
    assert!(time_minute(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_time_second() {
    assert_eq!(time_second(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_second(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_second(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(time_second(&mut "59".as_bstr()).unwrap(), 59);
    assert_eq!(time_second(&mut "60".as_bstr()).unwrap(), 60);

    assert!(time_second(&mut Stream::new(b"61")).is_err());
    assert!(time_second(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_date() {
    assert!(date(&mut Stream::new(b"201")).is_err());
    assert!(date(&mut Stream::new(b"2015p00p00")).is_err());
    assert!(date(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_time() {
    assert!(time(&mut Stream::new(b"20:")).is_err());
    assert!(time(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_time_with_timezone() {
    assert!(time(&mut Stream::new(b"20:")).is_err());
    assert!(time(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_datetime_error() {
    let test_datetimes = vec!["ppp", "dumd-di-duTmd:iu:m"];

    for iso_string in test_datetimes {
        let res = parse_datetime(&mut Stream::new(iso_string.as_bytes()));
        assert!(res.is_err());
    }
}

#[test]
fn disallows_notallowed() {
    assert!(time(&mut Stream::new(b"30:90:90")).is_err());
    assert!(date(&mut Stream::new(b"0000-20-40")).is_err());
    assert!(parse_datetime(&mut Stream::new(b"2001-w05-6t04:05:06.123z")).is_err());
}
