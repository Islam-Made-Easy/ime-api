// use actix_web::{ HttpResponse, Error, http };
// use serde_json::json;

// pub fn json_response<T: serde::Serialize>(
//     data: T,
//     status_code: http::StatusCode
// ) -> Result<HttpResponse, Error> {
//     Ok(
//         HttpResponse::build(status_code)
//             .content_type("application/json")
//             .body(serde_json::to_string(&data)?)
//     )
// }

// pub fn ok_response<T: serde::Serialize>(data: T) -> Result<HttpResponse, Error> {
//     json_response(data, http::StatusCode::OK)
// }

// pub fn error_response(message: &str, status_code: http::StatusCode) -> Result<HttpResponse, Error> {
//     let error_json = json!({ "error": message });
//     json_response(error_json, status_code)
// }
