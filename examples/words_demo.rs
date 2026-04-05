// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Demonstrates the Words module's full API.

use cmn::words::WORD_LIST;
use cmn::Words;

fn main() {
    println!("=== Built-in Word List ===\n");

    println!("WORD_LIST entries: {}", WORD_LIST.len());
    println!("First 5: {:?}", &WORD_LIST[..5]);

    println!("\n=== Words::default() ===\n");

    let words = Words::default();
    println!("Count:  {}", words.count());

    let list = words.words_list();
    println!(
        "Sorted: first={}, last={}",
        list[0],
        list[list.len() - 1]
    );

    println!("\n=== Words::new() + manual ops ===\n");

    let mut w = Words::new();
    println!("Empty count: {}", w.count());

    w.add_word("rust");
    w.add_word("cargo");
    w.add_word("crate");
    w.add_word("rust"); // duplicate
    println!("After adds:  {} (dup ignored)", w.count());
    println!("Contains 'rust': {}", w.contains("rust"));
    println!("Contains 'go':   {}", w.contains("go"));

    let removed = w.remove_word("cargo");
    println!("Removed 'cargo': {removed}");
    println!("Count after:     {}", w.count());

    println!("Sorted list:     {:?}", w.words_list());

    w.clear();
    println!("After clear:     {}", w.count());

    println!("\n=== FromIterator ===\n");

    let items = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
    ];
    let collected: Words = items.into_iter().collect();
    println!("Collected: {:?}", collected.words_list());

    println!("\n=== Serde Roundtrip ===\n");

    let mut original = Words::new();
    original.add_word("hello");
    original.add_word("world");

    let json = serde_json::to_string(&original).unwrap();
    println!("JSON:        {json}");

    let restored: Words = serde_json::from_str(&json).unwrap();
    println!("Restored:    {:?}", restored.words_list());
    println!("Match:       {}", restored.count() == original.count());
}
