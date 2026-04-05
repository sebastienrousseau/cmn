// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Demonstrates runtime constant lookup and the CONSTANTS_TABLE.

use cmn::constants::{
    Category, ConstantValue, Constants, CONSTANTS_TABLE,
};

fn main() {
    println!("=== Runtime Typed Lookup ===\n");

    let c = Constants::new();

    // Float constant
    if let Some(ConstantValue::Float(pi)) = c.get_value("PI") {
        println!("PI (Float)         = {pi}");
    }

    // String constant
    if let Some(ConstantValue::String(algo)) =
        c.get_value("HASH_ALGORITHM")
    {
        println!("HASH_ALGORITHM     = {algo}");
    }

    // U32 constant
    if let Some(ConstantValue::U32(cost)) = c.get_value("HASH_COST") {
        println!("HASH_COST (u32)    = {cost}");
    }

    // Usize constant
    if let Some(ConstantValue::Usize(len)) = c.get_value("HASH_LENGTH")
    {
        println!("HASH_LENGTH (usize)= {len}");
    }

    // CharArray constant
    if let Some(ConstantValue::CharArray(chars)) =
        c.get_value("SPECIAL_CHARS")
    {
        println!("SPECIAL_CHARS      = {} chars", chars.len());
    }

    // Missing constant
    let missing = c.get_value("DOES_NOT_EXIST");
    println!("Missing constant   = {missing:?}");

    println!("\n=== Display Trait ===\n");

    let values = [
        ("PI", c.get_value("PI")),
        ("HASH_ALGORITHM", c.get_value("HASH_ALGORITHM")),
        ("HASH_COST", c.get_value("HASH_COST")),
    ];
    for (name, val) in &values {
        if let Some(v) = val {
            println!("{name:20} = {v}");
        }
    }

    println!("\n=== CONSTANTS_TABLE (no_std) ===\n");

    println!("Total float constants: {}", CONSTANTS_TABLE.len());

    let math: Vec<_> = CONSTANTS_TABLE
        .iter()
        .filter(|(_, _, cat)| *cat == Category::Mathematical)
        .collect();
    let phys: Vec<_> = CONSTANTS_TABLE
        .iter()
        .filter(|(_, _, cat)| *cat == Category::Physical)
        .collect();

    println!("Mathematical: {}", math.len());
    println!("Physical:     {}", phys.len());

    println!("\n--- First 5 mathematical ---");
    for (name, val, _) in &math[..5] {
        println!("  {name:20} = {val}");
    }

    println!("\n--- First 5 physical ---");
    for (name, val, _) in &phys[..5] {
        println!("  {name:20} = {val}");
    }

    println!("\n=== Constant Validation ===\n");

    println!("All valid: {}", c.is_valid());
    println!("Count:     {}", c.constants().len());
}
