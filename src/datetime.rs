// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Lightweight datetime utilities for ISO 8601 parsing,
//! relative time formatting, and duration calculations.
//!
//! No external dependencies — uses only `std::time`.
//!
//! # Quick Start
//!
//! ```
//! use cmn::datetime::DateTime;
//!
//! // Parse ISO 8601
//! let dt = DateTime::parse("2026-04-05T14:30:00Z").unwrap();
//! assert_eq!(dt.year(), 2026);
//! assert_eq!(dt.month(), 4);
//! assert_eq!(dt.day(), 5);
//! assert_eq!(dt.hour(), 14);
//! assert_eq!(dt.minute(), 30);
//!
//! // Format back to ISO 8601
//! assert_eq!(dt.to_iso8601(), "2026-04-05T14:30:00Z");
//!
//! // Duration between two datetimes
//! let dt2 = DateTime::parse("2026-04-05T16:30:00Z").unwrap();
//! let dur = dt2.duration_since(&dt);
//! assert_eq!(dur.whole_seconds(), 7200);
//! assert_eq!(dur.whole_hours(), 2);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// A date and time with optional UTC offset.
///
/// Supports year 0000–9999, UTC (`Z`) and fixed offsets
/// (`+HH:MM` / `-HH:MM`). All arithmetic is performed in
/// seconds relative to the Unix epoch (1970-01-01T00:00:00Z).
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct DateTime {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    /// UTC offset in minutes (e.g. +05:30 = 330, Z = 0).
    offset_minutes: i16,
}

/// A signed duration between two [`DateTime`] values.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Duration {
    /// Total seconds (signed).
    seconds: i64,
}

/// Errors returned by [`DateTime::parse`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// The input string is too short or structurally invalid.
    InvalidFormat,
    /// A numeric field is out of range (e.g. month 13).
    OutOfRange,
    /// The timezone suffix is malformed.
    InvalidTimezone,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => {
                write!(f, "invalid ISO 8601 format")
            }
            Self::OutOfRange => {
                write!(f, "date/time field out of range")
            }
            Self::InvalidTimezone => {
                write!(f, "invalid timezone offset")
            }
        }
    }
}

impl std::error::Error for ParseError {}

// ------------------------------------------------------------------
// DateTime
// ------------------------------------------------------------------

impl DateTime {
    /// Creates a new `DateTime` from individual components.
    ///
    /// Returns `None` if any field is out of range.
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset_minutes: i16,
    ) -> Option<Self> {
        if !(1..=12).contains(&month)
            || day == 0
            || day > days_in_month(year, month)
            || hour > 23
            || minute > 59
            || second > 59
            || !(-1440..=1440).contains(&offset_minutes)
        {
            return None;
        }
        Some(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            offset_minutes,
        })
    }

    /// Parses an ISO 8601 datetime string.
    ///
    /// Accepted formats:
    /// - `YYYY-MM-DDTHH:MM:SSZ`
    /// - `YYYY-MM-DDTHH:MM:SS+HH:MM`
    /// - `YYYY-MM-DDTHH:MM:SS-HH:MM`
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let b = input.as_bytes();
        if b.len() < 20 {
            return Err(ParseError::InvalidFormat);
        }
        let year =
            parse_u32(b, 0, 4).ok_or(ParseError::InvalidFormat)? as i32;
        if b[4] != b'-' {
            return Err(ParseError::InvalidFormat);
        }
        let month =
            parse_u32(b, 5, 2).ok_or(ParseError::InvalidFormat)? as u8;
        if b[7] != b'-' {
            return Err(ParseError::InvalidFormat);
        }
        let day =
            parse_u32(b, 8, 2).ok_or(ParseError::InvalidFormat)? as u8;
        if b[10] != b'T' {
            return Err(ParseError::InvalidFormat);
        }
        let hour =
            parse_u32(b, 11, 2).ok_or(ParseError::InvalidFormat)? as u8;
        if b[13] != b':' {
            return Err(ParseError::InvalidFormat);
        }
        let minute =
            parse_u32(b, 14, 2).ok_or(ParseError::InvalidFormat)? as u8;
        if b[16] != b':' {
            return Err(ParseError::InvalidFormat);
        }
        let second =
            parse_u32(b, 17, 2).ok_or(ParseError::InvalidFormat)? as u8;

        let offset_minutes = match b[19] {
            b'Z' => {
                if b.len() != 20 {
                    return Err(ParseError::InvalidFormat);
                }
                0i16
            }
            b'+' | b'-' => {
                if b.len() != 25 {
                    return Err(ParseError::InvalidFormat);
                }
                let oh = parse_u32(b, 20, 2)
                    .ok_or(ParseError::InvalidTimezone)?
                    as i16;
                if b[22] != b':' {
                    return Err(ParseError::InvalidTimezone);
                }
                let om = parse_u32(b, 23, 2)
                    .ok_or(ParseError::InvalidTimezone)?
                    as i16;
                if oh > 23 || om > 59 {
                    return Err(ParseError::InvalidTimezone);
                }
                let total = oh * 60 + om;
                if b[19] == b'-' {
                    -total
                } else {
                    total
                }
            }
            _ => return Err(ParseError::InvalidTimezone),
        };

        DateTime::new(
            year,
            month,
            day,
            hour,
            minute,
            second,
            offset_minutes,
        )
        .ok_or(ParseError::OutOfRange)
    }

    // -- Accessors --

    /// Returns the year.
    pub fn year(&self) -> i32 {
        self.year
    }
    /// Returns the month (1–12).
    pub fn month(&self) -> u8 {
        self.month
    }
    /// Returns the day of the month (1–31).
    pub fn day(&self) -> u8 {
        self.day
    }
    /// Returns the hour (0–23).
    pub fn hour(&self) -> u8 {
        self.hour
    }
    /// Returns the minute (0–59).
    pub fn minute(&self) -> u8 {
        self.minute
    }
    /// Returns the second (0–59).
    pub fn second(&self) -> u8 {
        self.second
    }
    /// Returns the UTC offset in minutes.
    pub fn offset_minutes(&self) -> i16 {
        self.offset_minutes
    }

    // -- Formatting --

    /// Formats as ISO 8601 (e.g. `2026-04-05T14:30:00Z`).
    pub fn to_iso8601(&self) -> String {
        if self.offset_minutes == 0 {
            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                self.year,
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second,
            )
        } else {
            let sign = if self.offset_minutes >= 0 { '+' } else { '-' };
            let abs = self.offset_minutes.unsigned_abs();
            let oh = abs / 60;
            let om = abs % 60;
            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}\
                 {}{:02}:{:02}",
                self.year,
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second,
                sign,
                oh,
                om,
            )
        }
    }

    /// Converts this datetime to a Unix timestamp (seconds
    /// since 1970-01-01T00:00:00Z), adjusted for offset.
    pub fn to_unix_timestamp(&self) -> i64 {
        let days = days_from_civil(self.year, self.month, self.day);
        let secs = days * 86400
            + i64::from(self.hour) * 3600
            + i64::from(self.minute) * 60
            + i64::from(self.second);
        secs - i64::from(self.offset_minutes) * 60
    }

    /// Returns the current UTC datetime from the system clock.
    pub fn now() -> Self {
        let dur = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        Self::from_unix_timestamp(dur.as_secs() as i64)
    }

    /// Creates a `DateTime` from a Unix timestamp (UTC).
    pub fn from_unix_timestamp(ts: i64) -> Self {
        let (days, rem) = if ts >= 0 {
            (ts / 86400, ts % 86400)
        } else {
            let d = (ts + 1) / 86400 - 1;
            (d, ts - d * 86400)
        };
        let (y, m, d) = civil_from_days(days);
        let hour = (rem / 3600) as u8;
        let minute = ((rem % 3600) / 60) as u8;
        let second = (rem % 60) as u8;
        Self {
            year: y,
            month: m,
            day: d,
            hour,
            minute,
            second,
            offset_minutes: 0,
        }
    }

    /// Returns a new `DateTime` offset by the given seconds.
    pub fn add_seconds(&self, secs: i64) -> Self {
        Self::from_unix_timestamp(self.to_unix_timestamp() + secs)
    }

    /// Returns a new `DateTime` offset by the given hours.
    pub fn add_hours(&self, hours: i64) -> Self {
        self.add_seconds(hours * 3600)
    }

    /// Returns a new `DateTime` offset by the given days.
    pub fn add_days(&self, days: i64) -> Self {
        self.add_seconds(days * 86400)
    }

    /// Returns the signed [`Duration`] from `other` to `self`.
    pub fn duration_since(&self, other: &Self) -> Duration {
        Duration {
            seconds: self.to_unix_timestamp()
                - other.to_unix_timestamp(),
        }
    }

    /// Formats the duration between `self` and `other` as a
    /// human-readable relative string (e.g. "3 hours ago",
    /// "in 2 days").
    pub fn relative_to(&self, other: &Self) -> String {
        let d = self.duration_since(other);
        let abs = d.seconds.unsigned_abs();
        let past = d.seconds < 0;

        let label = if abs < 60 {
            format!("{abs} seconds")
        } else if abs < 3600 {
            let m = abs / 60;
            if m == 1 {
                "1 minute".to_string()
            } else {
                format!("{m} minutes")
            }
        } else if abs < 86400 {
            let h = abs / 3600;
            if h == 1 {
                "1 hour".to_string()
            } else {
                format!("{h} hours")
            }
        } else if abs < 2_592_000 {
            let d = abs / 86400;
            if d == 1 {
                "1 day".to_string()
            } else {
                format!("{d} days")
            }
        } else if abs < 31_536_000 {
            let mo = abs / 2_592_000;
            if mo == 1 {
                "1 month".to_string()
            } else {
                format!("{mo} months")
            }
        } else {
            let y = abs / 31_536_000;
            if y == 1 {
                "1 year".to_string()
            } else {
                format!("{y} years")
            }
        };

        if past {
            format!("{label} ago")
        } else if d.seconds == 0 {
            "just now".to_string()
        } else {
            format!("in {label}")
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_iso8601())
    }
}

impl TryFrom<&str> for DateTime {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

impl From<std::time::SystemTime> for DateTime {
    fn from(st: std::time::SystemTime) -> Self {
        let dur = st
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        Self::from_unix_timestamp(dur.as_secs() as i64)
    }
}

// ------------------------------------------------------------------
// Duration
// ------------------------------------------------------------------

impl Duration {
    /// Creates a `Duration` from a number of seconds.
    pub fn from_seconds(seconds: i64) -> Self {
        Self { seconds }
    }

    /// Total seconds (signed).
    pub fn whole_seconds(&self) -> i64 {
        self.seconds
    }

    /// Total whole minutes (truncated toward zero).
    pub fn whole_minutes(&self) -> i64 {
        self.seconds / 60
    }

    /// Total whole hours (truncated toward zero).
    pub fn whole_hours(&self) -> i64 {
        self.seconds / 3600
    }

    /// Total whole days (truncated toward zero).
    pub fn whole_days(&self) -> i64 {
        self.seconds / 86400
    }

    /// Returns `true` if the duration is negative.
    pub fn is_negative(&self) -> bool {
        self.seconds < 0
    }

    /// Returns the absolute value of this duration.
    pub fn abs(&self) -> Self {
        Self {
            seconds: self.seconds.abs(),
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abs = self.seconds.unsigned_abs();
        let h = abs / 3600;
        let m = (abs % 3600) / 60;
        let s = abs % 60;
        if self.seconds < 0 {
            write!(f, "-{h:02}:{m:02}:{s:02}")
        } else {
            write!(f, "{h:02}:{m:02}:{s:02}")
        }
    }
}

// ------------------------------------------------------------------
// Helpers
// ------------------------------------------------------------------

fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn days_in_month(y: i32, m: u8) -> u8 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(y) => 29,
        2 => 28,
        // Month is validated before this function is called;
        // returning 0 triggers the day > days_in_month check
        // in DateTime::new, preventing construction.
        _ => 0,
    }
}

/// Converts a day count (since Unix epoch) to (year, month, day).
/// Inverse of `days_from_civil` (Howard Hinnant algorithm).
fn civil_from_days(z: i64) -> (i32, u8, u8) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // 0..=146096
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // 0..=399
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153; // 0..=11
    let d = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
    let y = if m <= 2 { y + 1 } else { y } as i32;
    (y, m, d)
}

/// Days from 0000-01-01 to the given civil date
/// (algorithm from Howard Hinnant).
fn days_from_civil(y: i32, m: u8, d: u8) -> i64 {
    let y = i64::from(y);
    let m = i64::from(m);
    let d = i64::from(d);

    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let m_adj = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * m_adj as u64 + 2) / 5 + d as u64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i64 - 719468
}

/// Parse `len` ASCII digits starting at `offset`.
fn parse_u32(b: &[u8], offset: usize, len: usize) -> Option<u32> {
    if offset + len > b.len() {
        return None;
    }
    let mut val = 0u32;
    for &ch in &b[offset..offset + len] {
        if !ch.is_ascii_digit() {
            return None;
        }
        val = val * 10 + u32::from(ch - b'0');
    }
    Some(val)
}

#[cfg(test)]
mod internal_tests {
    #[test]
    fn days_in_month_invalid_returns_zero() {
        assert_eq!(super::days_in_month(2026, 0), 0);
        assert_eq!(super::days_in_month(2026, 13), 0);
    }

    #[test]
    fn parse_u32_out_of_bounds() {
        assert!(super::parse_u32(b"12", 0, 5).is_none());
        assert!(super::parse_u32(b"12", 3, 1).is_none());
    }
}
