use axum::{extract::State, routing::get, Json, Router, Server};
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();
    let router = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/api/cpu", get(get_cpu_info))
        .with_state(app_state.clone());
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    println!("Listening on http://{}", server.local_addr());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new_all();
        loop {
            sys.refresh_cpu();
            let mut cpu_usage = app_state.cpu_usage.lock().unwrap();
            let new_cpu_usage: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            if cpu_usage.len() != new_cpu_usage.len() {
                cpu_usage.resize(new_cpu_usage.len(), 0.0);
            }
            cpu_usage.clone_from_slice(&new_cpu_usage);
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    server.await.unwrap();
}

async fn get_cpu_info(State(state): State<AppState>) -> Json<Vec<f32>> {
    Json(state.cpu_usage.lock().unwrap().clone())
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
