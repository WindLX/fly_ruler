extern crate fly_ruler_utils;

use criterion::{criterion_group, criterion_main, Criterion};
use fly_ruler_utils::{
    algorithm::nelder_mead::{nelder_mead, NelderMeadOptions},
    model::Vector,
};

fn nm() {
    let func = |x: &Vector| {
        return 100.0 * (x[1] - x[0].powi(2)).powi(2) + (1.0 - x[1]).powi(2);
    };
    let x_0 = Vector::from(vec![-1.2, 1.0]);
    let options = NelderMeadOptions {
        max_fun_evals: 500,
        max_iter: 100,
        tol_fun: 1e-6,
        tol_x: 1e-6,
    };
    let _result = nelder_mead(Box::new(func), x_0, Some(options));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nm", |b| b.iter(|| nm()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
