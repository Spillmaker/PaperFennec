use axum::{
    routing::{get}, Router,
};
mod webserver;
mod encoder;
mod database;


// Rusts threading-system is called tokio, apparently.
#[tokio::main]
async fn main() {
    println!("PaperFennec is booting...");
    // To be able to read back from threads I need the tracing bundle.
    tracing_subscriber::fmt::init();

    // Create the web-server
    let app: Router = Router::new()
        .route("/", get(webserver::routes::root::hello_world))
    ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("PaperFennec is live!");

    let server_handle = tokio::spawn(async move {
        println!("Server running on port 8080");
        axum::serve(listener, app).await.unwrap();
    });

    let _test = tokio::spawn(async move {
        encoder::packager::convert_pdf_to_webp().await;
    });


    let args: Vec<String> = std::env::args().collect();
    // Find the test-arg by name

    let library_path_arg = args.iter().find_map(|s| s.strip_prefix("--library-path=").map(|s| s.to_string()));
    if library_path_arg.is_none() {
        eprintln!("Cannot launch PaperFennec! - --library-path option is required!");
        std::process::exit(1);
    }

    println!("Test arg: {}", library_path_arg.as_ref().unwrap());

    let _library_scanner = tokio::spawn(async move {
        database::scanner::scan_library(library_path_arg.unwrap()).await;
    });

    // We hold for the webserver. this keeps the application open indefinitely.
    server_handle.await.unwrap();
}


