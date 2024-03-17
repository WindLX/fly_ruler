extern crate fly_ruler_core;
extern crate fly_ruler_utils;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fly_ruler_core::parts::{
    block::PlaneBlock,
    flight::MechanicalModel,
    trim::{trim, TrimTarget},
};
use fly_ruler_plugin::{AerodynamicModel, AsPlugin};
use fly_ruler_utils::plane_model::ControlLimit;
use std::{sync::Arc, time::Instant};

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

fn step(plane: Arc<std::sync::Mutex<PlaneBlock>>, start_time: Instant) {
    let current_time = Instant::now();
    let delta_time = current_time.duration_since(start_time);
    let _result = plane
        .lock()
        .unwrap()
        .update([0.0, 0.0, 0.0, 0.0], delta_time.as_secs_f64())
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("core");
    let model = AerodynamicModel::new("../../../lua_system/model/f16_model");
    let model = model.unwrap();
    let _res = model
        .plugin()
        .install(&["../../../lua_system/model/f16_model/data"]);
    let plane = Arc::new(std::sync::Mutex::new(MechanicalModel::new(&model).unwrap()));
    let trim_target = TrimTarget::new(15000.0, 500.0);
    let trim_output = trim(plane, trim_target, None, CL, None, None).unwrap();
    let plane_block = Arc::new(std::sync::Mutex::new(
        PlaneBlock::new(&model, &trim_output, &[0.0, 0.0, 0.0], CL, 0.0).unwrap(),
    ));
    group.bench_function("plane", |b| {
        b.iter(|| step(black_box(plane_block.clone()), black_box(Instant::now())))
    });
    let _res = model.plugin().uninstall();
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
