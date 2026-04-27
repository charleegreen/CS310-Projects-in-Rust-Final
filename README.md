# Rust Recipe Finder

## Project Overview
The goal of this project was to rewrite a previous Python project in Rust. The original Python version relied on deeply nested functions, global mutable state, and a tkinter GUI. This Rust rewrite restructures the logic into clean modules, replaces the GUI with a CLI decision tree, and uses a SQLite database for recipe storage and querying.

The app walks the user through a series of questions about meal type, category, calorie range, and prep time, then displays a randomly selected matching recipe.

A key part of this project was the `migrate` binary, which parses 139 original `.txt` recipe files and loads them into the SQLite database using parallel processing via Rayon.

## Setup Instructions
1. Clone the repository
2. Make sure Rust is installed (https://rustup.rs)
3. Run the migration to populate the database:
```bash
   cargo run --bin migrate
```
4. Run the app:
```bash
   cargo run
```

## Usage Example
- Run the app and follow the prompts
- Select meal type, category, calorie range, and prep time
- A matching recipe will be displayed

## Contributors & License
- Charlee Green
- MIT License 