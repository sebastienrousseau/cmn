#![allow(missing_docs)]
// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use cmn::datetime::{DateTime, Duration, ParseError};

    // ===============================================================
    // DateTime::parse — valid ISO 8601
    // ===============================================================

    #[test]
    fn parse_utc_basic() {
        let dt = DateTime::parse("2026-04-05T14:30:00Z").unwrap();
        assert_eq!(dt.year(), 2026);
        assert_eq!(dt.month(), 4);
        assert_eq!(dt.day(), 5);
        assert_eq!(dt.hour(), 14);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 0);
        assert_eq!(dt.offset_minutes(), 0);
    }

    #[test]
    fn parse_positive_offset() {
        let dt = DateTime::parse("2026-01-15T09:00:00+05:30").unwrap();
        assert_eq!(dt.offset_minutes(), 330);
    }

    #[test]
    fn parse_negative_offset() {
        let dt = DateTime::parse("2026-12-31T23:59:59-08:00").unwrap();
        assert_eq!(dt.offset_minutes(), -480);
        assert_eq!(dt.hour(), 23);
        assert_eq!(dt.second(), 59);
    }

    #[test]
    fn parse_midnight() {
        let dt = DateTime::parse("2000-01-01T00:00:00Z").unwrap();
        assert_eq!(dt.hour(), 0);
        assert_eq!(dt.minute(), 0);
        assert_eq!(dt.second(), 0);
    }

    #[test]
    fn parse_leap_day() {
        let dt = DateTime::parse("2024-02-29T12:00:00Z").unwrap();
        assert_eq!(dt.month(), 2);
        assert_eq!(dt.day(), 29);
    }

    #[test]
    fn parse_epoch() {
        let dt = DateTime::parse("1970-01-01T00:00:00Z").unwrap();
        assert_eq!(dt.to_unix_timestamp(), 0);
    }

    // ===============================================================
    // DateTime::parse — invalid inputs
    // ===============================================================

    #[test]
    fn parse_too_short() {
        assert_eq!(
            DateTime::parse("2026-04-05"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_no_timezone() {
        assert_eq!(
            DateTime::parse("2026-04-05T14:30:00"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_bad_separator() {
        assert_eq!(
            DateTime::parse("2026-04-05 14:30:00Z"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_month_13() {
        assert_eq!(
            DateTime::parse("2026-13-01T00:00:00Z"),
            Err(ParseError::OutOfRange)
        );
    }

    #[test]
    fn parse_day_32() {
        assert_eq!(
            DateTime::parse("2026-01-32T00:00:00Z"),
            Err(ParseError::OutOfRange)
        );
    }

    #[test]
    fn parse_feb_29_non_leap() {
        assert_eq!(
            DateTime::parse("2025-02-29T00:00:00Z"),
            Err(ParseError::OutOfRange)
        );
    }

    #[test]
    fn parse_hour_24() {
        assert_eq!(
            DateTime::parse("2026-01-01T24:00:00Z"),
            Err(ParseError::OutOfRange)
        );
    }

    #[test]
    fn parse_bad_timezone_suffix() {
        assert_eq!(
            DateTime::parse("2026-01-01T00:00:00X"),
            Err(ParseError::InvalidTimezone)
        );
    }

    #[test]
    fn parse_empty_string() {
        assert_eq!(DateTime::parse(""), Err(ParseError::InvalidFormat));
    }

    #[test]
    fn parse_garbage() {
        assert_eq!(
            DateTime::parse("not-a-date-at-all!!!"),
            Err(ParseError::InvalidFormat)
        );
    }

    // ===============================================================
    // DateTime::new — validation
    // ===============================================================

    #[test]
    fn new_valid() {
        assert!(DateTime::new(2026, 4, 5, 14, 30, 0, 0).is_some());
    }

    #[test]
    fn new_month_zero() {
        assert!(DateTime::new(2026, 0, 1, 0, 0, 0, 0).is_none());
    }

    #[test]
    fn new_day_zero() {
        assert!(DateTime::new(2026, 1, 0, 0, 0, 0, 0).is_none());
    }

    #[test]
    fn new_second_60() {
        assert!(DateTime::new(2026, 1, 1, 0, 0, 60, 0).is_none());
    }

    #[test]
    fn new_offset_too_large() {
        assert!(DateTime::new(2026, 1, 1, 0, 0, 0, 1441).is_none());
    }

    // ===============================================================
    // DateTime::to_iso8601 — roundtrip
    // ===============================================================

    #[test]
    fn iso8601_roundtrip_utc() {
        let input = "2026-04-05T14:30:00Z";
        let dt = DateTime::parse(input).unwrap();
        assert_eq!(dt.to_iso8601(), input);
    }

    #[test]
    fn iso8601_roundtrip_positive_offset() {
        let input = "2026-04-05T14:30:00+05:30";
        let dt = DateTime::parse(input).unwrap();
        assert_eq!(dt.to_iso8601(), input);
    }

    #[test]
    fn iso8601_roundtrip_negative_offset() {
        let input = "2026-12-31T23:59:59-08:00";
        let dt = DateTime::parse(input).unwrap();
        assert_eq!(dt.to_iso8601(), input);
    }

    #[test]
    fn display_uses_iso8601() {
        let dt = DateTime::parse("2026-04-05T14:30:00Z").unwrap();
        assert_eq!(format!("{dt}"), "2026-04-05T14:30:00Z");
    }

    // ===============================================================
    // DateTime::to_unix_timestamp
    // ===============================================================

    #[test]
    fn unix_epoch_is_zero() {
        let dt = DateTime::parse("1970-01-01T00:00:00Z").unwrap();
        assert_eq!(dt.to_unix_timestamp(), 0);
    }

    #[test]
    fn unix_one_day() {
        let dt = DateTime::parse("1970-01-02T00:00:00Z").unwrap();
        assert_eq!(dt.to_unix_timestamp(), 86400);
    }

    #[test]
    fn unix_offset_adjusts() {
        let utc = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let plus2 =
            DateTime::parse("2026-04-05T14:00:00+02:00").unwrap();
        assert_eq!(utc.to_unix_timestamp(), plus2.to_unix_timestamp());
    }

    // ===============================================================
    // DateTime::duration_since
    // ===============================================================

    #[test]
    fn duration_two_hours() {
        let a = DateTime::parse("2026-04-05T10:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let d = b.duration_since(&a);
        assert_eq!(d.whole_seconds(), 7200);
        assert_eq!(d.whole_hours(), 2);
    }

    #[test]
    fn duration_negative() {
        let a = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T10:00:00Z").unwrap();
        let d = b.duration_since(&a);
        assert!(d.is_negative());
        assert_eq!(d.whole_seconds(), -7200);
    }

    #[test]
    fn duration_same_instant() {
        let a = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let d = a.duration_since(&a);
        assert_eq!(d.whole_seconds(), 0);
        assert!(!d.is_negative());
    }

    #[test]
    fn duration_cross_day() {
        let a = DateTime::parse("2026-04-05T23:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-06T01:00:00Z").unwrap();
        assert_eq!(b.duration_since(&a).whole_hours(), 2);
    }

    // ===============================================================
    // DateTime::relative_to
    // ===============================================================

    #[test]
    fn relative_seconds_ago() {
        let now = DateTime::parse("2026-04-05T12:00:30Z").unwrap();
        let then = DateTime::parse("2026-04-05T12:01:00Z").unwrap();
        let rel = now.relative_to(&then);
        assert_eq!(rel, "30 seconds ago");
    }

    #[test]
    fn relative_minutes_future() {
        let now = DateTime::parse("2026-04-05T12:05:00Z").unwrap();
        let then = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let rel = now.relative_to(&then);
        assert_eq!(rel, "in 5 minutes");
    }

    #[test]
    fn relative_hours() {
        let now = DateTime::parse("2026-04-05T15:00:00Z").unwrap();
        let then = DateTime::parse("2026-04-05T18:00:00Z").unwrap();
        let rel = now.relative_to(&then);
        assert_eq!(rel, "3 hours ago");
    }

    #[test]
    fn relative_days() {
        let now = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let then = DateTime::parse("2026-04-01T12:00:00Z").unwrap();
        let rel = now.relative_to(&then);
        assert_eq!(rel, "in 4 days");
    }

    #[test]
    fn relative_just_now() {
        let dt = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        assert_eq!(dt.relative_to(&dt), "just now");
    }

    #[test]
    fn relative_one_minute() {
        let a = DateTime::parse("2026-04-05T12:01:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        assert_eq!(a.relative_to(&b), "in 1 minute");
    }

    #[test]
    fn relative_one_hour() {
        let a = DateTime::parse("2026-04-05T13:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        assert_eq!(a.relative_to(&b), "in 1 hour");
    }

    #[test]
    fn relative_one_day() {
        let a = DateTime::parse("2026-04-06T12:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        assert_eq!(a.relative_to(&b), "in 1 day");
    }

    #[test]
    fn relative_months() {
        let a = DateTime::parse("2026-07-05T12:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        assert_eq!(a.relative_to(&b), "in 3 months");
    }

    #[test]
    fn relative_years() {
        let a = DateTime::parse("2028-04-05T12:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        assert_eq!(a.relative_to(&b), "in 2 years");
    }

    // ===============================================================
    // Duration
    // ===============================================================

    #[test]
    fn duration_from_seconds() {
        let d = Duration::from_seconds(90);
        assert_eq!(d.whole_seconds(), 90);
        assert_eq!(d.whole_minutes(), 1);
        assert_eq!(d.whole_hours(), 0);
        assert_eq!(d.whole_days(), 0);
    }

    #[test]
    fn duration_abs() {
        let d = Duration::from_seconds(-3600);
        assert!(d.is_negative());
        let a = d.abs();
        assert!(!a.is_negative());
        assert_eq!(a.whole_seconds(), 3600);
    }

    #[test]
    fn duration_display() {
        let d = Duration::from_seconds(3661);
        assert_eq!(format!("{d}"), "01:01:01");
    }

    #[test]
    fn duration_display_negative() {
        let d = Duration::from_seconds(-90);
        assert_eq!(format!("{d}"), "-00:01:30");
    }

    // ===============================================================
    // Trait coverage (Clone, Debug, Serialize, PartialOrd)
    // ===============================================================

    #[test]
    fn datetime_copy_eq() {
        let a = DateTime::parse("2026-04-05T14:30:00Z").unwrap();
        let b = a; // Copy
        assert_eq!(a, b);
    }

    #[test]
    fn datetime_ord() {
        let a = DateTime::parse("2026-04-05T14:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T15:00:00Z").unwrap();
        assert!(a < b);
    }

    #[test]
    fn datetime_debug() {
        let dt = DateTime::parse("2026-04-05T14:30:00Z").unwrap();
        let dbg = format!("{:?}", dt);
        assert!(dbg.contains("DateTime"));
    }

    #[test]
    fn datetime_serde_roundtrip() {
        let original = DateTime::parse("2026-04-05T14:30:00Z").unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let restored: DateTime = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn duration_serde_roundtrip() {
        let original = Duration::from_seconds(12345);
        let json = serde_json::to_string(&original).unwrap();
        let restored: Duration = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn parse_error_display() {
        assert_eq!(
            format!("{}", ParseError::InvalidFormat),
            "invalid ISO 8601 format"
        );
        assert_eq!(
            format!("{}", ParseError::OutOfRange),
            "date/time field out of range"
        );
        assert_eq!(
            format!("{}", ParseError::InvalidTimezone),
            "invalid timezone offset"
        );
    }

    #[test]
    fn parse_error_is_std_error() {
        let e: Box<dyn std::error::Error> =
            Box::new(ParseError::InvalidFormat);
        assert!(!e.to_string().is_empty());
    }

    // ===============================================================
    // Coverage: specific separator / guard-clause branches
    // ===============================================================

    #[test]
    fn parse_bad_dash_pos4() {
        // position 4 should be '-'
        assert_eq!(
            DateTime::parse("2026X04-05T14:30:00Z"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_bad_dash_pos7() {
        assert_eq!(
            DateTime::parse("2026-04X05T14:30:00Z"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_bad_colon_pos13() {
        assert_eq!(
            DateTime::parse("2026-04-05T14X30:00Z"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_bad_colon_pos16() {
        assert_eq!(
            DateTime::parse("2026-04-05T14:30X00Z"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_z_with_trailing_chars() {
        assert_eq!(
            DateTime::parse("2026-04-05T14:30:00Zextra"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_offset_wrong_length() {
        assert_eq!(
            DateTime::parse("2026-04-05T14:30:00+05"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn parse_offset_bad_colon() {
        assert_eq!(
            DateTime::parse("2026-04-05T14:30:00+05X30"),
            Err(ParseError::InvalidTimezone)
        );
    }

    #[test]
    fn parse_offset_hours_out_of_range() {
        assert_eq!(
            DateTime::parse("2026-04-05T14:30:00+25:00"),
            Err(ParseError::InvalidTimezone)
        );
    }

    #[test]
    fn relative_one_month() {
        // ~31 days = 1 month bucket
        let a = DateTime::parse("2026-05-06T12:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        assert_eq!(a.relative_to(&b), "in 1 month");
    }

    #[test]
    fn relative_one_year() {
        let a = DateTime::parse("2027-04-05T12:00:00Z").unwrap();
        let b = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        assert_eq!(a.relative_to(&b), "in 1 year");
    }

    // ===============================================================
    // Leap year edge cases
    // ===============================================================

    #[test]
    fn century_non_leap() {
        assert_eq!(
            DateTime::parse("1900-02-29T00:00:00Z"),
            Err(ParseError::OutOfRange)
        );
    }

    #[test]
    fn quad_century_leap() {
        let dt = DateTime::parse("2000-02-29T00:00:00Z").unwrap();
        assert_eq!(dt.day(), 29);
    }

    // ===============================================================
    // Days-in-month boundaries
    // ===============================================================

    #[test]
    fn last_day_each_month() {
        let cases = [
            ("2026-01-31T00:00:00Z", 31),
            ("2026-03-31T00:00:00Z", 31),
            ("2026-04-30T00:00:00Z", 30),
            ("2026-05-31T00:00:00Z", 31),
            ("2026-06-30T00:00:00Z", 30),
            ("2026-07-31T00:00:00Z", 31),
            ("2026-08-31T00:00:00Z", 31),
            ("2026-09-30T00:00:00Z", 30),
            ("2026-10-31T00:00:00Z", 31),
            ("2026-11-30T00:00:00Z", 30),
            ("2026-12-31T00:00:00Z", 31),
        ];
        for (input, expected_day) in &cases {
            let dt = DateTime::parse(input).unwrap();
            assert_eq!(dt.day(), *expected_day, "Failed for {input}");
        }
    }

    // ===============================================================
    // DateTime::now
    // ===============================================================

    #[test]
    fn now_returns_reasonable_year() {
        let dt = DateTime::now();
        assert!(dt.year() >= 2024);
        assert!(dt.year() <= 2100);
    }

    #[test]
    fn now_is_utc() {
        assert_eq!(DateTime::now().offset_minutes(), 0);
    }

    // ===============================================================
    // DateTime::from_unix_timestamp / roundtrip
    // ===============================================================

    #[test]
    fn from_unix_epoch_zero() {
        let dt = DateTime::from_unix_timestamp(0);
        assert_eq!(dt.year(), 1970);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);
        assert_eq!(dt.hour(), 0);
    }

    #[test]
    fn from_unix_known_date() {
        // 2026-04-05T12:00:00Z
        let dt = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let ts = dt.to_unix_timestamp();
        let rt = DateTime::from_unix_timestamp(ts);
        assert_eq!(rt.year(), 2026);
        assert_eq!(rt.month(), 4);
        assert_eq!(rt.day(), 5);
        assert_eq!(rt.hour(), 12);
    }

    #[test]
    fn from_unix_negative_timestamp() {
        // 1969-12-31T23:59:00Z = -60
        let dt = DateTime::from_unix_timestamp(-60);
        assert_eq!(dt.year(), 1969);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 31);
        assert_eq!(dt.hour(), 23);
        assert_eq!(dt.minute(), 59);
    }

    #[test]
    fn from_unix_roundtrip() {
        let dt = DateTime::parse("2026-06-15T08:30:45Z").unwrap();
        let rt = DateTime::from_unix_timestamp(dt.to_unix_timestamp());
        assert_eq!(dt.to_iso8601(), rt.to_iso8601());
    }

    // ===============================================================
    // DateTime::add_seconds / add_hours / add_days
    // ===============================================================

    #[test]
    fn add_seconds_positive() {
        let dt = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let dt2 = dt.add_seconds(3600);
        assert_eq!(dt2.hour(), 13);
    }

    #[test]
    fn add_seconds_negative() {
        let dt = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
        let dt2 = dt.add_seconds(-3600);
        assert_eq!(dt2.hour(), 11);
    }

    #[test]
    fn add_hours_crosses_day() {
        let dt = DateTime::parse("2026-04-05T23:00:00Z").unwrap();
        let dt2 = dt.add_hours(2);
        assert_eq!(dt2.day(), 6);
        assert_eq!(dt2.hour(), 1);
    }

    #[test]
    fn add_days_crosses_month() {
        let dt = DateTime::parse("2026-01-30T12:00:00Z").unwrap();
        let dt2 = dt.add_days(3);
        assert_eq!(dt2.month(), 2);
        assert_eq!(dt2.day(), 2);
    }

    // ===============================================================
    // TryFrom<&str>
    // ===============================================================

    #[test]
    fn try_from_str_valid() {
        let dt: Result<DateTime, _> = "2026-04-05T14:30:00Z".try_into();
        assert!(dt.is_ok());
        assert_eq!(dt.unwrap().year(), 2026);
    }

    #[test]
    fn try_from_str_invalid() {
        let dt: Result<DateTime, _> = "bad".try_into();
        assert!(dt.is_err());
    }

    // ===============================================================
    // From<SystemTime>
    // ===============================================================

    #[test]
    fn from_system_time() {
        let st = std::time::SystemTime::now();
        let dt: DateTime = st.into();
        assert!(dt.year() >= 2024);
    }
}
