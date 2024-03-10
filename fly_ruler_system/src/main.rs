use fly_ruler_plugin::PluginInfo;
use fly_ruler_system::system::System;
// use fly_ruler_system::{inputer::StickController, outputer::CSVOutputer, system::System};
use fly_ruler_utils::{
    error::FrError, plane_model::Control, AsInputer, AsOutputer, Command, OutputReceiver,
};
use mlua::{Function, Lua, LuaSerdeExt};
use std::{collections::HashMap, path::Path};

fn main() {
    env_logger::builder().format_timestamp(None).init();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // let stick = rt.block_on(async { StickController::new(0, Control::default()).await });
    // let receiver = stick.get_receiver();
    // let control_thread = StickController::thread_build(stick);

    // let cancellation_token = ViewerCancellationToken::new();
    // let mut csv = CSVOutputer::new("../Plane0.csv");

    // let mut system = rt.block_on(async {
    // let mut system = System::builder(Box::new(|_err: FrError| {}));
    // system
    // .set_dir("../config", "../plugins/model")
    // .init(Some(|_m: &HashMap<usize, PluginInfo>| {
    // vec![(0, vec!["../plugins/model/f16_model/data".to_string()])]
    // }))
    // .await
    // .set_controller(move |_m| {
    // let mut h = HashMap::new();
    // h.insert(0_usize, receiver);
    // h
    // })
    // .await
    // .set_viewer(0, |m: OutputReceiver| csv.set_receiver(m))
    // .await;
    // system
    // });

    // let viewer_thread = CSVOutputer::build_thread(&mut csv, cancellation_token.clone());

    rt.block_on(async move {
        let lua = Lua::new();
        lua.globals()
            .set(
                "create_system",
                Function::wrap(|_: &Lua, ()| Ok(System::new(Box::new(|_err: FrError| {})))),
            )
            .unwrap();
        lua.globals()
            .set(
                "create_command",
                Function::wrap(|lua: &Lua, value: mlua::Value| match value {
                    mlua::Value::String(s) => {
                        if s == "Exit" {
                            Ok(Command::Exit)
                        } else {
                            Ok(Command::Extra(s.to_string_lossy().to_string()))
                        }
                    }
                    mlua::Value::Table(_) => Ok(Command::Control(lua.from_value(value)?)),
                    _ => Err(mlua::Error::RuntimeError("Invalid command".to_string())),
                }),
            )
            .unwrap();
        let _res = lua.load(Path::new("./main.lua")).exec_async().await;

        // system.run(false).await.stop();
        // cancellation_token.cancel();
    });

    // control_thread.join().unwrap();
    // viewer_thread.join().unwrap();
}
