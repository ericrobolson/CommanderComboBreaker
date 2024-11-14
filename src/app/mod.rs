mod states;

use eframe::egui::{self, Ui};
pub enum StateResult {
    Noop,
    Change(Box<dyn State>),
}
pub trait State {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> StateResult;
}

pub struct App {
    state: Box<dyn State>,
    state_history: Vec<Box<dyn State>>,
}
impl App {
    pub fn run() {
        let app = App {
            state_history: vec![],
            state: Box::new(states::MegaSearch::new()),
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
        "CommanderComboBreaker",
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
            self.state.update(ui, ctx);
        });
    }
}
