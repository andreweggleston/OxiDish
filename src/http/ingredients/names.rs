use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;

use crate::http::{ApiContext, Error, Result, ResultExt};
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
) -> Result<Json<MultipleNamesBody>> {
    let ingredient_names = sqlx::query_as(
        "SELECT * FROM ingredient_names"
    ).fetch_all(&ctx.db).await?;

    Ok(
        Json(
            MultipleNamesBody {
                names: ingredient_names
            }
        )
    )
}

#[derive(serde::Deserialize)]
struct NewIngredientName {
    name: String,
}

async fn create_name(
    ctx: State<ApiContext>,
    Json(req): Json<NewIngredientName>,
) -> Result<Json<IngredientName>> {
    let ingredient_name = sqlx::query_as(
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
        )?;

    Ok(
        Json(
            ingredient_name
        )
    )
}