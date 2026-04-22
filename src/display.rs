use crate::filters::{CalorieRange, PrepTime};
use crate::recipes::{Category, MealType, Recipe};
use std::fmt;

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}, {}", self.meal_type, self.name)?;
        writeln!(f, "Total Cook Time = {} minutes", self.prep_time_mins)?;
        writeln!(f, "Total Calories per Serving = {} calories", self.calories)?;
        writeln!(f, "Servings = {}", self.servings)?;
        writeln!(f, "\n")?;
        writeln!(f, "INGREDIENTS")?;
        writeln!(f, "\n")?;
        for ingredient in &self.ingredients {
            writeln!(f, "  - {}", ingredient)?;
        }
        writeln!(f, "\n")?;
        writeln!(f, "DIRECTIONS")?;
        writeln!(f, "\n")?;
        match &self.instructions {
            Some(instructions) => writeln!(f, "{}", instructions)?,
            None => writeln!(f, "No directions available")?,
        };
        Ok(())
    }
}

impl fmt::Display for MealType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Breakfast => write!(f, "Breakfast"),
            Self::Lunch => write!(f, "Lunch"),
            Self::Dinner => write!(f, "Dinner"),
        }
    }
}
impl fmt::Display for CalorieRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
        }
    }
}

impl fmt::Display for PrepTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::American => write!(f, "American"),
            Self::Burger => write!(f, "Burger"),
            Self::Chinese => write!(f, "Chinese"),
            Self::Egg => write!(f, "Egg"),
            Self::Italian => write!(f, "Italian"),
            Self::Pancake => write!(f, "Pancake"),
            Self::Pasta => write!(f, "Pasta"),
            Self::Salad => write!(f, "Salad"),
            Self::Smoothie => write!(f, "Smoothie"),
        }
    }
}
