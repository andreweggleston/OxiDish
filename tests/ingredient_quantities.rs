#![feature(async_fn_track_caller)]

use axum::body::Body;
use axum::{http, Error};
use std::sync::Arc;

use axum::http::{Request, StatusCode};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use core::borrow::BorrowMut;

use OxiDish::config::Config;
use OxiDish::http::ApiContext;

mod common;
use common::response_json;

#[sqlx::test]
async fn test_get_ingredient_quantity(db: PgPool) -> Result<(), Error> {
    let mut app = OxiDish::http::api_router(ApiContext {
        config: Arc::new(Config {
            database_url: "unused".to_string(),
        }),
        db,
    });

    let new_ingredient_quantity = String::from("1");

    //i dont like this . perhaps i can make a macro
    let mut response1 = app
        .borrow_mut()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/ingredients/quantities")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "quantity": new_ingredient_quantity.as_str()
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let resp1_json = response_json(&mut response1).await?;
    let new_ingredient_id = resp1_json.get("id").unwrap().as_number();

    let mut response2 = app
        .borrow_mut()
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/api/ingredients/quantities")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let resp2_json = response_json(&mut response2).await?;

    assert_eq!(
        resp2_json,
        json!({
            "quantities": [
                {
                    "id": new_ingredient_id,
                    "quantity": new_ingredient_quantity.as_str()
                }
            ]
        })
    );
    Ok(())
}

#[sqlx::test]
async fn test_create_ingredient_quantity(db: PgPool) -> Result<(), Error> {
    let app = OxiDish::http::api_router(ApiContext {
        config: Arc::new(Config {
            database_url: "unused".to_string(),
        }),
        db,
    });

    let new_ingredient_quantity = String::from("1");

    //i dont like this . perhaps i can make a macro
    let mut response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/ingredients/quantities")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "quantity": new_ingredient_quantity.as_str()
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let resp_json = response_json(&mut response).await?;

    //id field exists and is a number
    assert!(resp_json.get("id").unwrap().is_number());
    //name field exists and is what we expect
    assert_eq!(
        *resp_json.get("quantity").unwrap(),
        json!(new_ingredient_quantity.as_str())
    );

    Ok(())
}

#[sqlx::test]
async fn test_create_two_identical_ingredient_quantities(db: PgPool) -> Result<(), Error> {
    let mut app = OxiDish::http::api_router(ApiContext {
        config: Arc::new(Config {
            database_url: "unused".to_string(),
        }),
        db,
    });

    let new_ingredient_quantity = String::from("1");

    //i dont like this . perhaps i can make a macro
    let response1 = app
        .borrow_mut()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/ingredients/quantities")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "quantity": new_ingredient_quantity.as_str()
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response1.status(), StatusCode::CREATED);

    let response2 = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/ingredients/quantities")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": new_ingredient_quantity.as_str()
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response2.status(), StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}
