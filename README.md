# Rust Recipe Finder

This is my final project for CS310 Programming Projects in Rust at Western Colorado University. I took a recipe finder app I made before in Python and rewrote it in Rust.

## What it does

The app asks you a few questions about what you want to eat, then picks a random recipe that matches your answers.

## Why I rewrote it

My original Python version had a lot of nested functions, used global variables and had a tkinter GUI. For the Rust version I:

- Split the code into separate files (modules) so it's more organized
- Got rid of the global variables
- Replaced the GUI with simple questions in the terminal
- Stored all the recipes in a SQLite database instead of reading text files every time

## Moving the recipes into the database

My old recipes were saved as 139 separate `.txt` files. I wrote a separate program (`migrate`) that reads all of them and puts them into the SQLite database. It uses Rayon to read the files in parallel so it runs faster.

## Setup

1. Clone the repository
2. Make sure Rust is installed (https://rustup.rs)
3. Run the migration to fill the database:
```
cargo run --bin migrate
```
4. Run the app:
```
cargo run
```

## How to use it

Run the app and answer the questions by typing a number:

1. **Meal type:** Breakfast, Lunch or Dinner
2. **Category:** depends on the meal
   - Breakfast: Eggs, Pancakes or Smoothie
   - Lunch: Salad, Burger or Pasta
   - Dinner: Italian, Chinese or American
3. **Calories:** under 200, 201–500 or over 500
4. **Prep time:** under 25 min, 26–45 min or over 45 min

The app then shows you a random recipe that matches. If nothing matches, it tells you to try a different combo.

## Files

- `main.rs` asks the questions and shows the result
- `db.rs` handles the SQLite database connection
- `filters.rs` finds recipes that match your answers
- `recipes.rs` holds the recipe data and options
- `display.rs` controls how a recipe gets printed
- `bin/migrate.rs` moves the old text files into the database

## Tools I used

- Rust
- rusqlite (for SQLite)
- Rayon (for parallel processing)
- rand (for picking a random recipe)

## What I learned

- How to break a messy program into clean modules
- How Rust's ownership rules work and why they make global variables harder
- How to use a SQLite database from Rust
- How to use Rayon to process lots of files at once
- Select meal type, category, calorie range, and prep time
- A matching recipe will be displayed

## Contributors & License
- Charlee Green
- MIT License 
