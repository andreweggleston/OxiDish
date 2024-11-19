use crate::models::{NewRecipe, Recipe, RecipeIngredient};
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

pub async fn create_recipe(pool: &PgPool, new_recipe: NewRecipe) -> Result<Recipe> {
    let recipe = sqlx::query_as!(
        Recipe,
        r#"
    INSERT INTO recipes (title, description)
    VALUES ($1, $2)
    RETURNING id, title, description"#,
        new_recipe.title,
        new_recipe.description
    )
    .fetch_one(pool)
    .await?;

    let mut builder: sqlx::QueryBuilder<'_, sqlx::Postgres> = sqlx::QueryBuilder::new("");
    builder.push(
        "INSERT INTO recipe_ingredients (id_recipe, id_ingredient_name, id_ingredient_amount) ",
    );
    builder.push_values(new_recipe.ingredients, |mut b, ingredient| {
        b.push_bind(&recipe.id)
            .push_bind(ingredient.id_ingredient_name)
            .push_bind(ingredient.ingredient_amount);
    });
    let _recipe_ingredients: Vec<RecipeIngredient> =
        builder.build_query_as().fetch_all(pool).await?;

    Ok(recipe)
}
