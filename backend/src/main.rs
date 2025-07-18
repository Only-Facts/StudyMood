mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match api::api().await {
        Ok(_) => {
            println!("[✅] Studymood API server shut down gracefully.");
            Ok(())
        }
        Err(e) => {
            eprintln!("[❌] Studymood API server failed to start: {e}");
            Err(e)
        }
    }
}
