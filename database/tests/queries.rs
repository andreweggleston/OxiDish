use database::queries::recipe::{get_recipe_by_id, search_recipes};

#[sqlx::test(fixtures("ingredient_names", "recipes"))]
async fn test_get_recipe_by_id_where_id_exists(pool: sqlx::PgPool) -> sqlx::Result<()> {
    let recipe = get_recipe_by_id(&pool, 1).await?;
    assert!(recipe.is_some());
    let recipe = recipe.unwrap();
    assert_eq!(recipe.id, 1);
    assert_eq!(recipe.title, "Pasta Bolognese");
    assert_eq!(recipe.description, "A simple pasta sauce");

    assert!(recipe
        .ingredients
        .iter()
        .any(|ingredient| { ingredient.name == "ground beef" }));
    assert!(recipe
        .ingredients
        .iter()
        .any(|ingredient| { ingredient.name == "tomato sauce" }));
    Ok(())
}

#[sqlx::test(fixtures("ingredient_names", "recipes"))]
async fn test_get_recipe_by_id_where_id_not_exists(pool: sqlx::PgPool) -> sqlx::Result<()> {
    let recipe = get_recipe_by_id(&pool, 55224).await?; // nonsense id
                                                        // number
    assert!(recipe.is_none());
    Ok(())
}

#[sqlx::test(fixtures("ingredient_names", "recipes"))]
async fn test_search_recipes_by_name_exact_title_name(pool: sqlx::PgPool) -> sqlx::Result<()> {
    let search_query = "Pasta Bolognese";
    let recipes = search_recipes(&pool, search_query).await?;
    assert!(recipes.len() > 0);
    assert!(recipes
        .into_iter()
        .filter_map(|r| {
            if r.title == search_query {
                Some(r.title)
            } else {
                None
            }
        })
        .collect::<String>()
        .contains(search_query));
    Ok(())
}

#[sqlx::test(fixtures("ingredient_names", "recipes"))]
async fn test_search_recipes_by_name_fuzzy_title_name(pool: sqlx::PgPool) -> sqlx::Result<()> {
    let search_query = "psta bolognese";

    let recipes = search_recipes(&pool, search_query).await?;
    assert!(recipes.len() > 0);
    assert!(recipes.get(0).unwrap().title == "Pasta Bolognese");

    Ok(())
}
