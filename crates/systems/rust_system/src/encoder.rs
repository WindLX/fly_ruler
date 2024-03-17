use fly_ruler_utils::{plane_model::CoreOutput, Command};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ViewMessage {
    id: String,
    output: Option<CoreOutput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ViewMessageGroup {
    time: f64,
    view_msg: Vec<ViewMessage>,
}

pub fn encode(time: f64, msg: Vec<(String, CoreOutput)>) -> Result<String, serde_json::Error> {
    let msg_group = ViewMessageGroup {
        time,
        view_msg: msg
            .into_iter()
            .map(|(id, output)| ViewMessage {
                id,
                output: Some(Into::<CoreOutput>::into(output)),
            })
            .collect(),
    };
    let json_str = serde_json::to_string(&msg_group);
    json_str
}

pub fn decode(command: &[u8]) -> Result<Command, serde_json::Error> {
    serde_json::from_slice(command)
}
