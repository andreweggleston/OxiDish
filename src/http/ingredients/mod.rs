mod names;
mod quantities;
mod units;

use axum::Router;
use crate::http::ApiContext;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .merge(names::router())
        .merge(quantities::router())
        .merge(units::router())
}