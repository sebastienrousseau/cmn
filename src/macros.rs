// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utility macros that eliminate common Rust boilerplate.
//!
//! Macros marked **no_std** work without the `std` feature.
//! Macros marked **std** require the `std` feature.
//!
//! ## Available macros
//!
//! | Macro | Requires | Description |
//! |--------|----------|------------|
//! | `cmn_assert` | no_std | Checks if the given expression is true. |
//! | `cmn_contains` | no_std | Checks if the given string contains the given substring. |
//! | `cmn_in_range` | no_std | Checks if the given value is in the given range. |
//! | `cmn_max` | no_std | Returns the maximum of the given values. |
//! | `cmn_min` | no_std | Returns the minimum of the given values. |
//! | `cmn_to_num` | no_std | Converts the given string to a number. |
//! | `cmn_constants` | no_std | Defines a set of `f64` constants. |
//! | `cmn` | std | Parses arguments via `Common::parse`. |
//! | `cmn_parse` | std | Parses the given input into a Rust value. |
//! | `cmn_join` | std | Joins strings together. |
//! | `cmn_split` | std | Splits a string into a vector of words. |
//! | `cmn_vec` | std | Creates a new vector with the given elements. |
//! | `cmn_map` | std | Creates a new map of the given key-value pairs. |
//! | `cmn_print` | std | Prints the arguments to the console. |
//! | `cmn_print_vec` | std | Prints a vector of elements to the console. |

// ---------------------------------------------------------------
// no_std-compatible macros (6)
// ---------------------------------------------------------------

/// Asserts that the given condition is true.
/// Panics with "Assertion failed!" if false.
#[macro_export]
macro_rules! cmn_assert {
    ($($arg:tt)*) => {
        if !$($arg)* {
            panic!("Assertion failed!");
        }
    };
}

/// Checks if a string contains a substring.
#[macro_export]
macro_rules! cmn_contains {
    ($s:expr, $sub:expr) => {
        $s.contains($sub)
    };
}

/// Checks if a value is within an inclusive range.
#[macro_export]
macro_rules! cmn_in_range {
    ($value:expr, $min:expr, $max:expr) => {
        ($value >= $min && $value <= $max)
    };
}

/// Returns the maximum of the given values.
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

/// Returns the minimum of the given values.
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

/// Converts a string to `f64`, returning `0.0` on failure.
#[macro_export]
macro_rules! cmn_to_num {
    ($s:expr) => {
        $s.parse::<f64>().unwrap_or(0.0)
    };
}

/// Defines a set of `f64` constants.
#[macro_export]
macro_rules! cmn_constants {
    ($($name:ident = $value:expr),*) => {
        $(
            /// The value of the constant.
            pub const $name: f64 = $value;
        )*
    };
}

// ---------------------------------------------------------------
// std-only macros (8) — require String, Vec, HashMap, println
// ---------------------------------------------------------------

/// Parses arguments via `Common::parse`. Requires `std`.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! cmn {
    ($($tt:tt)*) => {
        cmn::Common::parse($($tt)*)
    };
}

/// Parses the given input into a `Common` value.
/// Requires `std`.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! cmn_parse {
    ($input:expr) => {
        Common::parse($input)
    };
}

/// Joins strings together into a single `String`.
/// Requires `std`.
#[cfg(feature = "std")]
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

/// Splits a string into a `Vec<String>` by whitespace.
/// Requires `std`.
#[cfg(feature = "std")]
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

/// Creates a new `Vec` with the given elements.
/// Requires `std`.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! cmn_vec {
    ($($elem:expr),*) => {{
        let mut v = Vec::new();
        $(v.push($elem);)*
        v
    }};
}

/// Creates a `HashMap` from key-value pairs.
/// Requires `std`.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! cmn_map {
    ($($key:expr => $value:expr),*) => {{
        let mut map =
            ::std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

/// Prints the arguments to stdout. Requires `std`.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! cmn_print {
    ($($arg:tt)*) => {
        println!("{}", format_args!("{}", $($arg)*));
    };
}

/// Prints each element of a slice to stdout.
/// Requires `std`.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! cmn_print_vec {
    ($($v:expr),*) => {{
        for v in $($v),* {
            println!("{}", v);
        }
    }};
}
