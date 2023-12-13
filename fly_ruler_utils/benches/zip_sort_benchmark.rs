extern crate fly_ruler_utils;

use criterion::{criterion_group, criterion_main, Criterion};
use fly_ruler_utils::model::{Matrix, Vector};
use rand::Rng;

fn generate_random_vector(dim: usize) -> Vector {
    let mut rng = rand::thread_rng();
    let v: Vec<_> = (0..dim).map(|_| rng.gen_range(0.0..100.0)).collect();
    Vector::from(v)
}

fn generate_random_matrix(rows: usize, cols: usize) -> Matrix {
    let mut rng = rand::thread_rng();
    let m: Vec<Vec<_>> = (0..rows)
        .map(|_| (0..cols).map(|_| rng.gen_range(0.0..100.0)).collect())
        .collect();
    Matrix::from(m)
}

fn zip_sort() {
    let mut v = generate_random_vector(100);
    let m = generate_random_matrix(100, 100);
    let _m = v.zip_sort(&m);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("zip_sort", |b| b.iter(|| zip_sort()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
