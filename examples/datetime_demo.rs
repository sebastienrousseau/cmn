// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Demonstrates the datetime module's full API.

use cmn::datetime::{DateTime, Duration};

fn main() {
    println!("=== Parsing ISO 8601 ===\n");

    let dt = DateTime::parse("2026-04-05T14:30:00Z").unwrap();
    println!("Parsed:   {dt}");
    println!(
        "Year={} Month={} Day={} Hour={} Min={} Sec={}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second()
    );
    println!("Offset:   {} minutes", dt.offset_minutes());
    println!("Unix:     {}", dt.to_unix_timestamp());

    println!("\n=== Timezone Offsets ===\n");

    let utc = DateTime::parse("2026-04-05T12:00:00Z").unwrap();
    let plus530 = DateTime::parse("2026-04-05T17:30:00+05:30").unwrap();
    let minus8 = DateTime::parse("2026-04-05T04:00:00-08:00").unwrap();

    println!("UTC:      {utc}  (unix={})", utc.to_unix_timestamp());
    println!(
        "+05:30:   {plus530}  (unix={})",
        plus530.to_unix_timestamp()
    );
    println!(
        "-08:00:   {minus8}  (unix={})",
        minus8.to_unix_timestamp()
    );
    println!(
        "All same instant? {}",
        utc.to_unix_timestamp() == plus530.to_unix_timestamp()
            && utc.to_unix_timestamp() == minus8.to_unix_timestamp()
    );

    println!("\n=== DateTime::now() ===\n");

    let now = DateTime::now();
    println!("Now (UTC): {now}");

    println!("\n=== Arithmetic ===\n");

    let base = DateTime::parse("2026-04-05T10:00:00Z").unwrap();
    println!("Base:      {base}");
    println!("+3600s:    {}", base.add_seconds(3600));
    println!("+2 hours:  {}", base.add_hours(2));
    println!("+7 days:   {}", base.add_days(7));
    println!("-1 day:    {}", base.add_days(-1));

    println!("\n=== Duration ===\n");

    let a = DateTime::parse("2026-04-05T10:00:00Z").unwrap();
    let b = DateTime::parse("2026-04-07T14:30:00Z").unwrap();
    let dur = b.duration_since(&a);

    println!("A:         {a}");
    println!("B:         {b}");
    println!("Duration:  {dur}");
    println!("Seconds:   {}", dur.whole_seconds());
    println!("Hours:     {}", dur.whole_hours());
    println!("Days:      {}", dur.whole_days());

    let d = Duration::from_seconds(-7200);
    println!("\nNeg dur:   {d}");
    println!("Abs:       {}", d.abs());
    println!("Negative?  {}", d.is_negative());

    println!("\n=== Relative Time ===\n");

    let now_dt = DateTime::parse("2026-04-05T15:00:00Z").unwrap();
    let cases = [
        "2026-04-05T15:00:30Z",
        "2026-04-05T15:05:00Z",
        "2026-04-05T18:00:00Z",
        "2026-04-08T15:00:00Z",
        "2026-06-05T15:00:00Z",
        "2028-04-05T15:00:00Z",
    ];
    for s in &cases {
        let other = DateTime::parse(s).unwrap();
        println!("{s} => {}", now_dt.relative_to(&other));
    }

    println!("\n=== TryFrom / From ===\n");

    let from_str: DateTime = "2026-12-25T00:00:00Z".try_into().unwrap();
    println!("TryFrom:   {from_str}");

    let from_sys: DateTime = std::time::SystemTime::now().into();
    println!("From sys:  {from_sys}");
}
