use crate::recipes::Recipe;
use rusqlite::{Connection, Error, named_params};

pub struct SqlLiteConnection {
    pub conn: Connection,
}

impl<'a> SqlLiteConnection {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn create_recipe(&mut self, recipe: &Recipe) -> Result<i64, Error> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO recipe (name,meal_type,calories, prep_time_mins, category,
            ingredients, servings, instructions) VALUES (:name,:meal_type,:calories, :prep_time_mins, :category,
            :ingredients, :servings, :instructions)")?;
        stmt.execute(named_params! {
            ":name": recipe.name,
            ":calories": recipe.calories,
            ":instructions": recipe.instructions,
            ":meal_type" : recipe.meal_type.to_string(),
            ":prep_time_mins" : recipe.prep_time_mins,
            ":category" : recipe.category.to_string(),
            ":ingredients" : recipe.ingredients.join(","),
            ":servings" : recipe.servings,
        })?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn create_table(&self) -> Result<(), Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS recipe (
    id INTEGER PRIMARY KEY,
    name TEXT,
    meal_type TEXT,
    calories INTEGER,
    prep_time_mins INTEGER,
    category TEXT,
    ingredients TEXT,
    servings INTEGER,
    instructions TEXT
)",
            (),
        )?;
        Ok(())
    }
}
