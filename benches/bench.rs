use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing_n::vector::Vector;

fn criterion_benchmark(c: &mut Criterion) {
    {
        let vector = Vector::new([5_f64, -99.45_f64, 89342342.3454534_f64]);
        c.bench_function("norm", |b| b.iter(|| vector.norm()));
        c.bench_function("par_norm", |b| b.iter(|| vector.par_norm()));

        let v = Vector::new([1_f64; 1_000]);
        c.bench_function("negative - 1K", |b| b.iter(|| -v));
        c.bench_function("negative old - 1K", |b| b.iter(|| v.old_neg()));

        let v = Vector::new([1_f64; 2_000]);
        c.bench_function("negative - 2K", |b| b.iter(|| -v));
        c.bench_function("negative old - 2K", |b| b.iter(|| v.old_neg()));

        let v = Vector::new([1_f64; 5_000]);
        c.bench_function("negative - 5K", |b| b.iter(|| -v));
        c.bench_function("negative old - 5k", |b| b.iter(|| v.old_neg()));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
