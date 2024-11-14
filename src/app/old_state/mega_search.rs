use super::Color;

use crate::crawler::CrawlerTask;
use eframe::egui::{self, Ui};
use std::collections::HashMap;

pub struct MegaSearch {
    cards: Vec<String>,
    wip_card: String,
    colors: HashMap<Color, bool>,
}
impl MegaSearch {
    pub fn new() -> Self {
        Self {
            cards: vec![],
            wip_card: String::default(),
            colors: Color::check_list(),
        }
    }
}

pub fn render(ui: &mut Ui, ctx: &egui::Context, state: &mut MegaSearch) {
    // Render colors to search for
    ui.horizontal(|ui| {
        for (color, selected) in state.colors.iter_mut() {
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

    // Render card input
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let card_label = ui.label("Card: ");
                ui.text_edit_singleline(&mut state.wip_card)
                    .labelled_by(card_label.id);
                if ui
                    .button("Add Card")
                    .on_hover_text("Add card to search")
                    .clicked()
                {
                    state.cards.push(state.wip_card.clone());
                    state.cards.sort();
                    state.wip_card = String::default();
                }
            });

            ui.label("Cards to search for:");
            let mut cards_to_remove = vec![];
            for card in state.cards.iter() {
                let button = ui.button(card).on_hover_text("Remove card");
                if button.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
                }
                if button.clicked() {
                    cards_to_remove.push(card.clone());
                }
            }
            for card in cards_to_remove {
                state.cards.retain(|c| c != &card);
            }
        });
    });
}
