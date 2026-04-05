// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Demonstrates all 15 macros in the CMN library.

use cmn::{
    cmn_assert, cmn_constants, cmn_contains, cmn_in_range, cmn_join,
    cmn_map, cmn_max, cmn_min, cmn_print, cmn_print_vec, cmn_split,
    cmn_to_num, cmn_vec,
};

fn main() {
    println!("=== no_std Macros ===\n");

    // cmn_max!
    let max = cmn_max!(3, 7, 2, 9, 1);
    println!("cmn_max!(3,7,2,9,1) = {max}");

    // cmn_min!
    let min = cmn_min!(3, 7, 2, 9, 1);
    println!("cmn_min!(3,7,2,9,1) = {min}");

    // cmn_in_range!
    println!("cmn_in_range!(5, 0, 10)  = {}", cmn_in_range!(5, 0, 10));
    println!("cmn_in_range!(15, 0, 10) = {}", cmn_in_range!(15, 0, 10));

    // cmn_contains!
    println!(
        "cmn_contains!(\"hello world\", \"world\") = {}",
        cmn_contains!("hello world", "world")
    );

    // cmn_to_num!
    let n: f64 = cmn_to_num!("42.5");
    let bad: f64 = cmn_to_num!("not_a_number");
    println!("cmn_to_num!(\"42.5\")    = {n}");
    println!("cmn_to_num!(\"bad\")     = {bad}");

    // cmn_assert!
    cmn_assert!(1 + 1 == 2);
    println!("cmn_assert!(1+1==2)    = passed");

    // cmn_constants!
    cmn_constants! {
        MY_PI = core::f64::consts::PI,
        MY_E  = core::f64::consts::E
    }
    println!("cmn_constants! MY_PI   = {MY_PI}");
    println!("cmn_constants! MY_E    = {MY_E}");

    println!("\n=== std Macros ===\n");

    // cmn_vec!
    let v = cmn_vec!(10, 20, 30);
    println!("cmn_vec!(10,20,30)     = {v:?}");

    // cmn_map!
    let m = cmn_map!("a" => 1, "b" => 2, "c" => 3);
    println!("cmn_map!               = {m:?}");

    // cmn_join!
    let joined = cmn_join!("Hello", " ", "World");
    println!("cmn_join!              = {joined}");

    // cmn_split!
    let parts = cmn_split!("one two three");
    println!("cmn_split!             = {parts:?}");

    // cmn_print!
    cmn_print!("cmn_print! output");

    // cmn_print_vec!
    print!("cmn_print_vec!: ");
    cmn_print_vec!(&[1, 2, 3]);
}
