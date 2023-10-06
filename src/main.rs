use axum::{extract::State, routing::get, Router, Server};
use std::{
    fmt::Write,
    sync::{Arc, Mutex},
};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let sys = Arc::new(Mutex::new(System::new_all()));
    let app_state = AppState { sys };
    let router = Router::new()
        .route("/cpu", get(cpu_info))
        .with_state(app_state);
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    println!("Listening on http://{}", server.local_addr());
    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn cpu_info(State(state): State<AppState>) -> String {
    let mut s = String::new();
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    for (i, cpu) in sys.cpus().iter().enumerate() {
        writeln!(&mut s, "{}: {:.2}%", i, cpu.cpu_usage()).unwrap();
    }
    s
}
