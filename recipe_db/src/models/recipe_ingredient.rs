pub struct RecipeIngredient {
    pub id_recipe: i32,
    pub ingredient: Ingredient,
}

pub struct Ingredient {
    pub id_ingredient_name: i32,
    pub ingredient_amount: String,
}

pub struct IngredientName {
    pub id: i32,
    pub name: String,
}
