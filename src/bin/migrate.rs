use recipe_finder::db::*;
use recipe_finder::filters::{CalorieRange, PrepTime};
use recipe_finder::recipes::*;
use rusqlite::Connection;
use std::fs;
use std::path::Path;

//this crate is to push all my recipe .txt files that are in a folder into the database!
// example file name -- HighHighPasta1.txt

fn main() {
    let conn = Connection::open("RecipeFinder.db").expect("Database not found");
    let mut dbconnection = SqlLiteConnection::new(&conn);
    dbconnection.create_table().expect("Could not create table");

    let entries = fs::read_dir("recipes/").expect("Could not find recipes folder");

    for entry in entries {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();
        let stem = path
            .file_stem() //this strips away the file path and the .txt at the end.
            .expect("No filename")
            .to_str()
            .expect("Invalid filename");
        let stem = stem.trim_end_matches(|c: char| c.is_numeric()); //this gets rid of hte trailing 1 or 2 on the filename

        //this section is pulling off the first section of the file name(calories) and creating the enum and removing from filename
        let (calorie_range, rest) = if stem.starts_with("High") {
            (CalorieRange::High, stem.strip_prefix("High").unwrap())
        } else if stem.starts_with("Low") {
            (CalorieRange::Low, stem.strip_prefix("Low").unwrap())
        } else if stem.starts_with("Med") {
            (CalorieRange::Medium, stem.strip_prefix("Med").unwrap())
        } else {
            continue;
        };

        //this section does the same, but with prep_time, the next metadata in the filename structure.
        let (prep_time, last) = if rest.starts_with("High") {
            (Some(PrepTime::High), rest.strip_prefix("High").unwrap())
        } else if rest.starts_with("Low") {
            (Some(PrepTime::Low), rest.strip_prefix("Low").unwrap())
        } else if rest.starts_with("Med") {
            (Some(PrepTime::Medium), rest.strip_prefix("Med").unwrap())
        } else {
            (None, rest)
        };

        println!("stem={:?} rest={:?} last={:?}", stem, rest, last); //debug check

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
            _ => continue,
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

        //now the file parsing for the rest of hte recipe data
        let content = fs::read_to_string(&path).expect("Could not read file");
        let lines: Vec<&str> = content.lines().collect(); //split the file into lines
    }
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
        let after_hour = line
            .split("hour")
            .last()
            .or(line.split("hr").last())
            .unwrap_or("");
        let minutes = extract_first_number(after_hour).unwrap_or(0);
        hours * 60 + minutes
    } else {
        extract_first_number(line).unwrap_or(0)
    }
}

//recipe parsing function.
fn parse_recipe(content: &str) -> (String, u32, u32) {}
