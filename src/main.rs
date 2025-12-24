#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use catppuccin_egui::{MACCHIATO, Theme};
use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, Vec2, ViewportBuilder};
use tracing::info;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = ViewportBuilder::default()
        .with_resizable(false)
        .with_inner_size(Vec2::new(760., 320.));
    eframe::run_native(
        "Wavebreaker Installer",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .expect("The.. the app should run...");
}

#[derive(Debug, Default)]
enum InstallationStep {
    #[default]
    Start,
}

#[derive(Default)]
struct MyEguiApp {
    current_step: InstallationStep,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        const WAVEBREAKER_THEME: Theme = Theme {
            sky: Color32::from_rgb(0, 158, 255),
            yellow: Color32::from_rgb(255, 199, 119),
            red: Color32::from_rgb(255, 83, 112),
            base: Color32::from_rgb(33, 35, 55),
            mantle: Color32::from_rgb(25, 26, 42),
            crust: Color32::from_rgb(19, 20, 33),
            text: Color32::from_rgb(200, 211, 245),
            ..MACCHIATO
        };
        catppuccin_egui::set_theme(&cc.egui_ctx, WAVEBREAKER_THEME);

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Inter".to_owned(),
            std::sync::Arc::new(FontData::from_static(include_bytes!(
                "../fonts/Inter-Regular.ttf"
            ))),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Inter".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
