// use actix_web::{ HttpResponse, http::StatusCode };
// use log::error;

// pub fn handle_error<E: std::error::Error>(error: E) -> HttpResponse {
//     // Log the error
//     error!("Error: {}", error);

//     // Create an appropriate HTTP response
//     HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(
//         format!("Internal Server Error: {}", error)
//     )
// }
