use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;

use crate::http::{ApiContext, Error, OxiDishResult, ResultExt};
use crate::models::IngredientName;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/ingredients/names", get(list_ingredient_names).post(create_name))
}

#[derive(serde::Serialize)]
struct MultipleNamesBody {
    names: Vec<IngredientName>,
}

async fn list_ingredient_names(
    ctx: State<ApiContext>
) -> OxiDishResult<Json<MultipleNamesBody>> {
    let query_result = sqlx::query_as(
        "SELECT * FROM ingredient_names"
    ).fetch_all(&ctx.db).await;

    match query_result {
        Ok(ingredient_names) => {
            OxiDishResult::Ok(StatusCode::OK,
                              Json(MultipleNamesBody { names: ingredient_names }),
            )
        }
        Err(err) => {
            OxiDishResult::Err(err.into())
        }
    }
}

#[derive(serde::Deserialize)]
struct NewIngredientName {
    name: String,
}

async fn create_name(
    ctx: State<ApiContext>,
    Json(req): Json<NewIngredientName>,
) -> OxiDishResult<Json<IngredientName>> {
    let query_result = sqlx::query_as(
        r#"
        INSERT INTO ingredient_names (name) VALUES ($1) RETURNING id, name
        "#
    )
        .bind(req.name)
        .fetch_one(&ctx.db).await
        .on_constraint("ingredient_names_name_key",
                       |_| {
                           Error::unprocessable_entity([("name", "ingredient name already exists")])
                       },
        );

    match query_result {
        Ok(ingredient_name) => {
            OxiDishResult::Ok(
                StatusCode::CREATED,
                Json(ingredient_name),
            )
        }
        Err(err) => OxiDishResult::Err(err)
    }
}