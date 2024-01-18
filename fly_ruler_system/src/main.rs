use fly_ruler_plugin::PluginInfo;
use fly_ruler_system::{inputer::StickController, outputer::CSVViewer, system::System};
use fly_ruler_utils::{
    error::FrError, plane_model::Control, IsInputer, IsOutputer, OutputReceiver,
    ViewerCancellationToken,
};
use std::collections::HashMap;

fn main() {
    env_logger::builder().format_timestamp(None).init();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let stick = rt.block_on(async { StickController::new(0, Control::default()).await });
    let receiver = stick.get_receiver();
    let control_thread = StickController::thread_build(stick);

    let cancellation_token = ViewerCancellationToken::new();
    let mut csv = CSVViewer::new("..\\Plane0.csv");

    let mut system = rt.block_on(async {
        let mut system = System::builder(Box::new(|_err: FrError| {}));
        system
            .set_dir("..\\config", "..\\plugins")
            .init(Some(|_m: &HashMap<usize, PluginInfo>| {
                vec![(0, vec!["..\\plugins\\model\\f16_model\\data".to_string()])]
            }))
            .await
            .set_controller(move |_m| {
                let mut h = HashMap::new();
                h.insert(0_usize, receiver);
                h
            })
            .await
            .set_viewer(0, |m: OutputReceiver| csv.set_receiver(m))
            .await;
        system
    });

    let viewer_thread = CSVViewer::thread_build(csv, cancellation_token.clone());

    rt.block_on(async move {
        system.run(false).await.stop();
        cancellation_token.cancel();
    });

    control_thread.join().unwrap();
    viewer_thread.join().unwrap();
}
