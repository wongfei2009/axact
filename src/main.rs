use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum::{extract::State, routing::get, Router, Server};
use serde_json;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

type SnapShot = Vec<f32>;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();
    let router = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/api/realtime-cpu", get(realtime_cpu_info))
        .with_state(app_state.clone());
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    println!("Listening on http://{}", server.local_addr());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new_all();
        loop {
            sys.refresh_cpu();
            let new_cpu_usage: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            {
                let tx = app_state.tx.clone();
                let _ = tx.send(new_cpu_usage);
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    server.await.unwrap();
}

async fn realtime_cpu_info(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws| async {
        realtime_cpus_stream(ws, state).await;
    })
}

async fn realtime_cpus_stream(mut ws: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();
    loop {
        let cpu_usage = rx.recv().await.unwrap();
        let message = Message::Text(serde_json::to_string(&cpu_usage).unwrap());
        ws.send(message).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<SnapShot>,
}

impl AppState {
    fn new() -> Self {
        let (tx, _) = broadcast::channel::<SnapShot>(1);
        Self { tx }
    }
}
