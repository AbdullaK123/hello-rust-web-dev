use actix_cors::Cors;

pub fn cors_middleware() -> Cors {
    Cors::default()
        .allowed_origin("http://0.0.0.0:8000")
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}