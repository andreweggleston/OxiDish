use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::routing::get;

use crate::http::{ApiContext, Error, Result};
use crate::models::{Recipe, RecipeIngredient};


#[derive(serde::Serialize, serde::Deserialize)]
struct RecipeIngredientDTO {
    id_ingredient_name: i32,
    id_ingredient_quantity: i32,
    id_ingredient_unit: i32,
}

#[derive(serde::Deserialize)]
struct NewRecipe {
    title: String,
    description: String,
    ingredients: Vec<RecipeIngredientDTO>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct RecipeDTO {
    id: i32,
    title: String,
    description: String,
    ingredients: Vec<RecipeIngredientDTO>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct ListRecipesQuery {
    ingredient_name_ids: Option<Vec<i32>>, //list of ingredient name ids
}

#[derive(serde::Serialize)]
struct MultipleRecipesBody {
    recipes: Vec<Recipe>,
}

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/recipes",
               get(list_recipes).post(create_recipe))
        .route("/api/recipes/:recipe_id", get(show_recipe).put(update_recipe))
}

async fn list_recipes(
    ctx: State<ApiContext>,
    query: Query<ListRecipesQuery>,
) -> Result<Json<MultipleRecipesBody>> {
    let recipes: Vec<Recipe> = sqlx::query_as(
        r#"
        SELECT recipes.id, recipes.title, recipes.description
        FROM recipes
        INNER JOIN recipe_ingredients
        ON recipes.id = recipe_ingredients.id_recipe
        WHERE recipe_ingredients.id_ingredient_name IN $1
        UNIQUE
        "#
    ).bind(&query.ingredient_name_ids).fetch_all(&ctx.db).await?;

    Ok(
        Json(
            MultipleRecipesBody {
                recipes
            }
        )
    )
}

async fn create_recipe(
    ctx: State<ApiContext>,
    Json(req): Json<NewRecipe>,
) -> Result<Json<RecipeDTO>> {
    let recipe = sqlx::query_as!(
        Recipe,
        r#"
        INSERT INTO recipes (title, description)
        VALUES ($1, $2)
        RETURNING id, title, description"#,
        req.title, req.description
    ).fetch_one(&ctx.db).await?;

    let mut builder: sqlx::QueryBuilder<'_, sqlx::Postgres> = sqlx::QueryBuilder::new("");
    builder.push("INSERT INTO recipe_ingredients (id_recipe, id_ingredient_name, id_ingredient_unit, id_ingredient_quantity) ");
    builder.push_values(
        req.ingredients,
        |mut b, ingredient| {
            b
                .push_bind(&recipe.id)
                .push_bind(ingredient.id_ingredient_name)
                .push_bind(ingredient.id_ingredient_unit)
                .push_bind(ingredient.id_ingredient_quantity);
        }
    );

    let recipe_ingredients: Vec<RecipeIngredient> = builder.build_query_as().fetch_all(&ctx.db).await?;

    Ok(
        Json(
            RecipeDTO {
                id: recipe.id,
                title: recipe.title,
                description: recipe.description,
                ingredients: recipe_ingredients.into_iter().map(|ingredient| ingredient.into()).collect(),
            }
        )
    )
}

async fn show_recipe(
    ctx: State<ApiContext>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<RecipeDTO>> {
    let recipe = sqlx::query_as!(Recipe, r#"
        SELECT * FROM recipes WHERE id = $1
    "#, recipe_id).fetch_one(&ctx.db).await?;

    let ingredient_dtos = sqlx::query_as!(RecipeIngredient, r#"
        SELECT * FROM recipe_ingredients WHERE id_recipe = $1
    "#, recipe_id).fetch_all(&ctx.db).await?;

    Ok(
        Json(
            RecipeDTO {
                id: recipe.id,
                title: recipe.title,
                description: recipe.description,
                ingredients: ingredient_dtos.into_iter().map(|ingredient| ingredient.into()).collect(),
            }
        )
    )
}

async fn update_recipe(
    ctx: State<ApiContext>,
    Path(recipe_id): Path<i32>,
    Json(req): Json<RecipeDTO>,
) -> Result<Json<RecipeDTO>> {
    // first sanity check that the user is submitting a recipe for this path
    if req.id != recipe_id {
        Err(Error::unprocessable_entity([("id", "id of updated recipe dto does not match id in path")]))
    } else {
        let recipe = sqlx::query_as!(Recipe, r#"
            UPDATE recipes
            SET title = $1, description = $2
            WHERE id = $3
            RETURNING id, title, description
        "#, req.title, req.description, req.id).fetch_one(&ctx.db).await?;
        let mut builder
            = sqlx::QueryBuilder::new(
            "DELETE FROM recipe_ingredients WHERE id_recipe = $1;"
        );
        builder.push_bind(recipe_id);
        builder.push(
            "INSERT INTO recipe_ingredients (id_recipe, id_ingredient_name, id_ingredient_unit, id_ingredient_quantity) ");
        builder.push_values(req.ingredients, |mut b, ingredient| {
            b
                .push_bind(req.id)
                .push_bind(ingredient.id_ingredient_name)
                .push_bind(ingredient.id_ingredient_unit)
                .push_bind(ingredient.id_ingredient_quantity);
        });
        let ingredient_dtos = builder.build_query_as().fetch_all(&ctx.db).await?;

        Ok(
            Json(
                RecipeDTO {
                    id: recipe.id,
                    title: recipe.title,
                    description: recipe.description,
                    ingredients: ingredient_dtos.into_iter().map(|ingredient: RecipeIngredient| ingredient.into()).collect(),
                }
            )
        )
    }
}

impl From<RecipeIngredient> for RecipeIngredientDTO {
    fn from(ingredient: RecipeIngredient) -> Self {
        RecipeIngredientDTO {
            id_ingredient_name: ingredient.id_ingredient_name,
            id_ingredient_quantity: ingredient.id_ingredient_quantity,
            id_ingredient_unit: ingredient.id_ingredient_unit,
        }
    }
}