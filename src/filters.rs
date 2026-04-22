use crate::db::SqlLiteConnection;
use crate::recipes::{Category, MealType, Recipe};
use rusqlite::{Error, named_params};

pub fn filtering(
    //creates a vector of possible recipes that match the requests of the user
    conn: &SqlLiteConnection,
    calories: CalorieRange,
    prep_time: PrepTime,
    category: Category,
    meal_type: MealType, //these are all the requests from the user
) -> Result<Vec<Recipe>, Error> {
    let (cal_min, cal_max) = calories.bounds(); //making the range into actual numbers for Querying
    let (prep_min, prep_max) = prep_time.bounds();
    //prepare universal query statement
    let mut selectstmt = conn.conn.prepare("Select name, calories, instructions, meal_type, prep_time_mins, category,ingredients, servings FROM recipe 
        WHERE calories >= :cal_min 
        AND calories <= :cal_max
        AND prep_time_mins >= :prep_min
        AND prep_time_mins <= :prep_max
        AND category = :category
        AND meal_type = :meal_type")?;

    let recipelist = selectstmt.query_map(
        named_params! { //put in the actual requests into the query statement
            ":cal_min": cal_min,
            ":cal_max": cal_max,
            ":prep_min": prep_min,
            ":prep_max": prep_max,
            ":category": category.to_string(),
            ":meal_type": meal_type.to_string(),
        },
        |row| {
            //orginize the data from the dataset into recipe structs
            let meal_type_str: String = row.get(3)?; //get the index
            let meal_type = meal_type_str
                .parse::<MealType>() //parses the string into the enum type
                .map_err(|e| rusqlite::Error::InvalidParameterName(e))?; //error handling
            let category_str: String = row.get(5)?; //same for category
            let category = category_str
                .parse::<Category>()
                .map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
            let ingredients_str: String = row.get(6)?; //grabs the comma seperated string
            let ingredients = ingredients_str.split(',').map(|s| s.to_string()).collect();
            // splits the string by the commas, collects them into one vector
            Ok(Recipe {
                name: row.get(0)?,
                calories: row.get(1)?,
                instructions: row.get(2)?,
                meal_type, //now just need meal_type
                prep_time_mins: row.get(4)?,
                category,
                ingredients,
                servings: row.get(7)?,
            })
        },
    )?;
    Ok(recipelist.collect::<Result<Vec<Recipe>, _>>()?) //.collect puts the structs into the vector
}

#[derive(Debug, PartialEq)]
pub enum CalorieRange {
    //this goes here because it is used for the flitering, not necessary for the recipe struct
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq)]
pub enum PrepTime {
    Low,
    Medium,
    High,
}

// Sets the ranges for the calories and for the prep times!!!
impl CalorieRange {
    pub fn bounds(&self) -> (u32, u32) {
        match self {
            Self::Low => (0, 200),
            Self::Medium => (201, 500),
            Self::High => (501, u32::MAX),
        }
    }
}

impl PrepTime {
    pub fn bounds(&self) -> (u32, u32) {
        match self {
            Self::Low => (0, 25),
            Self::Medium => (26, 45),
            Self::High => (46, u32::MAX),
        }
    }
}
