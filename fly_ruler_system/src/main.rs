use fly_ruler_core::{
    algorithm::nelder_mead::NelderMeadOptions,
    core::{Core, CoreInitCfg},
    parts::trim::TrimTarget,
};
use fly_ruler_plugin::{AerodynamicModel, IsPlugin, PluginManager, PluginType};
use fly_ruler_system::{config::ConfigManager, controller::StickController};
use fly_ruler_utils::{
    plant_model::{Control, ControlLimit},
    Command, IsController,
};
use log::{error, info};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();
    // env_logger::init();

    let config_manager = ConfigManager::new("../config");
    let core_init_cfg = config_manager.load_core_init().unwrap();
    info!("Core Init Config:\n{}", core_init_cfg);

    let mut plugin_manager = PluginManager::new("../plugins");
    let r = plugin_manager
        .install(
            PluginType::Model,
            0,
            vec![Box::new("../plugins/model/f16_model/data")],
        )
        .await;

    let model = plugin_manager.get_model(0).await;
    let model = match model {
        Some(m) => m,
        None => {
            info!("System exit");
            return;
        }
    };

    let mut core = Core::new();
    let r = core.push_plant(model.clone(), core_init_cfg).await;
    match r {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            let model = Arc::into_inner(model).unwrap();
            model
                .lock()
                .await
                .plugin()
                .uninstall(Vec::new())
                .unwrap()
                .unwrap();
            return;
        }
    }

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
    core.start().await;

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
        if let Command::Exit = r {
            break;
        }
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
