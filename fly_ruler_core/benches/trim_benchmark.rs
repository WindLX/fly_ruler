extern crate fly_ruler_core;
extern crate fly_ruler_utils;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fly_ruler_core::{trim::trim, TrimTarget};
use fly_ruler_plugin::{model::Model, plugin::IsPlugin};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

fn tr(model: Arc<Mutex<Model>>) {
    let trim_target = TrimTarget::new(15000.0, 500.0);
    let _result = tokio_test::block_on(trim(trim_target, None, true, model, None, None));
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("core");
    group.measurement_time(Duration::from_secs(30));
    let model = Model::new("./install");
    let model = model.unwrap();
    let _res = model.plugin().install(vec![Box::new("./data")]);
    let model = Arc::new(Mutex::new(model));
    group.bench_function("trim", |b| b.iter(|| tr(black_box(model.clone()))));
    let model = Arc::into_inner(model).unwrap();
    let _res = tokio_test::block_on(model.lock())
        .plugin()
        .uninstall(Vec::new());
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
