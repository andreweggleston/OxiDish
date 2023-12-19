mod names;
mod quantities;
mod units;

use crate::http::ApiContext;
use axum::Router;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .merge(names::router())
        .merge(quantities::router())
        .merge(units::router())
}
