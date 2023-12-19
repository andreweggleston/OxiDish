use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

use crate::http::{ApiContext, Error, OxiDishResult, ResultExt};
use crate::models::IngredientQuantity;

pub fn router() -> Router<ApiContext> {
    Router::new().route(
        "/api/ingredients/quantities",
        get(list_ingredient_quantities).post(create_quantity),
    )
}

#[derive(serde::Serialize)]
struct MultipleQuantitiesBody {
    quantities: Vec<IngredientQuantity>,
}

async fn list_ingredient_quantities(
    ctx: State<ApiContext>,
) -> OxiDishResult<Json<MultipleQuantitiesBody>> {
    let ingredient_quantities_result = sqlx::query_as("SELECT * FROM ingredient_quantities")
        .fetch_all(&ctx.db)
        .await;

    match ingredient_quantities_result {
        Ok(ingredient_quantities) => OxiDishResult::Ok(
            StatusCode::OK,
            Json(MultipleQuantitiesBody {
                quantities: ingredient_quantities,
            }),
        ),
        Err(err) => OxiDishResult::Err(err.into()),
    }
}

#[derive(serde::Deserialize)]
struct NewIngredientQuantity {
    quantity: String,
}

async fn create_quantity(
    ctx: State<ApiContext>,
    Json(req): Json<NewIngredientQuantity>,
) -> OxiDishResult<Json<IngredientQuantity>> {
    let ingredient_quantity_result = sqlx::query_as(
        r#"
        INSERT INTO ingredient_quantities (quantity) VALUES ($1) RETURNING id, quantity
        "#,
    )
    .bind(req.quantity)
    .fetch_one(&ctx.db)
    .await
    .on_constraint("ingredient_quantities_quantity_key", |_| {
        Error::unprocessable_entity([("quantity", "ingredient quantity already exists")])
    });

    match ingredient_quantity_result {
        Ok(ingredient_quantity) => {
            OxiDishResult::Ok(StatusCode::CREATED, Json(ingredient_quantity))
        }
        Err(err) => OxiDishResult::Err(err),
    }
}
