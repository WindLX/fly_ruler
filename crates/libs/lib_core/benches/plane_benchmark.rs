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
use std::{cell::RefCell, rc::Rc, time::Instant};

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

fn step(plane: Rc<RefCell<PlaneBlock>>, start_time: Instant) {
    let mut count = 0;
    loop {
        count += 1;
        if count == 100 {
            break;
        }
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(start_time);
        let _result = plane
            .borrow_mut()
            .update([0.0, 0.0, 0.0, 0.0], delta_time.as_secs_f64())
            .unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("core");
    let model = AerodynamicModel::new("../../../LSE/models/f16_model");
    let model = model.unwrap();
    let _res = model
        .plugin()
        .install(&["../../../LSE/models/f16_model/data"]);
    let plane = Rc::new(RefCell::new(MechanicalModel::new(&model).unwrap()));
    let trim_target = TrimTarget::new(15000.0, 500.0, None, None);
    let trim_output = trim(plane, trim_target, None, CL, None, None).unwrap();
    let plane_block = Rc::new(RefCell::new(
        PlaneBlock::new("123", &model, &trim_output, &[0.0, 0.0, 0.0], CL).unwrap(),
    ));

    group.bench_function("plane", |b| {
        b.iter(|| step(black_box(plane_block.clone()), black_box(Instant::now())))
    });
    let _res = model.plugin().uninstall();
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
