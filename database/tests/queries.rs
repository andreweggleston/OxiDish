use database::queries::recipe::get_recipe_by_id;

#[sqlx::test(fixtures("fixtures/ingredient_names.sql", "fixtures/recipes.sql"))]
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

#[sqlx::test(fixtures("fixtures/ingredient_names.sql", "fixtures/recipes.sql"))]
async fn test_get_recipe_by_id_where_id_not_exists(pool: sqlx::PgPool) -> sqlx::Result<()> {
    let recipe = get_recipe_by_id(&pool, 55224).await?; // nonsense id
                                                        // number
    assert!(recipe.is_none());
    Ok(())
}
