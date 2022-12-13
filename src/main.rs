use std::env;

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use streamdeck_rs::{registration::RegistrationParams, StreamDeckSocket};

#[derive(Debug, Deserialize, Serialize)]
pub struct Empty();

#[tokio::main(worker_threads = 1)]
async fn main() {
    let params = &RegistrationParams::from_args(env::args()).unwrap();

    let socket = StreamDeckSocket::<
        Empty, //MyGlobalSettings,
        Empty, //MyActionSettings,
        Empty, //FromInspector,
        Empty, //ToInspector,
    >::connect(
        params.port,
        params.event.to_string(),
        params.uuid.to_string(),
    )
    .await
    .expect("connection failed");

    let (mut _sink, mut _stream) = socket.split();
}
