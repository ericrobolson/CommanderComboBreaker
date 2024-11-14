mod app_state;

use app_state::AppState;
use eframe::egui::{self, Ui};

pub struct App {
    state: AppState,
}
impl App {
    pub fn run() {
        let app = App {
            state: AppState::SearchBuilder {
                card_limit: 2,
                colors: "Abzan".to_string(),
            },
        };
        main(app).unwrap();
    }

    pub fn run_combo_builder(cards: Vec<(String, u32)>, combos: Vec<Vec<String>>) {
        let app = App {
            state: AppState::ComboFinder {
                search: "".to_string(),
                cards,
                combo_card: None,
                combos,
            },
        };
        main(app).unwrap();
    }
}

fn main(app: App) -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(app))
        }),
    )
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            self.state.render(ui).unwrap();

            // List out combos

            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}
