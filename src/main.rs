use actix_web::{HttpServer, App, middleware::Logger};
use env_logger::Builder;
use log::LevelFilter;
use crate::routes::{
    get_all_questions,
    get_random_questions,
    get_questions_by_type,
    get_filtered_questions,
};

mod utils;
mod routes;
mod models;
mod handlers;
mod middleware;
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    Builder::new().filter_level(LevelFilter::Info).init();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(get_all_questions)
            .service(get_random_questions)
            .service(get_filtered_questions)
            .service(get_questions_by_type)
    }).bind("127.0.0.1:8081")?;
    let result = server.run().await;

    if let Err(e) = result {
        println!("Error: {:?}", e);
    }

    println!("Server stopped");

    Ok(())
}
