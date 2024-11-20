use crate::models::{RecipeIngredientRow, RecipeRow};
use sqlx::{PgPool, Result};

pub struct RecipeResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub ingredients: Vec<IngredientResponse>,
}

pub struct IngredientResponse {
    pub id_name: i32,
    pub name: String,
    pub amount: String,
}

pub struct NewRecipe {
    pub title: String,
    pub description: String,
    pub ingredients: Vec<crate::models::IngredientEntry>, // all ingredients should exist in the database. In order to
                                                          // add a recipe which uses unseen ingredients, those must be
                                                          // added before the new recipe is created
}

pub async fn get_recipe_by_id(pool: &PgPool, recipe_id: i32) -> Result<Option<RecipeResponse>> {
    struct RecipeViewRow {
        recipe_id: i32,
        recipe_title: String,
        recipe_description: String,

        // ingredients are optional because there might be a recipe with no ingredients!
        id_ingredient_name: Option<i32>, // not sure if this should be included, might make API
        // design cleaner
        ingredient_name: Option<String>,
        ingredient_amount: Option<String>,
    }

    let rows = sqlx::query_as!(
        RecipeViewRow,
        r#"
        WITH recipe_data AS (
            SELECT id AS recipe_id, title, description
            FROM recipes
            WHERE id = $1
        )
        SELECT 
            rd.recipe_id,
            rd.title AS recipe_title,
            rd.description AS recipe_description,
            ri.id_ingredient_name,
            inames.name AS ingredient_name,
            ri.ingredient_amount
        FROM recipe_data rd
        LEFT JOIN 
            recipe_ingredients ri ON rd.recipe_id = ri.id_recipe
        LEFT JOIN 
            ingredient_names inames ON ri.id_ingredient_name = inames.id;
        "#,
        recipe_id
    )
    .fetch_all(pool)
    .await?;

    if rows.is_empty() {
        return Ok(None);
    }

    Ok(Some(RecipeResponse {
        id: rows[0].recipe_id,
        title: rows[0].recipe_title.clone(),
        description: rows[0].recipe_description.clone(),
        ingredients: rows
            .into_iter()
            .filter_map(|row| {
                row.id_ingredient_name.map(|id| IngredientResponse {
                    id_name: id,
                    name: row.ingredient_name.unwrap_or_default(),
                    amount: row.ingredient_amount.unwrap_or_default(),
                })
            })
            .collect(),
    }))
}

pub async fn create_recipe(pool: &PgPool, new_recipe: NewRecipe) -> Result<RecipeRow> {
    let recipe = sqlx::query_as!(
        RecipeRow,
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
    let _recipe_ingredients: Vec<RecipeIngredientRow> =
        builder.build_query_as().fetch_all(pool).await?;

    Ok(recipe)
}
