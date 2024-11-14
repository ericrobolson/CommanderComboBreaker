// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod crawler;
mod search_result;
mod web_page;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

impl Color {
    pub fn all() -> Vec<Color> {
        vec![
            Color::White,
            Color::Blue,
            Color::Black,
            Color::Red,
            Color::Green,
            Color::Colorless,
        ]
    }

    pub fn check_list() -> std::collections::HashMap<Color, bool> {
        let mut colors = std::collections::HashMap::new();
        colors.insert(Color::White, false);
        colors.insert(Color::Blue, false);
        colors.insert(Color::Black, false);
        colors.insert(Color::Red, false);
        colors.insert(Color::Green, false);
        colors.insert(Color::Colorless, false);
        colors
    }
}

fn main() -> Result<(), String> {
    // app::App::run();
    let db = rusqlite::Connection::open("ccb.sqlite").unwrap();
    db.execute(
        "CREATE TABLE IF NOT EXISTS html_page (
            id   INTEGER PRIMARY KEY,
            url  TEXT NOT NULL,
            html_body TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    app::App::run();

    Ok(())
}

fn commander_spellbook_search(commander_name: &str, color: &str) -> String {
    let commander_name = html_escape::encode_text(commander_name);
    let color = html_escape::encode_text(color);
    format!(
        "https://commanderspellbook.com/search/?q=card%3A%22{}%22+ci%3A{}+legal%3Acommander",
        commander_name, color
    )
}
