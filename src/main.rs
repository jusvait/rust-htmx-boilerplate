use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::{Router, debug_handler, routing::get};
use std::fmt;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {}

struct AppError(askama::Error);

impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        AppError(value)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Template)]
        #[template(path = "error.html")]
        struct ErrorTemplate {}

        let status = StatusCode::INTERNAL_SERVER_ERROR;

        if let Ok(body) = (ErrorTemplate {}).render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}

#[debug_handler]
async fn hello() -> Result<impl IntoResponse, AppError> {
    Ok(Html(HelloTemplate {}.render()?))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a single route
    let app = Router::new().route("/", get(hello));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
