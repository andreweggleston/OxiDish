pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
}

pub struct NewRecipe {
    pub title: String,
    pub description: String,
    pub ingredients: Vec<crate::models::IngredientEntry>, // all ingredients should exist in the database. In order to
                                                          // add a recipe which uses unseen ingredients, those must be
                                                          // added before the new recipe is created
}
