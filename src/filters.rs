use crate::recipes::{Category, MealType, PrepTime, Recipe};

#[derive(Debug, PartialEq)]
pub enum CalorieRange {
    //this goes here because it is used for the flitering, not necessary for the recipe struct
    Low,
    Medium,
    High,
}
