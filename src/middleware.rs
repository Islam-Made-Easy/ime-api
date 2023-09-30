// use actix_service::Service;
// use actix_web::{
//     body::BoxBody,
//     dev::{ServiceRequest, ServiceResponse},
// };
// use log::info;
// use std::time::Instant;

// pub async fn log_middleware<'a, S>(
//     req: ServiceRequest,
//     srv: &'a S,
// ) -> Result<ServiceResponse<BoxBody>, actix_web::Error>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>,
// {
//     let start = Instant::now();
//     let path = req.path().to_owned();
//     let client_ip = req.peer_addr().unwrap();

//     // Log incoming request
//     info!(
//         "Received request from {} {} {}",
//         client_ip,
//         req.method(),
//         path
//     );

//     let res = srv.call(req).await?;

//     // Log response
//     let elapsed = start.elapsed().as_micros();
//     info!("Responded with {} in {}μs", res.status(), elapsed);

//     Ok(res)
// }

// pub async fn error_middleware<S>(
//     req: ServiceRequest,
//     srv: &S,
//     err: actix_web::Error,
// ) -> Result<ServiceResponse<BoxBody>, actix_web::Error>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>,
// {
//     info!("Error: {}", err);

//     srv.call(req).await
// }

// pub fn response_middleware<S>(
//     _req: ServiceRequest,
//     _srv: &S,
//     res: ServiceResponse<BoxBody>,
// ) -> Result<ServiceResponse<BoxBody>, actix_web::Error>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>,
// {
//     info!("Response: {}", res.status());

//     Ok(res)
// }
