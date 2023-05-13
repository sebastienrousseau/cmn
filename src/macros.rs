// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Macros for the cmn crate.
//!
//! This module bundles all macros used across the cmn crate.
//! These include macros for validating input, and macros for
//! generating the Common struct.
//!
//! ## Generic macros for the cmn crate.
//!
//! This crate provides the following macros:
//!
//! | Macro | Description |
//! |--------|------------|
//! | `cmn` | The main macro for the cmn crate. It takes any number of arguments and parses them into a Rust value. |
//! | `cmn_assert` | Checks if the given expression is true. |
//! | `cmn_contains` | Checks if the given string contains the given substring. |
//! | `cmn_in_range` | Checks if the given value is in the given range. |
//! | `cmn_join` | Joins a vector of strings into a single string. |
//! | `cmn_map` | Creates a new map of the given key-value pairs. |
//! | `cmn_max` | Returns the maximum of the given values. |
//! | `cmn_min` | Returns the minimum of the given values. |
//! | `cmn_parse` | Parses the given input into a Rust value. |
//! | `cmn_print_vec` | Prints a vector of elements to the console. |
//! | `cmn_print` | Prints the arguments to the console. |
//! | `cmn_split` | Splits a string into a vector of words. |
//! | `cmn_to_num` | Converts the given string to a number. |
//! | `cmn_vec` | Creates a new vector of the given elements. |
//!

/// This macro takes any number of arguments and parses them into a
/// Rust value.
#[macro_export]
macro_rules! cmn {
    ($($tt:tt)*) => {
        // Parse the arguments into a Rust value.
        $crate::parse!($($tt)*)
    };
}

/// This macro asserts that the given condition is true. If the
/// condition is false, the macro panics with the given message.
#[macro_export]
macro_rules! cmn_assert {
    ($($arg:tt)*) => {
        if !$($arg)* {
            panic!("Assertion failed!");
        }
    };
}

/// This macro checks if the given string contains the given substring.
#[macro_export]
macro_rules! cmn_contains {
    ($s:expr, $sub:expr) => {
        $s.contains($sub)
    };
}

/// This macro checks if the given value is within the given range.
#[macro_export]
macro_rules! cmn_in_range {
    ($value:expr, $min:expr, $max:expr) => {
        if $value >= $min && $value <= $max {
            true
        } else {
            false
        }
    };
}

/// This macro joins the given strings together with the given separator.
#[macro_export]
macro_rules! cmn_join {
    ($($s:expr),*) => {{
        let mut s = String::new();
        $(
            s += &$s;
        )*
        s
    }};
}

/// This macro finds the maximum value of the given values.
#[macro_export]
macro_rules! cmn_max {
    ($x:expr $(, $y:expr)*) => {{
        let mut max = $x;
        $(
            if max < $y { max = $y; }
        )*
        max
    }};
}

/// This macro finds the minimum value of the given values.
#[macro_export]
macro_rules! cmn_min {
    ($x:expr $(, $y:expr)*) => {{
        let mut min = $x;
        $(
            if min > $y { min = $y; }
        )*
        min
    }};
}

/// This macro prints the given arguments to the console.
#[macro_export]
macro_rules! cmn_print {
    ($($arg:tt)*) => {
        println!("{}", format_args!("{}", $($arg)*));
    };
}

/// This macro prints the given vector of values to the console.
#[macro_export]
macro_rules! cmn_print_vec {
    ($($v:expr),*) => {{
        for v in $($v),* {
            println!("{}", v);
        }
    }};
}

/// This macro splits the given string into a vector of strings.
#[macro_export]
macro_rules! cmn_split {
    ($s:expr) => {{
        let mut v = Vec::new();
        for w in $s.split_whitespace() {
            v.push(w.to_string());
        }
        v
    }};
}

/// This macro creates a new vector with the given elements.
#[macro_export]
macro_rules! cmn_vec {
    ($($elem:expr),*) => {{
        let mut v = Vec::new();
        $(v.push($elem);)*
        v
    }};
}

/// This macro parses the given input into a Rust value.
#[macro_export]
macro_rules! cmn_parse {
    ($input:expr) => {
        Common::parse($input)
    };
}
