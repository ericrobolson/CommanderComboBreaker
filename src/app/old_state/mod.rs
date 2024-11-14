mod mega_search;

use crate::crawler::CrawlerTask;
use eframe::egui::{self, Ui};
use mega_search::MegaSearch;
use std::collections::HashMap;

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

    pub fn check_list() -> HashMap<Color, bool> {
        let mut colors = HashMap::new();
        colors.insert(Color::White, false);
        colors.insert(Color::Blue, false);
        colors.insert(Color::Black, false);
        colors.insert(Color::Red, false);
        colors.insert(Color::Green, false);
        colors.insert(Color::Colorless, false);
        colors
    }
}

pub enum AppState {
    MegaSearch(MegaSearch),
    SearchBuilder {
        card_limit: u32,
        colors: String,
    },
    Searching {
        task: CrawlerTask,
    },
    ComboFinder {
        search: String,
        cards: Vec<(String, u32)>,
        combo_card: Option<String>,
        combos: Vec<Vec<String>>,
    },
}
impl AppState {
    pub fn multi_search() -> Self {
        let mut colors = HashMap::new();
        colors.insert(Color::White, false);
        colors.insert(Color::Blue, false);
        colors.insert(Color::Black, false);
        colors.insert(Color::Red, false);
        colors.insert(Color::Green, false);
        colors.insert(Color::Colorless, false);

        AppState::MegaSearch(MegaSearch::new())
    }

    pub fn render(&mut self, ui: &mut Ui, ctx: &egui::Context) -> eframe::Result {
        match self {
            AppState::SearchBuilder { card_limit, colors } => {
                // Render search builder
                ui.horizontal(|ui| {
                    let card_limit_label = ui.label("Card Limit: ");
                    ui.add(egui::Slider::new(card_limit, 1..=10).text("Card Limit"))
                        .labelled_by(card_limit_label.id);
                });

                ui.horizontal(|ui| {
                    let colors_label = ui.label("Colors: ");
                    ui.text_edit_singleline(colors).labelled_by(colors_label.id);
                });

                if ui.button("Search").clicked() {
                    *self = AppState::Searching {
                        task: CrawlerTask::new(colors.clone(), *card_limit as usize),
                    };
                }
            }
            AppState::ComboFinder {
                search,
                cards,
                combo_card,
                combos,
            } => {
                // Render combo finder
                ui.horizontal(|ui| {
                    let name_label = ui.label("Search: ");
                    ui.text_edit_singleline(search).labelled_by(name_label.id);
                });

                let label = combo_card.as_deref().unwrap_or("None");

                ui.label(format!("Selected card: {}", label));
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        for (name, count) in cards.iter().take(10) {
                            if ui.button(format!("{}: {}", name, count)).clicked() {
                                *combo_card = Some(name.clone());
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        if let Some(combo_card) = combo_card.as_ref() {
                            let matching_combos =
                                combos.iter().filter(|combo| combo.contains(&combo_card));
                            ui.vertical(|ui| {
                                matching_combos.clone().for_each(|combo| {
                                    ui.label(format!("{:?}", combo));
                                });
                            });

                            // Now get a count of each card in the combos
                            let mut card_counts = std::collections::HashMap::new();
                            for combo in matching_combos {
                                for card in combo {
                                    let count = card_counts.entry(card).or_insert(0);
                                    *count += 1;
                                }
                            }

                            // Print top 10 cards
                            let mut cards = vec![];
                            for (name, count) in card_counts {
                                cards.push((name, count));
                            }
                            cards.sort_unstable_by_key(|a| (a.1, a.0));
                            cards.reverse();
                            ui.vertical(|ui| {
                                for (name, count) in cards.iter().take(10) {
                                    ui.label(format!("{}: {}", name, count));
                                }
                            });
                        }
                    });
                });
            }
            AppState::Searching { task } => {
                // Perform update
                task.update();

                // Render searching
                ui.label("Searching...");
                ui.label(format!("Found {} combos", task.combos_found()));

                if ui.button("Stop Search").clicked() {
                    task.stop();
                }

                if let Some(result) = task.result.take() {
                    *self = AppState::ComboFinder {
                        search: "".to_string(),
                        cards: result.cards,
                        combo_card: None,
                        combos: result.combos,
                    };
                }
            }
            AppState::MegaSearch(state) => {
                mega_search::render(ui, ctx, state);
            }
        }
        Ok(())
    }
}
