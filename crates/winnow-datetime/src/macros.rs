#[macro_export]
macro_rules! duration_part_seq {
    ({
        whole: $whole_parser:expr,
        sep: $sep_parser:expr,
        fraction: $fraction_parser:expr,
        end: $end_parser:expr
    }) => {
        move |input: &mut _| {
            seq!((
                $whole_parser,
                opt(preceded($sep_parser, $fraction_parser.map(|n| {
                   winnow_datetime::util::digits_to_fractional_f32(n)
                }
                ))),
                _ : $end_parser
            ))
            .map(move |(whole, frac)| DurationPart{ whole, frac })
            .parse_next(input)
        }
    };
}

#[macro_export]
macro_rules! date_yddd_seq {
    (
    $date_type: ident::$variant:ident {
        year: $year_parser:expr,
        day: $day_parser:expr,
    }) => {
        move |input: &mut _| {
            seq!($date_type::$variant {
                year: $year_parser,
                day: $day_parser,
            })
            .parse_next(input)
        }
    };
}

#[macro_export]
macro_rules! date_ymd_seq {
    (
    $date_type: ident::$variant:ident {
        year: $year_parser:expr,
        month: $month_parser:expr,
        day: $day_parser:expr,
    }) => {
        seq!($date_type::$variant {
            year: $year_parser,
            month: $month_parser,
            day: $day_parser,
        })
    };
}

#[macro_export]
macro_rules! date_ywd_seq {
    (
    $date_type: ident::$variant:ident {
        year: $year_parser:expr,
        week: $week_parser:expr,
        day: $day_parser:expr,
    }) => {
        seq!($date_type::$variant {
            year: $year_parser,
            week: $week_parser,
            day: $day_parser,
        })
    };
}

#[macro_export]
macro_rules! time_seq {
    (
    $variant:ident {
        hour: $hour_parser:expr,
        minute: $minute_parser:expr,
        second: $second_parser:expr,
        millisecond: $millisecond_parser:expr,
        offset: $offset_parser:expr,
        time_zone: $time_zone_parser:expr,
        calendar: $calendar_parser:expr,
    }) => {
        seq!($variant {
            hour: $hour_parser,
            minute: $minute_parser,
            second: $second_parser,
            millisecond: $millisecond_parser,
            offset: $offset_parser,
            time_zone: $time_zone_parser,
            calendar: $calendar_parser,
        })
    };
}

#[macro_export]
macro_rules! partial_time_seq {
    (
    $variant:ident {
        hour: $hour_parser:expr,
        minute: $minute_parser:expr,
        second: $second_parser:expr,
        millisecond: $millisecond_parser:expr,
        offset: $offset_parser:expr,
    }) => {
        seq!($variant {
            hour: $hour_parser,
            minute: $minute_parser,
            second: $second_parser,
            millisecond: $millisecond_parser,
            offset: $offset_parser,
        })
    };
}
