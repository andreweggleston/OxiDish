// This is imported by different tests that use different functions.
#![allow(dead_code)]
#![feature(async_fn_track_caller)]

use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use axum::http::{request, Request};
use axum::response::Response;
use http_body_util::BodyExt;

pub trait RequestBuilderExt {
    fn json(self, json: serde_json::Value) -> Request<Body>;

    fn empty_body(self) -> Request<Body>;
}

impl RequestBuilderExt for request::Builder {
    fn json(self, json: serde_json::Value) -> Request<Body> {
        self.header("Content-Type", "application/json")
            .body(Body::from(json.to_string()))
            .expect("failed to build request")
    }

    fn empty_body(self) -> Request<Body> {
        self.body(Body::empty()).expect("failed to build request")
    }
}

#[track_caller]
pub async fn response_json(resp: &mut Response<Body>) -> Result<serde_json::Value, axum::Error> {
    assert_eq!(
        resp.headers()
            .get(CONTENT_TYPE)
            .expect("expected Content-Type"),
        "application/json"
    );

    let body = resp.body_mut();

    let bytes = body.collect().await?.to_bytes();

    Ok(serde_json::from_slice(&bytes).expect("failed to read response body as json"))
}

#[track_caller]
pub fn expect_string(value: &serde_json::Value) -> &str {
    value
        .as_str()
        .unwrap_or_else(|| panic!("expected string, got {value:?}"))
}
