use actix_web::{HttpServer, App};
use ime_api::lib::get_all_questions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| { App::new().service(get_all_questions) }).bind(
        "127.0.0.1:8081"
    )?;
    let result = server.run().await;

    if let Err(e) = result {
        println!("Error: {:?}", e);
    }

    println!("Server stopped");

    Ok(())
}
