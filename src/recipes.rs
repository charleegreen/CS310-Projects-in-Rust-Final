use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub meal_type: MealType,
    pub calories: u32,
    pub prep_time_mins: u32,
    pub category: Category,
    pub ingredients: Vec<String>,
    pub instructions: Option<String>,
    pub servings: u32,
}

#[derive(Debug, PartialEq)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
}

#[derive(Debug, PartialEq)]
pub enum Category {
    American,
    Burger,
    Chinese,
    Egg,
    Italian,
    Pancake,
    Pasta,
    Salad,
    Smoothie,
}

impl FromStr for MealType {
    //converts the string from the database into the enum type
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Breakfast" => Ok(Self::Breakfast),
            "Lunch" => Ok(Self::Lunch),
            "Dinner" => Ok(Self::Dinner),
            _ => Err(format!("Unknown meal type: {}", s)), //error handling in case it doesn't match
        }
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "American" => Ok(Self::American),
            "Burger" => Ok(Self::Burger),
            "Chinese" => Ok(Self::Chinese),
            "Egg" => Ok(Self::Egg),
            "Italian" => Ok(Self::Italian),
            "Pancake" => Ok(Self::Pancake),
            "Pasta" => Ok(Self::Pasta),
            "Salad" => Ok(Self::Salad),
            "Smoothie" => Ok(Self::Smoothie),
            _ => Err(format!("Unknown category: {}", s)), //error handling in case it doesn't match
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn meal_type_parse() {
        assert_eq!("Breakfast".parse::<MealType>(), Ok(MealType::Breakfast))
    }

    #[test]
    fn meal_type_parse_fail() {
        assert!("breakfast".parse::<MealType>().is_err())
    }

    #[test]
    fn cat_parse() {
        assert_eq!("Italian".parse::<Category>(), Ok(Category::Italian))
    }

    #[test]
    fn cat_parse_fail() {
        assert!("italy".parse::<Category>().is_err())
    }
}
