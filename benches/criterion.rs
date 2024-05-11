// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks for the Common (CMN) library.

#![allow(missing_docs)]

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};

pub use cmn::Common;
pub use cmn::Constants;
pub use cmn::Words;

#[allow(unused_results)]
fn bench_cmn(c: &mut Criterion) {
    c.bench_function("common", |b| {
        b.iter(|| {
            let common = black_box(Common::default());
            black_box(common.constants());
            black_box(common.words());
        })
    });
}

#[allow(unused_results)]
fn bench_words(c: &mut Criterion) {
    c.bench_function("Words::new", |b| {
        b.iter(|| {
            let _words = black_box(Words::new());
        })
    });

    c.bench_function("Words::default", |b| {
        b.iter(|| {
            let _words = black_box(Words::default());
        })
    });
}

criterion_group!(benches, bench_cmn, bench_words);
criterion_main!(benches);
