use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

use crate::http::{ApiContext, Error, OxiDishResult, ResultExt};
use crate::models::IngredientUnit;

pub fn router() -> Router<ApiContext> {
    Router::new().route(
        "/api/ingredients/units",
        get(list_ingredient_units).post(create_unit),
    )
}

#[derive(serde::Serialize)]
struct MultipleUnitsBody {
    units: Vec<IngredientUnit>,
}

async fn list_ingredient_units(ctx: State<ApiContext>) -> OxiDishResult<Json<MultipleUnitsBody>> {
    let ingredient_units_result = sqlx::query_as("SELECT * FROM ingredient_units")
        .fetch_all(&ctx.db)
        .await;

    match ingredient_units_result {
        Ok(ingredient_units) => OxiDishResult::Ok(
            StatusCode::OK,
            Json(MultipleUnitsBody {
                units: ingredient_units,
            }),
        ),
        Err(err) => OxiDishResult::Err(err.into()),
    }
}

#[derive(serde::Deserialize)]
struct NewIngredientUnit {
    unit: String,
    truncation: String,
}

async fn create_unit(
    ctx: State<ApiContext>,
    Json(req): Json<NewIngredientUnit>,
) -> OxiDishResult<Json<IngredientUnit>> {
    let ingredient_unit_result = sqlx::query_as(
        r#"
        INSERT INTO ingredient_units (unit, truncation) VALUES ($1, $2) RETURNING id, unit, truncation
        "#
    )
        .bind(req.unit).bind(req.truncation)
        .fetch_one(&ctx.db).await
        .on_constraint("ingredient_quantities_quantity_key",
                       |_| {
                           Error::unprocessable_entity([("quantity", "ingredient quantity already exists")])
                       },
        );

    match ingredient_unit_result {
        Ok(ingredient_unit) => OxiDishResult::Ok(StatusCode::CREATED, Json(ingredient_unit)),
        Err(err) => OxiDishResult::Err(err),
    }
}
