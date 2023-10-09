use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum::{extract::State, routing::get, Json, Router, Server};
use serde_json;
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();
    let router = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/api/cpu", get(get_cpu_info))
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
                let mut cpu_usage = app_state.cpu_usage.lock().unwrap();
                if cpu_usage.len() != new_cpu_usage.len() {
                    cpu_usage.resize(new_cpu_usage.len(), 0.0);
                }
                cpu_usage.clone_from_slice(&new_cpu_usage);
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    server.await.unwrap();
}

async fn get_cpu_info(State(state): State<AppState>) -> Json<Vec<f32>> {
    Json(state.cpu_usage.lock().unwrap().clone())
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
    loop {
        let cpu_usage = state.cpu_usage.lock().unwrap().clone();
        let message = Message::Text(serde_json::to_string(&cpu_usage).unwrap());
        ws.send(message).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

#[derive(Clone)]
struct AppState {
    cpu_usage: Arc<Mutex<Vec<f32>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            cpu_usage: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
