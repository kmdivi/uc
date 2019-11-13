use chrono::{DateTime, TimeZone, Utc};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let datetime = &args[1];

    if datetime.len() == 10 {
        let datetime: i64 = datetime.parse::<i64>().unwrap();
        let timestamp = get_utc(datetime);
        println!("{}", timestamp);
    } else {
        let timestamp: i64 = get_unixtime(datetime);
        println!("{}", timestamp);
    }
}

fn get_unixtime(datetime: &str) -> i64 {
    let dt: DateTime<Utc> = Utc
        .datetime_from_str(datetime, "%Y-%m-%d %H:%M:%S")
        .unwrap();

    let timestamp: i64 = dt.timestamp();
    timestamp
}

fn get_utc(unixtime: i64) -> String {
    let dt = Utc.timestamp(unixtime, 0);
    dt.to_string()
}

#[test]
fn test_get_unixtime() {
    let timestamp: i64 = get_unixtime("2019-11-13 13:42:55");
    assert_eq!(timestamp, 1573652575);
}

#[test]
fn test_get_utc() {
    let timestamp: String = get_utc(1573652575);
    assert_eq!(timestamp, "2019-11-13 13:42:55 UTC".to_string());
}
