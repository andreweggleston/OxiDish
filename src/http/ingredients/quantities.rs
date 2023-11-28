use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;

use crate::http::{ApiContext, Error, Result, ResultExt};
use crate::models::IngredientQuantity;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/ingredients/quantities", get(list_ingredient_quantities).post(create_quantity))
}

#[derive(serde::Serialize)]
struct MultipleQuantitiesBody {
    quantities: Vec<IngredientQuantity>,
}

async fn list_ingredient_quantities(
    ctx: State<ApiContext>
) -> Result<Json<MultipleQuantitiesBody>> {
    let ingredient_quantities = sqlx::query_as(
        "SELECT * FROM ingredient_quantities"
    ).fetch_all(&ctx.db).await?;

    Ok(
        Json(
            MultipleQuantitiesBody {
                quantities: ingredient_quantities
            }
        )
    )
}

#[derive(serde::Deserialize)]
struct NewIngredientQuantity {
    quantity: String,
}

async fn create_quantity(
    ctx: State<ApiContext>,
    Json(req): Json<NewIngredientQuantity>,
) -> Result<Json<IngredientQuantity>> {
    let ingredient_quantity = sqlx::query_as(
        r#"
        INSERT INTO ingredient_quantities (quantity) VALUES ($1) RETURNING id, quantity
        "#
    )
        .bind(req.quantity)
        .fetch_one(&ctx.db).await
        .on_constraint("ingredient_quantities_quantity_key",
                       |_| {
                           Error::unprocessable_entity([("quantity", "ingredient quantity already exists")])
                       },
        )?;

    Ok(
        Json(
            ingredient_quantity
        )
    )
}