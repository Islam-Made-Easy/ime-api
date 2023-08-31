use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::error::Error;
use futures::future::{ok, Ready};
use log::info;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

pub struct LogMiddleware;

impl<S: 'static, Req, B> Transform<S, ServiceRequest> for LogMiddleware
    where
        S: Service<Req, Response=ServiceResponse, Error=Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = LogMiddlewareMiddleware<S, B, Req>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LogMiddlewareMiddleware { service })
    }
}

pub struct LogMiddlewareMiddleware<S, B, Req: Send + Sync + 'static> {
    service: S,
}

impl<S, B, Req> Service<ServiceRequest> for LogMiddlewareMiddleware<S, B, Req>
    where
        S: Service<Req, Response=ServiceResponse<B>, Error=Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let path = req.path().to_owned();
        let remote_addr = req
            .connection_info()
            .remote()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Log incoming request with IP address and timestamp
        info!(
            "[{}] Received request from {} - {} {}",
            chrono::Local::now(),
            remote_addr,
            req.method(),
            path
        );

        let fut = self.service.call(req);

        let path_clone = path.clone();
        Box::pin(async move {
            let res = fut.await?;

            let elapsed = start.elapsed().as_micros();
            // Log response with IP address, status code, and elapsed time
            info!(
                "[{}] Responded with {} in {}μs from {} - {}",
                chrono::Local::now(),
                res.status(),
                elapsed,
                remote_addr,
                path_clone
            );

            Ok(res)
        })
    }
}
