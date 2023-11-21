use std::net::SocketAddr;
use axum::{response::IntoResponse, routing::get, Router};

async fn hello_world() -> impl IntoResponse {
    "Hello World"
}

#[shuttle_runtime::main]
async fn shuttle() -> shuttle_axum::ShuttleAxum{
    let app = Router::new().route("/", get(hello_world));
    Ok(app.into())
    // let addr = SocketAddr::new([0, 0, 0, 0].into(), 3000);

    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

}
