use axum::{routing::get, Router, Server};
use sysinfo::{System, SystemExt, CpuExt};
use std::fmt::Write;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/cpu", get(cpu_info));
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    println!("Listening on http://{}", server.local_addr());
    server.await.unwrap();
}

async fn cpu_info() -> String {
    let mut s = String::new();
    let mut sys = System::new();
    sys.refresh_cpu();
    for (i, cpu) in sys.cpus().iter().enumerate() {
        writeln!(&mut s, "{}: {:.2}%", i, cpu.cpu_usage()).unwrap();
    }
    s
}
