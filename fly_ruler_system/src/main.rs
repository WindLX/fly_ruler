use fly_ruler_core::{
    algorithm::nelder_mead::NelderMeadOptions, core::Core, parts::trim::TrimTarget,
};
use fly_ruler_plugin::{IsPlugin, Model};
use fly_ruler_system::controller::StickController;
use fly_ruler_utils::{plant_model::Control, Command, IsController};
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
    let fi_flag = true;
    let nm_options = Some(NelderMeadOptions {
        max_fun_evals: 50000,
        max_iter: 10000,
        tol_fun: 1e-6,
        tol_x: 1e-6,
    });

    let mut core = Core::new();
    let _ = core
        .push_plant(
            Some(Duration::from_millis(100)),
            None,
            model.clone(),
            &[0.0, 0.0, 0.0],
            trim_target,
            trim_init,
            fi_flag,
            None,
            nm_options.clone(),
        )
        .await;

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
                Command::Control(control) => {
                    // info!("\n{:?}", &control);
                    let r = core.step(&[control]).await;
                    info!("\n{:#?}", &r.unwrap().unwrap().get(&0));
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
