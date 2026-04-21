#[derive(Debug, PartialEq)]
pub struct Recipe {
    pub meal_type: MealType,
    pub calories: u32,
    pub prep_time: PrepTime,
    pub category: Category,
    pub name: String,
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
pub enum PrepTime {
    Low,
    Medium,
    High,
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
