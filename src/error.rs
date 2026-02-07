use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

/*
 * Error wrapper for rendering errors and printing them to stdout.
 * Renders encountered errors using the ../error.html template.
 */
pub struct AppError(askama::Error);

// 1. Implement a conversion from rendering errors to AppError.
impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        AppError(value)
    }
}

// 2. Implement a conversion from the AppError into Response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Template)]
        #[template(path = "error.html")]
        struct ErrorTemplate {}

        println!("{}", self.0);

        let status = StatusCode::INTERNAL_SERVER_ERROR;

        if let Ok(body) = (ErrorTemplate {}).render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
