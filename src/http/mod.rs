/// Defines common error types, used in the Result type defined below.
/// That Result type is returned by handlers
mod error;

mod ingredients;
mod recipes;

use crate::config::Config;

use anyhow::Context;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum OxiDishResult<T, S = StatusCode, E = Error> {
    Ok(S, T),
    Err(E),
}

impl<T> IntoResponse for OxiDishResult<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            OxiDishResult::Ok(status_code, value) => {
                let mut response = value.into_response();
                *response.status_mut() = status_code;
                response
            }
            OxiDishResult::Err(err) => err.into_response(),
        }
    }
}

use tower_http::trace::TraceLayer;

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `Extension<ApiContext>` to a handler function's
/// parameters.
///
/// In other projects I've passed this stuff as separate objects, e.g.
/// using a separate actix-web `Data` extractor for each of `Config`, `PgPool`, etc.
/// It just ends up being kind of annoying that way, but does have the whole
/// "pass only what you need where you need it" angle.
///
/// It may not be a bad idea if you need your API to be more modular (turn routes
/// on and off, and disable any unused extension objects) but it's really up to a
/// judgement call.
#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: Arc::new(config),
        db,
    };

    let app = api_router(api_context);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app)
        .await
        .context("error running http server")
}

pub fn api_router(api_context: ApiContext) -> Router {
    // This is the order that the modules were authored in.
    Router::new()
        .merge(recipes::router())
        .merge(ingredients::router())
        // Enables logging. Use `RUST_LOG=tower_http=debug`
        .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}
