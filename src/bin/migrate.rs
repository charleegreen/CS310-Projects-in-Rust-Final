use rayon::prelude::*;
//this will allow the multithreading for the data migration into the database
//the multithreading part is the data parsing of the files.
use recipe_finder::db::*;
use recipe_finder::filters::{CalorieRange, PrepTime};
use recipe_finder::recipes::*;
use rusqlite::Connection;
use std::fs;
use std::sync::Mutex; //needed to make threads take turns actually going into the database

//chose to do a bin because this code should only be run once, and is "seperate" from the recipe finder app
//this crate is to push all my recipe .txt files that are in a folder into the database!
// example file name -- HighHighPasta1.txt

fn main() {
    let dbconnection =
        SqlLiteConnection::new(Connection::open("RecipeFinder.db").expect("Database not found"));
    dbconnection.create_table().expect("Could not create table");
    let dbconnection = Mutex::new(dbconnection);

    let entries: Vec<_> = fs::read_dir("recipes/")
        .expect("Could not find recipes folder")
        .collect();

    entries.par_iter().for_each(|entry| {
        let entry = entry.as_ref().expect("Could not read entry");
        let path = entry.path();
        let stem = path
            .file_stem() //this strips away the file path and the .txt at the end.
            .expect("No filename")
            .to_str()
            .expect("Invalid filename");
        let stem = stem.trim_end_matches(|c: char| c.is_numeric()); //this gets rid of hte trailing 1 or 2 on the filename

        //this section is pulling off the first section of the file name(calories) and creating the enum and removing from filename
        let (_calorie_range, rest) = if stem.starts_with("High") {
            //learned the underscore tells Rust that its okay that its unused
            (CalorieRange::High, stem.strip_prefix("High").unwrap())
        } else if stem.starts_with("Low") {
            (CalorieRange::Low, stem.strip_prefix("Low").unwrap())
        } else if stem.starts_with("Med") {
            (CalorieRange::Medium, stem.strip_prefix("Med").unwrap())
        } else {
            return;
        };

        //this section does the same, but with prep_time, the next metadata in the filename structure.
        let (_prep_time, last) = if rest.starts_with("High") {
            (Some(PrepTime::High), rest.strip_prefix("High").unwrap())
        } else if rest.starts_with("Low") {
            (Some(PrepTime::Low), rest.strip_prefix("Low").unwrap())
        } else if rest.starts_with("Med") {
            (Some(PrepTime::Medium), rest.strip_prefix("Med").unwrap())
        } else {
            (None, rest)
        };

        let category = match last {
            //grab the category
            "Amer" => Category::American,
            "Burger" => Category::Burger,
            "Chin" => Category::Chinese,
            "Egg" => Category::Egg,
            "Ital" => Category::Italian,
            "Pan" => Category::Pancake,
            "Pasta" => Category::Pasta,
            "Salad" => Category::Salad,
            "Smooth" => Category::Smoothie,
            _ => return,
        };
        let meal_type = match category {
            //deduct what the meal_type is
            Category::American => MealType::Dinner,
            Category::Burger => MealType::Lunch,
            Category::Chinese => MealType::Dinner,
            Category::Egg => MealType::Breakfast,
            Category::Italian => MealType::Dinner,
            Category::Pancake => MealType::Breakfast,
            Category::Pasta => MealType::Lunch,
            Category::Salad => MealType::Lunch,
            Category::Smoothie => MealType::Breakfast,
        };

        //now the file parsing for the rest of the recipe data
        let content = fs::read_to_string(&path).expect("Could not read file");
        let (name, calories, prep_time_mins, instructions, ingredients, servings) =
            parse_recipe(&content);
        let recipe = Recipe {
            name: name,
            meal_type: meal_type,
            calories: calories,
            prep_time_mins: prep_time_mins,
            category: category,
            ingredients: ingredients,
            instructions: instructions,
            servings: servings,
        };
        dbconnection
            .lock()
            .unwrap()
            .create_recipe(&recipe)
            .expect("Could not create recipe");
    })
}

//need these so that I can grab numbers from the file
fn extract_first_number(line: &str) -> Option<u32> {
    let digits: String = line
        .chars()
        .skip_while(|c| !c.is_numeric()) // skip until first digit
        .take_while(|c| c.is_numeric()) // take digits until non-digit
        .collect();
    digits.parse().ok()
}

//my files are not consistent with structure on time. some say hours, some just minutes. This helper function is to help me combine to be straight minutes
fn parse_time(line: &str) -> u32 {
    if line.contains("hour") || line.contains("hr") {
        let hours = extract_first_number(line).unwrap_or(0);
        // find minutes after "hour" or "hr"
        let after_hour = if line.contains("hour") {
            line.split("hour").last().unwrap_or("")
        } else {
            line.split("hr").last().unwrap_or("")
        };
        let minutes = extract_first_number(after_hour).unwrap_or(0);
        hours * 60 + minutes
    } else {
        extract_first_number(line).unwrap_or(0)
    }
}

//recipe parsing function.          name, cal, prep, instructions, ingredients
fn parse_recipe(content: &str) -> (String, u32, u32, Option<String>, Vec<String>, u32) {
    let lines: Vec<&str> = content.lines().collect(); //seperate into lines
    let name = lines[0].trim().to_string(); //line one is always the name
    let mut calories = 0u32; //tells rust that this 0 is a u32 specifically
    let mut prep_time_mins = 0u32;
    let mut servings = 1u32;
    let mut in_ingredients = false;
    let mut in_directions = false;
    let mut ingredients: Vec<String> = Vec::new();
    let mut instructions: Vec<String> = Vec::new();

    for line in &lines {
        if line.contains("alorie") {
            //locating calorie line
            calories = extract_first_number(line).unwrap_or(0);
        } else if line.contains("ime") {
            //locating time line
            prep_time_mins = parse_time(line);
        } else if line.contains("erving") {
            servings = extract_first_number(line).unwrap_or(0);
        } else if line.contains("INGREDIENTS") {
            //locates if line is in ingredients
            in_ingredients = true;
            in_directions = false;
        } else if line.contains("DIRECTIONS") {
            //locates if line is in directions
            in_ingredients = false;
            in_directions = true;
        } else if in_ingredients && !line.trim().is_empty() {
            //pushes line to ingredients
            ingredients.push(line.trim().to_string());
        } else if in_directions && !line.trim().is_empty() {
            //pushes line to directions
            instructions.push(line.trim().to_string());
        }
    }
    //there arent instructions in Smoothie. so0 this is the wrap for the Option type
    let instructions_text = if instructions.is_empty() {
        None
    } else {
        Some(instructions.join("\n"))
    };
    (
        name,
        calories,
        prep_time_mins,
        instructions_text,
        ingredients,
        servings,
    )
}

//majority of tests here
#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_parse_recipe() {
        let content = "Chicken Soup\nTotal Time= 30 min\nCalories= 400 per serving\n4 servings\n\nINGREDIENTS\n\nChicken\nNoodles\n\nDIRECTIONS\n\n1. Cook it.";
        let (name, calories, prep_time_mins, instructions, ingredients, servings) =
            parse_recipe(content);
        assert_eq!(name, "Chicken Soup");
        assert_eq!(calories, 400);
        assert_eq!(prep_time_mins, 30);
        assert_eq!(servings, 4);
        assert_eq!(ingredients, vec!["Chicken", "Noodles"]);
        assert!(instructions.is_some());
    }

    #[test]
    fn test_parse_smoothie() {
        let content = "Cherry Smoothie\nTime = 10 min\nCalories = 150\n 2 servings\nINGREDIENTS\nCherries\nMilk\nBlueberries";
        let (name, calories, prep_time_mins, instructions, ingredient, servings) =
            parse_recipe(content);
        assert_eq!(name, "Cherry Smoothie");
        assert_eq!(calories, 150);
        assert_eq!(servings, 2);
        assert_eq!(prep_time_mins, 10);
        assert_eq!(ingredient, vec!["Cherries", "Milk", "Blueberries"]);
        assert!(instructions.is_none());
    }
    #[test]
    fn extract_num() {
        assert_eq!(
            extract_first_number("Calories = 400 per serving"),
            Some(400)
        )
    }

    #[test]
    fn extract_num_empty() {
        assert_eq!(extract_first_number("no numbers here"), None)
    }

    #[test]
    fn parse_time_1() {
        assert_eq!(parse_time("Total Time = 45 min"), 45)
    }

    #[test]
    fn parse_time_2() {
        assert_eq!(parse_time("Total Time = 1 hour and 30 min"), 90)
    }

    #[test]
    fn parse_time_3() {
        assert_eq!(parse_time("Total time = 2 hrs 20 minutes"), 140)
    }
}
