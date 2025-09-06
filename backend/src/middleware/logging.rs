use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};
use tracing::info;

pub async fn request_logging(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let start_time = std::time::Instant::now();
    let request_id = uuid::Uuid::new_v4();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let remote_addr = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        remote_addr = %remote_addr,
        user_agent = %user_agent,
        "🔥 Request started"
    );

    let res = next.call(req).await?;
    let duration = start_time.elapsed();

    info!(
        request_id = %request_id,
        status = res.status().as_u16(),
        duration_ms = duration.as_millis(),
        "✅ Request completed"
    );

    Ok(res)
}