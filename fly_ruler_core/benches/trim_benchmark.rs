extern crate fly_ruler_core;
extern crate fly_ruler_utils;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fly_ruler_core::parts::{
    flight::MechanicalModel,
    trim::{trim, TrimTarget},
};
use fly_ruler_plugin::{AerodynamicModel, IsPlugin};
use fly_ruler_utils::plant_model::ControlLimit;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

const CL: ControlLimit = ControlLimit {
    thrust_cmd_limit_top: 19000.0,
    thrust_cmd_limit_bottom: 1000.0,
    thrust_rate_limit: 10000.0,
    ele_cmd_limit_top: 25.0,
    ele_cmd_limit_bottom: -25.0,
    ele_rate_limit: 60.0,
    ail_cmd_limit_top: 21.5,
    ail_cmd_limit_bottom: -21.5,
    ail_rate_limit: 80.0,
    rud_cmd_limit_top: 30.0,
    rud_cmd_limit_bottom: -30.0,
    rud_rate_limit: 120.0,
    alpha_limit_top: 45.0,
    alpha_limit_bottom: -20.0,
    beta_limit_top: 30.0,
    beta_limit_bottom: -30.0,
};

fn tr(plant: Arc<std::sync::Mutex<MechanicalModel>>) {
    let trim_target = TrimTarget::new(15000.0, 500.0);
    let _result = trim(plant, trim_target, None, CL, None, None);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("core");
    group.measurement_time(Duration::from_secs(30));
    let model = AerodynamicModel::new("../plugins/model/f16_model");
    let model = model.unwrap();
    let _res = model.plugin().install(&["../plugins/model/f16_model/data"]);
    let model = Arc::new(Mutex::new(model));
    let plant = Arc::new(std::sync::Mutex::new(
        tokio_test::block_on(MechanicalModel::new(model.clone())).unwrap(),
    ));
    group.bench_function("trim", |b| b.iter(|| tr(black_box(plant.clone()))));
    let model = Arc::into_inner(model).unwrap();
    let _res = tokio_test::block_on(model.lock())
        .plugin()
        .uninstall(&Vec::<String>::new());
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
