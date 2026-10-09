use tokio::time::{sleep, Duration};

pub async fn convert_pdf_to_webp() -> () {
    println!("Converting from PDF to WEBP");
    sleep(Duration::from_secs(2)).await;
    println!("Done.");
}