use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub fn current_timestamp() -> i64 {
    Local::now().timestamp()
}

pub fn parse_timestamp(millis: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(millis, 0);
    let datetime = DateTime::<Local>::from_utc(naive, Local.offset_from_utc_datetime(&naive));
    let formatted = datetime.format("%a %b %0e %T %Y");

    formatted.to_string()
}
