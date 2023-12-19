use fly_ruler_core::{
    algorithm::nelder_mead::NelderMeadOptions,
    core::{Core, CoreInitData},
    parts::trim::TrimTarget,
};
use fly_ruler_plugin::{IsPlugin, Model};
use fly_ruler_system::controller::StickController;
use fly_ruler_utils::{
    plant_model::{Control, ControlLimit},
    Command, IsController,
};
use log::info;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();
    // env_logger::init();
    let model = Model::new("./install").unwrap();
    model
        .plugin()
        .install(vec![Box::new("./data")])
        .unwrap()
        .unwrap();

    let model = Arc::new(Mutex::new(model));

    let trim_target = TrimTarget::new(15000.0, 500.0);
    let trim_init = None;
    let nm_options = Some(NelderMeadOptions {
        max_fun_evals: 50000,
        max_iter: 10000,
        tol_fun: 1e-6,
        tol_x: 1e-6,
    });

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

    let core_init = CoreInitData {
        sample_time: Some(100),
        time_scale: None,
        ctrl_limit: CL,
        deflection: [0.0, 0.0, 0.0],
        trim_init,
        trim_target,
        flight_condition: None,
        optim_options: nm_options,
    };

    let mut core = Core::new();
    let _ = core.push_plant(model.clone(), core_init).await;

    let mut controller = StickController::new(0, Control::default()).await;
    let rx = controller.get_receiver();

    // let h1 = tokio::spawn(async move {
    //     loop {
    //         let c = controller.clone().send().await;
    //         dbg!(&c);
    //         if let Command::Exit = c {
    //             break;
    //         }
    //     }
    // });

    let h2 = tokio::spawn(async move {
        loop {
            let command = rx.clone().try_receive().await;
            match command {
                Command::Control(control, _) => {
                    info!("\n{}", &control);
                    let r = core.step(&[control]).await;
                    info!("\n{}", &r.unwrap().unwrap().get(&0).unwrap());
                }
                Command::Extra(s) => {
                    let r = core.step(&[Control::default()]).await;
                    // dbg!(&r.unwrap().unwrap().get(&0));
                }
                Command::Exit => break,
            }
        }
    });

    // let r = tokio::join!(h1);
    loop {
        let r = controller.send().await;
    }
    let model = Arc::into_inner(model).unwrap();
    model
        .lock()
        .await
        .plugin()
        .uninstall(Vec::new())
        .unwrap()
        .unwrap();
}
