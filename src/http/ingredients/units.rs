use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;

use crate::http::{ApiContext, Error, Result, ResultExt};
use crate::models::IngredientUnit;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/ingredients/unitss", get(list_ingredient_units).post(create_unit))
}

#[derive(serde::Serialize)]
struct MultipleUnitsBody {
    units: Vec<IngredientUnit>,
}

async fn list_ingredient_units(
    ctx: State<ApiContext>
) -> Result<Json<MultipleUnitsBody>> {
    let ingredient_units = sqlx::query_as(
        "SELECT * FROM ingredient_units"
    ).fetch_all(&ctx.db).await?;

    Ok(
        Json(
            MultipleUnitsBody {
                units: ingredient_units
            }
        )
    )
}

#[derive(serde::Deserialize)]
struct NewIngredientUnit {
    unit: String,
    truncation: String,
}

async fn create_unit(
    ctx: State<ApiContext>,
    Json(req): Json<NewIngredientUnit>,
) -> Result<Json<IngredientUnit>> {
    let ingredient_unit = sqlx::query_as(
        r#"
        INSERT INTO ingredient_units (unit, truncation) VALUES ($1, $2)
        "#
    )
        .bind(req.unit).bind(req.truncation)
        .fetch_one(&ctx.db).await
        .on_constraint("ingredient_quantities_quantity_key",
                       |_| {
                           Error::unprocessable_entity([("quantity", "ingredient quantity already exists")])
                       },
        )?;

    Ok(
        Json(
            ingredient_unit
        )
    )
}