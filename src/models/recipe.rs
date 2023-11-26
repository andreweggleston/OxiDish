#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(sqlx::FromRow)]
pub struct RecipeIngredient {
    pub id: i32,
    pub id_quantity: i32,
    pub id_unit: i32,
    pub id_name: i32,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct IngredientQuantity {
    pub id: i32,
    pub quantity: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Clone)]
pub struct IngredientUnit {
    pub id: i32,
    pub unit: String,
    pub truncation: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Clone)]
pub struct IngredientName {
    pub id: i32,
    pub name: String,
}

