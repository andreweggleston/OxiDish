/// Defines common error types, used in the Result type defined below.
/// That Result type is returned by handlers
mod error;

mod recipes;
mod ingredients;

use std::net::{Ipv4Addr, SocketAddr};
use crate::config::Config;
use std::sync::Arc;
use anyhow::Context;
use axum::Router;
use sqlx::PgPool;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

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
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: Arc::new(config),
        db,
    };

    let app = api_router(api_context);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running http server")
}

fn api_router(api_context: ApiContext) -> Router {
    // This is the order that the modules were authored in.
    Router::new()
        .merge(recipes::router())
        .merge(ingredients::router())
        // Enables logging. Use `RUST_LOG=tower_http=debug`
        .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}