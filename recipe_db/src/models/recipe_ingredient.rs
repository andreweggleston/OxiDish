pub struct RecipeIngredient {
    pub id_recipe: i32,
    pub ingredient: IngredientEntry,
}

pub struct IngredientEntry {
    pub id_ingredient_name: i32,
    pub ingredient_amount: String,
}

pub struct Ingredient {
    pub id: i32,
    pub name: String,
}
