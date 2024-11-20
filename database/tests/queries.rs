use database::queries::recipe::{self, RecipeResponse};

#[sqlx::test(fixtures("ingredient_names", "recipes"))]
async fn test_get_recipe_by_id(pool: sqlx::PgPool) -> sqlx::Result<()> {
    let recipe = recipe::get_recipe_by_id(&pool, 1).await?;
    assert!(recipe.is_some());
    let recipe = recipe.unwrap();
    println!(
        r#"
        id: {}
        title: {}
        description: {}"#,
        recipe.id, recipe.title, recipe.description
    );

    println!("ingredients:");
    for i in &recipe.ingredients {
        println!(
            "\tid_name: {}\n\tname: {}\n\tamount: {}\n",
            i.id_name, i.name, i.amount
        );
    }

    println!();

    todo!();

    Ok(())
}
