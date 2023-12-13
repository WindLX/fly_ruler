extern crate fly_ruler_utils;

use criterion::{criterion_group, criterion_main, Criterion};
use fly_ruler_utils::{
    algorithm::runge_kutta::runge_kutta,
    model::{Matrix, Vector},
};

fn rk() {
    let func = |x: &Vector, y: &Vector| {
        return y.clone() - x.clone() * x.clone() + 1.0;
    };

    let analytic_func = |x: &Vector| {
        let temp = x.map(|xx| xx.exp());
        return (x.clone() + Vector::from(vec![1.0])) * (x.clone() + Vector::from(vec![1.0]))
            - temp * 0.5;
    };

    let y_0 = Vector::from(vec![0.5]);
    let x_span = (Vector::from(vec![0.0]), Vector::from(vec![1.0]));
    let n = 11;

    let result = runge_kutta(Box::new(func), y_0, x_span.clone(), n);

    let x_eval = Matrix::linespace(&x_span.0, &x_span.1, n);
    let mut analytic_result = Vec::new();
    let mut err = Vec::new();
    for i in 0..x_eval.dim() {
        let res = analytic_func(&x_eval[i]);
        err.push((res.clone() - result[i].clone()).norm());
        analytic_result.push(res);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rk", |b| b.iter(|| rk()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
