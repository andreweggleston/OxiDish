use crate::models::Recipe;
use sqlx::{PgPool, Result};

pub async fn get_recipe(pool: &PgPool, recipe_id: i32) -> Result<Option<Recipe>> {
    sqlx::query_as!(
        Recipe,
        r#"
    SELECT * FROM recipes WHERE id = $1
    "#,
        recipe_id
    )
    .fetch_optional(pool)
    .await
}
