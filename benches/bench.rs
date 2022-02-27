use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing_n::vector::Vector;

fn criterion_benchmark(c: &mut Criterion) {
    {
        let vector = Vector::new([5_f64, -99.45_f64, 89342342.3454534_f64]);
        c.bench_function("norm", |b| b.iter(|| vector.norm()));
        c.bench_function("par_norm", |b| b.iter(|| vector.par_norm()));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
