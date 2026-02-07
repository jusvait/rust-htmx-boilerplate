use axum::Router;
use tower_http::services::ServeDir;

pub fn serve_static() -> Router {
    let dir: String = std::env::var("STATIC_DIR").unwrap();
    Router::new().nest_service("/static", ServeDir::new(dir))
}
