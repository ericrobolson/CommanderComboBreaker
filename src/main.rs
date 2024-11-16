// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::task;
use std::{collections::HashSet, io::Write};

mod app;
mod crawler;
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

    let mut env_args = std::env::args().collect::<Vec<_>>();
    env_args.remove(0);

    if env_args.is_empty() {
        app::App::run();
    } else {
        let card = Some(env_args.join(" "));
        let colors = vec![Color::Blue, Color::Red, Color::Green];

        let mut search = crawler::CrawlerTask::new(colors.clone(), card.clone());
        loop {
            search.update();
            if search.result.is_some() {
                break;
            }
        }
        let result = search.result.clone().unwrap();
        let cards: Vec<String> = result
            .cards
            .iter()
            .filter(|(combo_card, _)| {
                if let Some(original_card) = card.clone() {
                    &original_card != combo_card
                } else {
                    true
                }
            })
            .map(|(card, _)| card.clone())
            .collect();

        let mut tasks = vec![];
        for card in cards {
            let task = crawler::CrawlerTask::new(colors.clone(), Some(card));
            tasks.push(task);
        }

        loop {
            for task in tasks.iter_mut() {
                task.update();
            }

            if tasks.iter().all(|task| task.result.is_some()) {
                break;
            }
        }
        let max_combos = 3;
        let mut card_counts = std::collections::HashMap::new();
        let mut total_combos = vec![];

        for task in tasks {
            let result = task.result.clone().unwrap();
            let mut combos = result.combos.clone();

            for i in (0..combos.len()).rev() {
                if combos[i].len() > max_combos {
                    combos.remove(i);
                }
            }

            for combo in combos.iter() {
                for card in combo.iter() {
                    let count = card_counts.entry(card.clone()).or_insert(0);
                    *count += 1;
                }

                if combo.is_empty() == false {
                    total_combos.push(combo.clone());
                }
            }
        }

        let minimum_combo_count = 20;
        for (card_to_check, count) in card_counts.iter() {
            if *count < minimum_combo_count && card != Some(card_to_check.clone()) {
                // Remove from total_combos anything that uses that card
                total_combos = total_combos
                    .iter()
                    .filter(|combo| !combo.contains(card_to_check))
                    .cloned()
                    .collect();
            }
        }

        total_combos.dedup();

        let mut cards = HashSet::new();
        if let Some(card) = card.clone() {
            cards.insert(card.clone());
        }
        for combo in total_combos.iter() {
            for card in combo.iter() {
                cards.insert(card.clone());
            }
        }

        // Now we have all the cards that are in combos, let's filter the combos down
        let mut filtered_combos = vec![];
        for combo in total_combos.iter() {
            let mut make_combo = true;

            if let Some(card) = card.clone() {
                if !combo.contains(&card) {
                    make_combo = false;
                }
            }

            if make_combo {
                filtered_combos.push(combo.clone());
            }
        }

        let card_count = cards.len();
        let cards = cards.iter().cloned().collect::<Vec<String>>().join("\n");
        let combo_count = filtered_combos.len();
        let combos = filtered_combos
            .iter()
            .map(|combo| combo.join(", "))
            .collect::<Vec<String>>()
            .join("\n");
        // Save to disk
        let mut file = std::fs::File::create("combos.txt").unwrap();

        let contents = format!(
            "-----\nCards: {}\n-----\n{}\n\n\n\n-----\nCombos: {}\n-----\n{}",
            card_count, cards, combo_count, combos
        );

        file.write_all(contents.as_bytes()).unwrap();
    }

    Ok(())
}
