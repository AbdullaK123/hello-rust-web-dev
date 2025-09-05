use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::time::Instant;
use uuid::Uuid;
use tracing::{info, warn, error};

pub struct RequestLogging;

impl<S, B> Transform<S, ServiceRequest> for RequestLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggingMiddleware { service }))
    }
}

pub struct RequestLoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let request_id = Uuid::new_v4();
        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        let remote_addr = req.connection_info().realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        info!(
            request_id = %request_id,
            method = %method,
            path = %path,
            remote_addr = %remote_addr,
            user_agent = %user_agent,
            "🔥 Request started"
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let status = res.status();

            match status.as_u16() {
                500..=511 => {
                    error!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = duration.as_millis(),
                        remote_addr = %remote_addr,
                        "💥 Request completed with server error"
                    );
                }
                400..=499 => {
                    warn!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = duration.as_millis(),
                        remote_addr = %remote_addr,
                        "⚠️  Request completed with client error"
                    );
                }
                _ => {
                    info!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = duration.as_millis(),
                        remote_addr = %remote_addr,
                        "✅ Request completed successfully"
                    );
                }
            };

            Ok(res)
        })
    }
}
