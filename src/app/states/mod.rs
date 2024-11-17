use super::{State, StateResult};
use crate::{
    crawler::{CrawlerResult, CrawlerTask},
    Color,
};
use eframe::egui::{self};

pub struct MegaSearch {
    search: String,
    cards: Vec<(String, bool, bool)>,
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
            .filter_map(|(name, view_combos, _)| {
                if *view_combos {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    fn selected_colors(&self) -> Vec<Color> {
        self.color_checkboxes
            .iter()
            .filter_map(|(color, selected)| if *selected { Some(*color) } else { None })
            .collect()
    }

    fn add_card(&mut self, card: String) {
        let card_name = if card.is_empty() {
            None
        } else {
            Some(card.clone())
        };
        let task = CrawlerTask::new(
            self.selected_colors(),
            card_name,
            Some(crate::crawler::Format::Commander),
        );
        self.tasks.push((card.clone(), task));
        self.cards.push((card.clone(), false, false));
        self.cards.sort();
    }

    fn recalculate_matching_combos(&mut self) {}

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
                self.cards
                    .iter_mut()
                    .for_each(|(_name, selected, in_combo_pool)| {
                        *selected = false;
                        *in_combo_pool = false;
                    });
            }
        });
        let mut cards_to_remove = vec![];
        let mut should_recalculate_combos = false;
        for (card, view_combos, in_combo_pool) in self.cards.iter_mut() {
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.checkbox(view_combos, "View Combos");
                if ui.checkbox(in_combo_pool, "In Combo Pool").changed() {
                    should_recalculate_combos = true;
                }

                let button = ui.button("Remove").on_hover_text("Remove card");
                if button.clicked() {
                    cards_to_remove.push(card.clone());
                }

                ui.label(format!("{}", card));
            });
        }

        for card in cards_to_remove {
            should_recalculate_combos = true;
            self.cards
                .retain(|(name, _view_combos, _in_combo_pool)| name != &card);
        }

        if should_recalculate_combos {
            self.recalculate_matching_combos();
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
