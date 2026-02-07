use crate::error::AppError;

use askama::Template;
use axum::debug_handler;
use axum::response::{Html, IntoResponse};

// This template struct is used to render the template in ../templates
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[debug_handler]
pub async fn index() -> Result<impl IntoResponse, AppError> {
    Ok(Html(IndexTemplate {}.render()?))
}
