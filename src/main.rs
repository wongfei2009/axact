use axum::{extract::State, routing::get, Json, Router, Server};
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let sys = Arc::new(Mutex::new(System::new_all()));
    let app_state = AppState { sys };
    let router = Router::new()
        .route("/api/cpu", get(cpu_info))
        .with_state(app_state);
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    println!("Listening on http://{}", server.local_addr());
    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn cpu_info(State(state): State<AppState>) -> Json<Vec<f32>> {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    let cpu_usage: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(cpu_usage)
}
