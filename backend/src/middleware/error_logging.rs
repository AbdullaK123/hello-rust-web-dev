use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use tracing::error;

pub struct ErrorLogging;

impl<S, B> Transform<S, ServiceRequest> for ErrorLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ErrorLoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorLoggingMiddleware { service }))
    }
}

pub struct ErrorLoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorLoggingMiddleware<S>
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
        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        
        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => {
                    // Check for server errors (5xx status codes)
                    if res.status().is_server_error() {
                        error!(
                            method = %method,
                            path = %path,
                            status = res.status().as_u16(),
                            "Server error occurred"
                        );
                    }
                    Ok(res)
                }
                Err(err) => {
                    error!(
                        method = %method,
                        path = %path,
                        error = %err,
                        "Request processing error"
                    );
                    Err(err)
                }
            }
        })
    }
}
