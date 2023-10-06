use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello_world));
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap())
        .serve(router.into_make_service());
    println!("Listening on http://{}", server.local_addr());
    server.await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
