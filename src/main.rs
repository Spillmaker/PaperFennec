use axum::{
    routing::{get, post},
    http::{StatusCode},
    Json, Router,
};


// Rusts threading-system is called tokio, apparently.
#[tokio::main]
async fn main() {
    println!("PaperFennec is booting...");

    // To be able to read back from threads I need the tracing bundle.
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/", get(hello_world))
    ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("PaperFennec is live!");
    axum::serve(listener, app).await.unwrap();

}

async fn hello_world() -> (StatusCode, &'static str) {
    (StatusCode::IM_A_TEAPOT, "Hello, World!")
}


