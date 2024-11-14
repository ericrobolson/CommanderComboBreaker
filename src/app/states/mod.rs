use super::{State, StateResult};
use crate::{
    crawler::{CrawlerResult, CrawlerTask},
    Color,
};
use eframe::egui::{self};

pub struct MegaSearch {
    search: String,
    cards: Vec<(String, bool)>,
    tasks: Vec<(String, CrawlerTask)>,
    results: std::collections::HashMap<String, CrawlerResult>,
    color_checkboxes: std::collections::HashMap<Color, bool>,
}
impl MegaSearch {
    pub fn new() -> Self {
        Self {
            search: String::default(),
            cards: vec![],
            tasks: vec![],
            results: std::collections::HashMap::new(),
            color_checkboxes: Color::check_list(),
        }
    }
}

impl MegaSearch {
    fn selected_cards(&self) -> Vec<String> {
        self.cards
            .iter()
            .filter_map(|(name, selected)| if *selected { Some(name.clone()) } else { None })
            .collect()
    }
    fn selected_colors(&self) -> Vec<Color> {
        self.color_checkboxes
            .iter()
            .filter_map(|(color, selected)| if *selected { Some(*color) } else { None })
            .collect()
    }

    fn add_card(&mut self, card: String) {
        let task = CrawlerTask::new(self.selected_colors(), card.clone());
        self.tasks.push((card.clone(), task));
        self.cards.push((card.clone(), false));
        self.cards.sort();
    }

    fn render_search_box(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            for (color, selected) in self.color_checkboxes.iter_mut() {
                let color_name = match color {
                    Color::White => "White",
                    Color::Blue => "Blue",
                    Color::Black => "Black",
                    Color::Red => "Red",
                    Color::Green => "Green",
                    Color::Colorless => "Colorless",
                };
                ui.checkbox(selected, color_name);
            }
        });
        ui.horizontal(|ui| {
            ui.label("Search: ");
            ui.text_edit_singleline(&mut self.search);
            if ui
                .button("Add Card")
                .on_hover_text("Add card to search")
                .clicked()
            {
                // Start a new search task
                self.add_card(self.search.clone());
                self.search = String::default();
            }
        });

        ui.label(&format!("Background searches: {}", self.tasks.len()));
    }

    fn render_combo_selector(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("Clear selected combo cards").clicked() {
                self.cards.iter_mut().for_each(|(_name, selected)| {
                    *selected = false;
                });
            }
        });
        let mut cards_to_remove = vec![];
        for (card, selected) in self.cards.iter_mut() {
            ui.horizontal(|ui| {
                ui.checkbox(selected, "View Combos");

                let button = ui.button("Remove").on_hover_text("Remove card");
                if button.clicked() {
                    cards_to_remove.push(card.clone());
                }

                ui.label(format!("{}", card));
            });
        }

        for card in cards_to_remove {
            self.cards.retain(|(name, _selected)| name != &card);
        }
    }

    fn render_combos(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let selected_cards = self.selected_cards();
        let mut cards_to_add = vec![];

        for (name, result) in self.results.iter() {
            if !selected_cards.contains(name) {
                continue;
            }

            ui.collapsing(name, |ui| {
                for (card, num_results) in result.cards.iter() {
                    ui.horizontal(|ui| {
                        if ui.button("Add").clicked() {
                            cards_to_add.push(card.clone());
                        }
                        ui.label(format!("{}: {}", card, num_results));
                    });
                }
            });
        }

        for card in cards_to_add {
            self.add_card(card);
        }
    }
}

impl State for MegaSearch {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> StateResult {
        // Process tasks, removing them as they complete
        for i in (0..self.tasks.len()).rev() {
            self.tasks[i].1.update();
            if self.tasks[i].1.result.is_some() {
                let name = self.tasks[i].0.clone();
                let result = self.tasks[i].1.result.take().unwrap();
                self.results.insert(name, result);
                self.tasks.remove(i);
            }
        }

        ui.heading("Card Search");
        self.render_search_box(ui, ctx);

        ui.separator();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                self.render_combo_selector(ui, ctx);
            });
            ui.separator();
            ui.vertical(|ui| {
                self.render_combos(ui, ctx);
            });
        });

        StateResult::Noop
    }
}
