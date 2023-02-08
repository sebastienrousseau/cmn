use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub use cmn::Common;
pub use cmn::Constants;
pub use cmn::Words;

fn bench_cmn(c: &mut Criterion) {
    c.bench_function("common", |b| {
        b.iter(|| {
            let common = black_box(Common::default());
            black_box(common.constants());
            black_box(common.words());
        });
    });
}
fn bench_words(c: &mut Criterion) {
    c.bench_function("Words::new", |b| {
        b.iter(|| {
            let words = black_box(Words::new());
            let _ = black_box(words.words_list());
        })
    });
    c.bench_function("Words::default", |b| {
        b.iter(|| {
            let words = black_box(Words::default());
            let _ = black_box(words.words_list());
        })
    });
}

criterion_group!(benches, bench_cmn, bench_words);
criterion_main!(benches);
