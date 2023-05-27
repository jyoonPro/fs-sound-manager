use axum::extract::{Path, State};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_server::Handle;
use rodio::{
    cpal, cpal::traits::HostTrait, source::Source, Decoder, DeviceTrait, OutputStream,
    OutputStreamHandle, Sink,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub struct AudioServer {
    axum_handle: Option<Handle>,
}

struct AudioServerState {
    device: cpal::Device,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl AudioServer {
    pub fn new() -> Self {
        Self { axum_handle: None }
    }

    pub fn start_server(&mut self, port: u16) {
        let host = cpal::default_host();
        let devices = host.output_devices().unwrap();
        let device = devices.into_iter().next().unwrap(); // TODO: Get device from user selection
        let (_stream, stream_handle) = OutputStream::try_from_device(&device).unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let shared_state = Arc::new(AudioServerState {
            device,
            stream_handle,
            sink,
        });

        let app = Router::new()
            .route("/play/:id", get(play_audio))
            .route("/stop", get(stop_audio))
            .with_state(shared_state);

        let handle = Handle::new();
        self.axum_handle = Some(handle.clone());

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        tokio::spawn(async move {
            axum_server::bind(addr)
                .handle(handle)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
    }

    pub fn stop_server(&mut self) {
        if let Some(axum_handle) = self.axum_handle.take() {
            axum_handle.shutdown();
        }
    }
}

async fn play_audio(
    Path(file_id): Path<u32>,
    State(state): State<Arc<AudioServerState>>,
) -> StatusCode {
    let file = BufReader::new(File::open("/Users/jyoon/Downloads/test.mp3").unwrap());
    let source = Decoder::new(file).unwrap();

    state.sink.append(source);
    StatusCode::ACCEPTED
}

async fn stop_audio(State(state): State<Arc<AudioServerState>>) -> StatusCode {
    state.sink.stop();
    StatusCode::ACCEPTED
}
