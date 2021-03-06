//! Traits and utilities for temporal data.
use crate::prelude::*;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

// Conversion extracted from:
// https://docs.rs/arrow/1.0.0/src/arrow/array/array.rs.html#589

/// Number of seconds in a day
const SECONDS_IN_DAY: i64 = 86_400;
/// Number of milliseconds in a second
const MILLISECONDS_IN_SECOND: i64 = 1_000;
/// Number of microseconds in a second
const MICROSECONDS_IN_SECOND: i64 = 1_000_000;
/// Number of nanoseconds in a second
const NANOSECONDS_IN_SECOND: i64 = 1_000_000_000;

pub(crate) fn date32_as_datetime(v: i32) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(v as i64 * SECONDS_IN_DAY, 0)
}

pub(crate) fn date64_as_datetime(v: i64) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(
        // extract seconds from milliseconds
        v / MILLISECONDS_IN_SECOND,
        // discard extracted seconds and convert milliseconds to nanoseconds
        (v % MILLISECONDS_IN_SECOND * MICROSECONDS_IN_SECOND) as u32,
    )
}

pub(crate) fn timestamp_nanoseconds_as_datetime(v: i64) -> NaiveDateTime {
    // some nanoseconds will be truncated down as integer division rounds downwards
    let seconds = v / 1_000_000_000;
    // we can use that to compute the remaining nanoseconds
    let nanoseconds = (v - (seconds * 1_000_000_000)) as u32;

    NaiveDateTime::from_timestamp(seconds, nanoseconds)
}

pub(crate) fn timestamp_microseconds_as_datetime(v: i64) -> NaiveDateTime {
    // see nanoseconds for the logic
    let seconds = v / 1_000_000;
    let microseconds = (v - (seconds * 1_000_000)) as u32;

    NaiveDateTime::from_timestamp(seconds, microseconds)
}

pub(crate) fn timestamp_milliseconds_as_datetime(v: i64) -> NaiveDateTime {
    // see nanoseconds for the logic
    let seconds = v / 1000;
    let milliseconds = (v - (seconds * 1000)) as u32;

    NaiveDateTime::from_timestamp(seconds, milliseconds)
}

pub(crate) fn timestamp_seconds_as_datetime(seconds: i64) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(seconds, 0)
}

// date64 is number of milliseconds since the Unix Epoch
pub(crate) fn naive_datetime_to_date64(v: &NaiveDateTime) -> i64 {
    v.timestamp_millis()
}

pub(crate) fn naive_datetime_to_timestamp_nanoseconds(v: &NaiveDateTime) -> i64 {
    v.timestamp_nanos()
}

pub(crate) fn naive_datetime_to_timestamp_microseconds(v: &NaiveDateTime) -> i64 {
    v.timestamp() * 1_000_000 + v.timestamp_subsec_micros() as i64
}

pub(crate) fn naive_datetime_to_timestamp_milliseconds(v: &NaiveDateTime) -> i64 {
    v.timestamp_millis()
}

pub(crate) fn naive_datetime_to_timestamp_seconds(v: &NaiveDateTime) -> i64 {
    v.timestamp()
}

pub(crate) fn naive_time_to_time64_nanoseconds(v: &NaiveTime) -> i64 {
    // 3600 seconds in an hour
    v.hour() as i64 * 3600 * NANOSECONDS_IN_SECOND
        // 60 seconds in a minute
        + v.minute() as i64 * 60 * NANOSECONDS_IN_SECOND
        + v.second() as i64 * NANOSECONDS_IN_SECOND
        + v.nanosecond() as i64
}

pub(crate) fn naive_time_to_time64_microseconds(v: &NaiveTime) -> i64 {
    v.hour() as i64 * 3600 * MICROSECONDS_IN_SECOND
        + v.minute() as i64 * 60 * MICROSECONDS_IN_SECOND
        + v.second() as i64 * MICROSECONDS_IN_SECOND
        + v.nanosecond() as i64 / 1000
}

pub(crate) fn naive_time_to_time32_milliseconds(v: &NaiveTime) -> i32 {
    v.hour() as i32 * 3600 * MILLISECONDS_IN_SECOND as i32
        + v.minute() as i32 * 60 * MILLISECONDS_IN_SECOND as i32
        + v.second() as i32 * MILLISECONDS_IN_SECOND as i32
        + v.nanosecond() as i32 / 1000_000
}

pub(crate) fn naive_time_to_time32_seconds(v: &NaiveTime) -> i32 {
    v.hour() as i32 * 3600 + v.minute() as i32 * 60 + v.second() as i32 + v.nanosecond() as i32
}
pub(crate) fn time64_nanosecond_as_time(v: i64) -> NaiveTime {
    NaiveTime::from_num_seconds_from_midnight(
        // extract seconds from nanoseconds
        (v / NANOSECONDS_IN_SECOND) as u32,
        // discard extracted seconds
        (v % NANOSECONDS_IN_SECOND) as u32,
    )
}

pub(crate) fn time64_microsecond_as_time(v: i64) -> NaiveTime {
    NaiveTime::from_num_seconds_from_midnight(
        // extract seconds from microseconds
        (v / MICROSECONDS_IN_SECOND) as u32,
        // discard extracted seconds and convert microseconds to
        // nanoseconds
        (v % MICROSECONDS_IN_SECOND * MILLISECONDS_IN_SECOND) as u32,
    )
}

pub(crate) fn time32_second_as_time(v: i32) -> NaiveTime {
    NaiveTime::from_num_seconds_from_midnight(v as u32, 0)
}

pub(crate) fn time32_millisecond_as_time(v: i32) -> NaiveTime {
    let v = v as u32;
    NaiveTime::from_num_seconds_from_midnight(
        // extract seconds from milliseconds
        v / MILLISECONDS_IN_SECOND as u32,
        // discard extracted seconds and convert milliseconds to
        // nanoseconds
        v % MILLISECONDS_IN_SECOND as u32 * MICROSECONDS_IN_SECOND as u32,
    )
}

pub fn unix_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}

pub trait FromNaiveTime<T, N> {
    fn new_from_naive_time(name: &str, v: &[N]) -> Self;

    fn parse_from_str_slice(name: &str, v: &[&str], fmt: &str) -> Self;
}

fn parse_naive_time_from_str(s: &str, fmt: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(s, fmt).ok()
}

macro_rules! impl_from_naive_time {
    ($arrowtype:ident, $chunkedtype:ident, $func:ident) => {
        impl FromNaiveTime<$arrowtype, NaiveTime> for $chunkedtype {
            fn new_from_naive_time(name: &str, v: &[NaiveTime]) -> Self {
                let unit = v.iter().map($func).collect::<AlignedVec<_>>();
                ChunkedArray::new_from_aligned_vec(name, unit)
            }

            fn parse_from_str_slice(name: &str, v: &[&str], fmt: &str) -> Self {
                ChunkedArray::new_from_opt_iter(
                    name,
                    v.iter()
                        .map(|s| parse_naive_time_from_str(s, fmt).as_ref().map($func)),
                )
            }
        }
    };
}

impl_from_naive_time!(
    Time64NanosecondType,
    Time64NanosecondChunked,
    naive_time_to_time64_nanoseconds
);
impl_from_naive_time!(
    Time64MicrosecondType,
    Time64MicrosecondChunked,
    naive_time_to_time64_microseconds
);
impl_from_naive_time!(
    Time32MillisecondType,
    Time32MillisecondChunked,
    naive_time_to_time32_milliseconds
);
impl_from_naive_time!(
    Time32SecondType,
    Time32SecondChunked,
    naive_time_to_time32_seconds
);

pub trait AsNaiveTime {
    fn as_naive_time(&self) -> Vec<Option<NaiveTime>>;
}

macro_rules! impl_as_naivetime {
    ($ca:ty, $fun:ident) => {
        impl AsNaiveTime for $ca {
            fn as_naive_time(&self) -> Vec<Option<NaiveTime>> {
                self.into_iter().map(|opt_t| opt_t.map($fun)).collect()
            }
        }
    };
}

impl_as_naivetime!(Time32SecondChunked, time32_second_as_time);
impl_as_naivetime!(Time32MillisecondChunked, time32_millisecond_as_time);
impl_as_naivetime!(Time64NanosecondChunked, time64_nanosecond_as_time);
impl_as_naivetime!(Time64MicrosecondChunked, time64_microsecond_as_time);

fn parse_naive_datetime_from_str(s: &str, fmt: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, fmt).ok()
}

pub trait FromNaiveDateTime<T, N> {
    fn new_from_naive_datetime(name: &str, v: &[N]) -> Self;

    fn parse_from_str_slice(name: &str, v: &[&str], fmt: &str) -> Self;
}

macro_rules! impl_from_naive_datetime {
    ($arrowtype:ident, $chunkedtype:ident, $func:ident) => {
        impl FromNaiveDateTime<$arrowtype, NaiveDateTime> for $chunkedtype {
            fn new_from_naive_datetime(name: &str, v: &[NaiveDateTime]) -> Self {
                let unit = v.iter().map($func).collect::<AlignedVec<_>>();
                ChunkedArray::new_from_aligned_vec(name, unit)
            }

            fn parse_from_str_slice(name: &str, v: &[&str], fmt: &str) -> Self {
                ChunkedArray::new_from_opt_iter(
                    name,
                    v.iter()
                        .map(|s| parse_naive_datetime_from_str(s, fmt).as_ref().map($func)),
                )
            }
        }
    };
}

impl_from_naive_datetime!(Date64Type, Date64Chunked, naive_datetime_to_date64);
impl_from_naive_datetime!(
    TimestampNanosecondType,
    TimestampNanosecondChunked,
    naive_datetime_to_timestamp_nanoseconds
);
impl_from_naive_datetime!(
    TimestampMicrosecondType,
    TimestampMicrosecondChunked,
    naive_datetime_to_timestamp_microseconds
);
impl_from_naive_datetime!(
    TimestampMillisecondType,
    TimestampMillisecondChunked,
    naive_datetime_to_timestamp_milliseconds
);
impl_from_naive_datetime!(
    TimestampSecondType,
    TimestampSecondChunked,
    naive_datetime_to_timestamp_seconds
);

pub trait FromNaiveDate<T, N> {
    fn new_from_naive_date(name: &str, v: &[N]) -> Self;

    fn parse_from_str_slice(name: &str, v: &[&str], fmt: &str) -> Self;
}

fn naive_date_to_date32(nd: NaiveDate, unix_time: NaiveDate) -> i32 {
    nd.signed_duration_since(unix_time).num_days() as i32
}

fn parse_naive_date_from_str(s: &str, fmt: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, fmt).ok()
}

fn unix_time_naive_date() -> NaiveDate {
    NaiveDate::from_ymd(1970, 1, 1)
}

impl FromNaiveDate<Date32Type, NaiveDate> for Date32Chunked {
    fn new_from_naive_date(name: &str, v: &[NaiveDate]) -> Self {
        let unix_date = unix_time_naive_date();

        let unit = v
            .iter()
            .map(|v| naive_date_to_date32(*v, unix_date))
            .collect::<AlignedVec<_>>();
        ChunkedArray::new_from_aligned_vec(name, unit)
    }

    fn parse_from_str_slice(name: &str, v: &[&str], fmt: &str) -> Self {
        let unix_date = unix_time_naive_date();

        ChunkedArray::new_from_opt_iter(
            name,
            v.iter().map(|s| {
                parse_naive_date_from_str(s, fmt)
                    .as_ref()
                    .map(|v| naive_date_to_date32(*v, unix_date))
            }),
        )
    }
}

pub trait AsNaiveDateTime {
    fn as_naive_datetime(&self) -> Vec<Option<NaiveDateTime>>;
}

macro_rules! impl_as_naive_datetime {
    ($ca:ty, $fun:ident) => {
        impl AsNaiveDateTime for $ca {
            fn as_naive_datetime(&self) -> Vec<Option<NaiveDateTime>> {
                self.into_iter().map(|opt_t| opt_t.map($fun)).collect()
            }
        }
    };
}

impl_as_naive_datetime!(Date32Chunked, date32_as_datetime);
impl_as_naive_datetime!(Date64Chunked, date64_as_datetime);
impl_as_naive_datetime!(
    TimestampNanosecondChunked,
    timestamp_nanoseconds_as_datetime
);
impl_as_naive_datetime!(
    TimestampMicrosecondChunked,
    timestamp_microseconds_as_datetime
);
impl_as_naive_datetime!(
    TimestampMillisecondChunked,
    timestamp_milliseconds_as_datetime
);
impl_as_naive_datetime!(TimestampSecondChunked, timestamp_seconds_as_datetime);

pub trait AsNaiveDate {
    fn as_naive_date(&self) -> Vec<Option<NaiveDate>>;
}

impl AsNaiveDate for Date32Chunked {
    fn as_naive_date(&self) -> Vec<Option<NaiveDate>> {
        self.into_iter()
            .map(|opt_t| {
                opt_t.map(|v| {
                    let dt = date32_as_datetime(v);
                    NaiveDate::from_ymd(dt.year(), dt.month(), dt.day())
                })
            })
            .collect()
    }
}

#[cfg(all(test, feature = "temporal"))]
mod test {
    use crate::prelude::*;
    use chrono::{NaiveDateTime, NaiveTime};

    #[test]
    fn from_time() {
        let times: Vec<_> = ["23:56:04", "00:00:00"]
            .iter()
            .map(|s| NaiveTime::parse_from_str(s, "%H:%M:%S").unwrap())
            .collect();
        let t = Time64NanosecondChunked::new_from_naive_time("times", &times);
        // NOTE: the values are checked and correct.
        assert_eq!([86164000000000, 0], t.cont_slice().unwrap());
        let t = Time64MicrosecondChunked::new_from_naive_time("times", &times);
        assert_eq!([86164000000, 0], t.cont_slice().unwrap());
        let t = Time32MillisecondChunked::new_from_naive_time("times", &times);
        assert_eq!([86164000, 0], t.cont_slice().unwrap());
        let t = Time32SecondChunked::new_from_naive_time("times", &times);
        assert_eq!([86164, 0], t.cont_slice().unwrap());
    }

    #[test]
    fn from_datetime() {
        let datetimes: Vec<_> = [
            "1988-08-25 00:00:16",
            "2015-09-05 23:56:04",
            "2012-12-21 00:00:00",
        ]
        .iter()
        .map(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap())
        .collect();

        // NOTE: the values are checked and correct.
        let dt = Date64Chunked::new_from_naive_datetime("name", &datetimes);
        assert_eq!(
            [588470416000, 1441497364000, 1356048000000],
            dt.cont_slice().unwrap()
        );
        let dt = TimestampNanosecondChunked::new_from_naive_datetime("name", &datetimes);
        assert_eq!(
            [588470416000000000, 1441497364000000000, 1356048000000000000],
            dt.cont_slice().unwrap()
        );
        let dt = TimestampMicrosecondChunked::new_from_naive_datetime("name", &datetimes);
        assert_eq!(
            [588470416000000, 1441497364000000, 1356048000000000],
            dt.cont_slice().unwrap()
        );
        let dt = TimestampMillisecondChunked::new_from_naive_datetime("name", &datetimes);
        assert_eq!(
            [588470416000, 1441497364000, 1356048000000],
            dt.cont_slice().unwrap()
        );
        let dt = TimestampSecondChunked::new_from_naive_datetime("name", &datetimes);
        assert_eq!(
            [588470416, 1441497364, 1356048000],
            dt.cont_slice().unwrap()
        );
    }

    #[test]
    fn from_date() {
        let dates = &[
            "2020-08-21",
            "2020-08-21",
            "2020-08-22",
            "2020-08-23",
            "2020-08-22",
        ];
        let fmt = "%Y-%m-%d";
        let ca = Date32Chunked::parse_from_str_slice("dates", dates, fmt);
        assert_eq!(
            [18495, 18495, 18496, 18497, 18496],
            ca.cont_slice().unwrap()
        );
    }
}
