use sqlx::{PgPool, Result};

use crate::models::recipe_ingredient::IngredientNameRow;

pub async fn create_ingredient_name(pool: &PgPool, name: &str) -> Result<IngredientNameRow> {
    let ingredient_name = sqlx::query_as!(
        IngredientNameRow,
        r#"
    INSERT INTO ingredient_names (name)
    VALUES ($1)
    RETURNING id, name
    "#,
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(ingredient_name)
}
