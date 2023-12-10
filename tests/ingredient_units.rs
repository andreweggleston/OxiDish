#![feature(async_fn_track_caller)]

use std::sync::Arc;
use axum::body::Body;
use axum::{Error, http};

use axum::http::{Request, StatusCode};
use serde_json::json;
use sqlx::{PgPool};
use tower::ServiceExt;

use core::borrow::BorrowMut;

use OxiDish::config::Config;
use OxiDish::http::ApiContext;

mod common;
use common::response_json;



#[sqlx::test]
async fn test_get_ingredient_unit(db: PgPool) -> Result<(), Error>{
    let mut app = OxiDish::http::api_router(ApiContext {
        config: Arc::new(Config { database_url: "unused".to_string() }),
        db,
    });

    let new_ingredient_unit_name = String::from("Quart");
    let new_ingredient_unit_truncation = String::from("Qt");

    //i dont like this . perhaps i can make a macro
    let mut response1 = app.borrow_mut()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/ingredients/units")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "unit": new_ingredient_unit_name.as_str(),
                        "truncation": new_ingredient_unit_truncation.as_str()
                    })).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let resp1_json = response_json(&mut response1).await?;
    let new_ingredient_unit_id = resp1_json.get("id").unwrap().as_number();

    let mut response2 = app.borrow_mut()
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/api/ingredients/units")
                .body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    let resp2_json = response_json(&mut response2).await?;

    assert_eq!(resp2_json, json!({
        "units": [
            {
                "id": new_ingredient_unit_id,
                "unit": new_ingredient_unit_name.as_str(),
                "truncation": new_ingredient_unit_truncation.as_str()
            }
        ]
    }));
    Ok(())
}

#[sqlx::test]
async fn test_create_ingredient_unit(db: PgPool) -> Result<(), Error>{
    let app = OxiDish::http::api_router(ApiContext {
        config: Arc::new(Config { database_url: "unused".to_string() }),
        db,
    });

    let new_ingredient_unit_name = String::from("Quart");
    let new_ingredient_unit_truncation = String::from("Qt");

    //i dont like this . perhaps i can make a macro
    let mut response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/ingredients/units")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "unit": new_ingredient_unit_name.as_str(),
                        "truncation": new_ingredient_unit_truncation.as_str()
                    })).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let resp_json = response_json(&mut response).await?;

    //id field exists and is a number
    assert!(resp_json.get("id").unwrap().is_number());
    //unit field exists and is what we expect
    assert_eq!(*resp_json.get("unit").unwrap(), json!(new_ingredient_unit_name.as_str()));
    //truncation field exists and is what we expect
    assert_eq!(*resp_json.get("truncation").unwrap(), json!(new_ingredient_unit_truncation.as_str()));


    Ok(())
}