mod db;
mod display;
mod filters;
mod recipes;
use db::*;
use display::*;
use filters::*;
use rand::prelude::*;
use recipes::*;
use rusqlite::Connection;
use std::io;

fn main() {
    let dbconnection =
        SqlLiteConnection::new(Connection::open("RecipeFinder.db").expect("Database not found"));
    dbconnection.create_table().expect("Could not create table");

    println!("Welcome to the Recipe Finder!");
    let meal = get_user_input(
        "To start, what meal are you eating? \n
        [1] Breakfast \n
        [2] Lunch \n
        [3] Dinner \n
        Please enter 1, 2, or 3",
    );

    let meal_type = match meal.as_str() {
        //making the return into the enum type
        "1" => MealType::Breakfast,
        "2" => MealType::Lunch,
        "3" => MealType::Dinner,
        _ => {
            println!("Invalid entry, restarting...");
            return;
        }
    };

    let category = match meal_type {
        MealType::Breakfast => get_user_input(
            "Breakfast it is! Do you want\n
            [1]Eggs\n
            [2]Pancakes\n
            [3]Smoothie\n
            Please enter 1, 2, or 3",
        ),
        MealType::Lunch => get_user_input(
            "Lunch coming up! Do you want\n
            [4]Salad\n
            [5]Burger\n
            [6]Pasta\n
            Please enter 4, 5, or 6",
        ),
        MealType::Dinner => get_user_input(
            "Let's get Dinner rolling! Do you want\n
            [7]Italian\n
            [8]Chinese\n
            [9]American\n
            Please enter 7, 8, or 9",
        ),
    };

    let category = match category.as_str() {
        //making the return into the enum type
        "1" => Category::Egg,
        "2" => Category::Pancake,
        "3" => Category::Smoothie,
        "4" => Category::Salad,
        "5" => Category::Burger,
        "6" => Category::Pasta,
        "7" => Category::Italian,
        "8" => Category::Chinese,
        "9" => Category::American,
        _ => {
            println!("Invalid entry, restarting....");
            return;
        }
    };

    let calorie = get_user_input(
        "How calorie dense do you want you food? \n
        [1] under 200 calories \n
        [2] 201 - 500 calories \n
        [3] above 500 calories \n
        Please enter 1, 2, or 3",
    );

    let calories = match calorie.as_str() {
        "1" => CalorieRange::Low,
        "2" => CalorieRange::Medium,
        "3" => CalorieRange::High,
        _ => {
            println!("Invalid entry, restarting...");
            return;
        }
    };

    let time = get_user_input(
        "How much time do you have? \n
        [1] under 25 min \n
        [2] 26-45 min \n
        [3] over 45 min \n
        Please enter 1, 2, or 3",
    );

    let prep_time = match time.as_str() {
        "1" => PrepTime::Low,
        "2" => PrepTime::Medium,
        "3" => PrepTime::High,
        _ => {
            println!("Invalid entry, restarting...");
            return;
        }
    };

    //call the filtering function with choices from the user, then if recipes found (Ok), then choose randomly from the vector
    // if no matches, but the filtering function worked, then returns no recipes found. If error from the function than states the error
    match filtering(&dbconnection, calories, prep_time, category, meal_type) {
        Ok(recipes) => {
            if recipes.is_empty() {
                //filtering ran, but no recipes match
                println!("No recipes found! Try a different combo next time!");
            } else {
                let mut rng = rand::rng();
                match recipes.choose(&mut rng) {
                    Some(recipe) => println!("{}", recipe), //prints the recipe!!
                    None => println!("No recipes found! Try a different combo next time!"), //need here too because the rand produces a result type
                }
            }
        }
        Err(e) => println!("Error finding recipes: {}", e), //issues running filtering
    };
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn low_cal_bounds() {
        let bounds = CalorieRange::Low.bounds();
        assert_eq!(bounds, (0, 200))
    }

    #[test]
    fn med_cal_bounds() {
        let bounds = CalorieRange::Medium.bounds();
        assert_eq!(bounds, (201, 500))
    }

    #[test]
    fn high_cal_bounds() {
        let bounds = CalorieRange::High.bounds();
        assert_eq!(bounds, (501, u32::MAX))
    }

    #[test]
    fn low_prep_bounds() {
        let bounds = PrepTime::Low.bounds();
        assert_eq!(bounds, (0, 25))
    }

    #[test]
    fn med_prep_bounds() {
        let bounds = PrepTime::Medium.bounds();
        assert_eq!(bounds, (26, 45))
    }

    #[test]
    fn high_prep_bounds() {
        let bounds = PrepTime::High.bounds();
        assert_eq!(bounds, (46, u32::MAX))
    }
}
