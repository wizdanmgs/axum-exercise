mod app;
mod error;
mod logging;
mod middleware;
mod routes;
mod state;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logging::init();

    let app = app::create_app();

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind address");

    axum::serve(listener, app).await.expect("server error");
}
