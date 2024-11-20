pub struct RecipeIngredient {
    pub id_recipe: i32,
    pub ingredient: IngredientEntry,
}

//lots of work here to make this generic for all databases--would be easier if it was just
//impl<'r, sqlx::postgres::PgRow> sqlx::FromRow<'r, sqlx::postgres::PgRow> for RecipeIngredient
//wouldn't need any of the where clauses--but below is bascially what would be expanded by derive
impl<'a, R: sqlx::Row> sqlx::FromRow<'a, R> for RecipeIngredient
where
    String: sqlx::decode::Decode<'a, <R as sqlx::Row>::Database>,
    String: sqlx::types::Type<<R as sqlx::Row>::Database>,
    i32: sqlx::Decode<'a, <R as sqlx::Row>::Database>,
    i32: sqlx::types::Type<<R as sqlx::Row>::Database>,
    &'a std::primitive::str: ::sqlx::ColumnIndex<R>, //<--- ?
{
    fn from_row(row: &'a R) -> Result<Self, sqlx::Error> {
        Ok(RecipeIngredient {
            id_recipe: row.try_get("id_recipe")?,
            ingredient: IngredientEntry {
                id_ingredient_name: row.try_get("id_ingredient_name")?,
                ingredient_amount: row.try_get("ingredient_amount")?,
            },
        })
    }
}

pub struct IngredientEntry {
    pub id_ingredient_name: i32,
    pub ingredient_amount: String,
}

pub struct Ingredient {
    pub id: i32,
    pub name: String,
}
