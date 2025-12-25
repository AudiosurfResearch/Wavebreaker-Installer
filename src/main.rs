#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{borrow::Cow, collections::BTreeMap, io::Cursor, path};

use anyhow::{self, Context, bail};
use catppuccin_egui::{MACCHIATO, Theme};
use eframe::egui::{
    self, Button, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId,
    InnerResponse, Layout, Margin, ProgressBar, Response, RichText, Stroke, TextEdit, TextStyle,
    Ui, Vec2, ViewportBuilder, ViewportCommand,
};
use egui_extras::install_image_loaders;
use lazy_async_promise::{
    BoxedSendError, ImmediateValuePromise, ImmediateValueState, Progress, ProgressTrackedImValProm,
    StringStatus,
};
use steamlocate::{Library, SteamDir};
use tracing::{info, instrument};
use tracing_subscriber;

const WAVEBREAKER_THEME: Theme = Theme {
    sky: Color32::from_rgb(0, 158, 255),
    yellow: Color32::from_rgb(255, 199, 119),
    red: Color32::from_rgb(255, 83, 112),
    green: Color32::from_rgb(54, 211, 153),
    base: Color32::from_rgb(33, 35, 55),
    mantle: Color32::from_rgb(25, 26, 42),
    crust: Color32::from_rgb(19, 20, 33),
    text: Color32::from_rgb(200, 211, 245),
    ..MACCHIATO
};

#[instrument]
fn get_audiosurf_path() -> anyhow::Result<String> {
    info!("Trying to find Steam...");
    let steamdir = SteamDir::locate()?;
    info!("Steam at {:?}", steamdir.path());
    if let Some((app, library)) = steamdir.find_app(12900)? {
        if app.app_id == 12900 {
            let app_dir = Library::resolve_app_dir(&library, &app);
            info!("Audiosurf found at {:?}", app_dir);
            return Ok(app_dir.to_str().unwrap().to_owned());
        }
    }
    bail!("Game not found, make sure it's installed through Steam!");
}

#[instrument]
fn is_valid_audiosurf_folder(path: &str) -> bool {
    info!("Validating: {:?}", path::Path::new(path));
    return path::Path::new(path).join("engine").exists();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = ViewportBuilder::default()
        .with_resizable(false)
        .with_inner_size(Vec2::new(640., 300.));
    eframe::run_native(
        "Wavebreaker Installer",
        native_options,
        Box::new(|cc| Ok(Box::new(EguiApp::new(cc)))),
    )
    .expect("The.. the app should run...");
}

#[derive(Debug, Default, Clone)]
enum InstallationStep {
    #[default]
    Start,
    LocateGame {
        autolocation_ran: bool,
        audiosurf_path: Option<String>,
    },
    Installing,
}

#[derive(Default)]
struct EguiApp {
    current_step: InstallationStep,
    install_task: Option<ProgressTrackedImValProm<(), Cow<'static, str>>>,
    game_path: String,
}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        catppuccin_egui::set_theme(&cc.egui_ctx, WAVEBREAKER_THEME);
        cc.egui_ctx.style_mut(|style| {
            style.visuals.widgets.inactive.corner_radius = CornerRadius::same(8);
            style.visuals.widgets.active.corner_radius = CornerRadius::same(8);
            style.visuals.widgets.hovered.corner_radius = CornerRadius::same(8);

            style.visuals.widgets.inactive.weak_bg_fill = WAVEBREAKER_THEME.sky;
            style.visuals.widgets.active.weak_bg_fill = Color32::from_rgb(0, 135, 219);
            style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(0, 135, 219);

            style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
            style.visuals.widgets.active.bg_stroke = Stroke::NONE;
            style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;

            style.visuals.widgets.hovered.fg_stroke.color = Color32::WHITE;
            style.visuals.widgets.active.fg_stroke.color = Color32::WHITE;
            style.visuals.widgets.inactive.fg_stroke.color = Color32::WHITE;

            style.visuals.widgets.active.expansion = -0.3;

            style.spacing.window_margin = Margin::same(20);
        });

        use FontFamily::{Monospace, Proportional};
        let text_styles: BTreeMap<TextStyle, FontId> = [
            (TextStyle::Heading, FontId::new(25.0, Proportional)),
            (TextStyle::Body, FontId::new(16.0, Proportional)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(14.0, Proportional)),
            (TextStyle::Small, FontId::new(8.0, Proportional)),
        ]
        .into();
        cc.egui_ctx.all_styles_mut(|style| {
            style.text_styles = text_styles.clone();
        });

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Inter".to_owned(),
            std::sync::Arc::new(FontData::from_static(include_bytes!(
                "../fonts/Inter-Regular.ttf"
            ))),
        );
        fonts.font_data.insert(
            "Inter-SemiBold".to_owned(),
            std::sync::Arc::new(FontData::from_static(include_bytes!(
                "../fonts/Inter-SemiBold.ttf"
            ))),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Inter".to_owned());
        let mut bold_family = BTreeMap::new();
        bold_family.insert(
            FontFamily::Name("semibold".into()),
            vec!["Inter-SemiBold".to_owned()],
        );
        fonts.families.append(&mut bold_family);
        cc.egui_ctx.set_fonts(fonts);

        install_image_loaders(&cc.egui_ctx);

        Self::default()
    }

    #[instrument]
    fn install(path: String) -> ProgressTrackedImValProm<(), Cow<'static, str>> {
        ProgressTrackedImValProm::new(
            |s| {
                ImmediateValuePromise::new(async move {
                    s.send(StringStatus::new(
                        Progress::from_percent(0),
                        "Checking folder validity".into(),
                    ))
                    .await?;

                    if !is_valid_audiosurf_folder(&path) {
                        return Err(BoxedSendError(
                            anyhow::anyhow!("Invalid Audiosurf folder!").into(),
                        ));
                    }

                    s.send(StringStatus::new(
                        Progress::from_percent(25),
                        "Getting release from GitHub".into(),
                    ))
                    .await?;

                    let target = "https://github.com/AudiosurfResearch/Wavebreaker-Hook/releases/latest/download/Wavebreaker-Package.zip";
                    let response = reqwest::get(target).await?;
                    let content = response.bytes().await?;

                    s.send(StringStatus::new(
                        Progress::from_percent(50),
                        "Checking for and deleting old files".into(),
                    ))
                    .await?;

                    info!("Checking for old files");
                    let old_files = vec![
                        "engine\\channels\\Wavebreaker-Hook.dll",
                        "engine\\channels\\wavebreakerclient.dll",
                        "engine\\channels\\wavebreaker_client.dll",
                        "engine\\SongSelector\\RadioBrowser.cgr",
                        "engine\\Wavebreaker.toml",
                        "engine\\Wavebreaker-Hook.ini",
                        "engine\\Wavebreaker-Client.toml",
                    ];
                    for file in old_files {
                        let file_path = path::Path::new(&path).join(file);
                        if file_path.exists() {
                            info!("Removing {}", file);
                            std::fs::remove_file(file_path)
                                .context("Failed to remove old files")
                                .map_err(|e| BoxedSendError(e.into()))?;
                        }
                    }

                    s.send(StringStatus::new(
                        Progress::from_percent(75),
                        "Extracting files".into(),
                    ))
                    .await?;

                    let target_dir = path::Path::new(&path).join("engine");
                    zip_extract::extract(Cursor::new(content.to_vec()), &target_dir, true)
                        .context("Failed to extract zip file")
                        .map_err(|e| BoxedSendError(e.into()))?;

                    Ok(())
                })
            },
            2000,
        )
    }
}

/// widget that draws the Wavebreaker logo
fn header_bar(ui: &mut Ui) -> InnerResponse<Response> {
    ui.horizontal(|ui| {
        ui.add_sized(
            [32., 32.],
            egui::Image::new(egui::include_image!("../img/logo.svg")),
        );
        ui.add_space(4.);
        ui.heading(
            RichText::new("Wavebreaker")
                .font(FontId::new(22., FontFamily::Name("semibold".into()))),
        )
    })
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let frame = egui::Frame::window(&ctx.style()).inner_margin(Margin::same(16));
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            header_bar(ui);
            ui.add_space(16.);
            let mut installation_step = self.current_step.clone();
            match &mut installation_step {
                InstallationStep::Start => {
                    ui.label(
                        RichText::new("Welcome to the installer for Wavebreaker!")
                            .font(FontId::new(16., FontFamily::Name("semibold".into()))),
                    );
                    ui.add_space(8.);
                    ui.label("This program will install the Wavebreaker client mod for you, which will let the game connect to the custom server.");
                    ui.add_space(8.);
                    ui.label("Press the \"Next\" button to get started!");

                    ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                        if ui
                            .add_sized([64., 32.], Button::new(RichText::new("Next")))
                            .clicked()
                        {
                            self.current_step = InstallationStep::LocateGame { autolocation_ran: false, audiosurf_path: get_audiosurf_path().ok() };
                        }
                    });
                }
                InstallationStep::LocateGame { autolocation_ran, audiosurf_path } => {
                    if !*autolocation_ran {
                        self.current_step = InstallationStep::LocateGame { autolocation_ran: true, audiosurf_path: audiosurf_path.clone() };
                        self.game_path = audiosurf_path.clone().unwrap_or_default();
                    }
                    match audiosurf_path {
                        Some(_) => {
                            ui.label(RichText::new("Game folder was found automatically!").color(WAVEBREAKER_THEME.green));
                        }
                        None => {
                            ui.label(RichText::new("Failed to locate game folder!").color(WAVEBREAKER_THEME.red));
                        }
                    }
                    ui.add_space(8.);
                    ui.add(TextEdit::singleline(&mut self.game_path).text_color(WAVEBREAKER_THEME.text).margin(Margin::same(12)));
                    ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                        if ui
                            .add_sized([64., 32.], Button::new(RichText::new("Install")))
                            .clicked()
                        {
                            self.current_step = InstallationStep::Installing;
                            self.install_task = Some(Self::install(self.game_path.clone()));
                        }
                    });
                }
                InstallationStep::Installing => {
                    let state = self.install_task.get_or_insert_with(|| {Self::install(self.game_path.clone())});
                    let state2 = state.poll_state();
                    match state2 {
                        ImmediateValueState::Updating => {
                            if let Some(status) = state.last_status() {
                                ui.label(RichText::new(status.message.clone()));
                                ui.add(
                                    ProgressBar::new(status.progress.as_f32()).fill(WAVEBREAKER_THEME.sky).desired_height(12.0),
                                );
                                ctx.request_repaint(); // constantly requests UI to be redrawn, so the progress bar updates without user interaction
                            } else {
                                ui.label("Preparing for update...");
                            }
                        }
                        ImmediateValueState::Success(_) => {
                            ui.label(
                                RichText::new("Congratulations, you're all set! You can now launch the game!").color(WAVEBREAKER_THEME.green),
                            );
                            if cfg!(unix) {
                                ui.label("Since you're on Linux or macOS, you may need to set Steam to use Proton 8 for Audiosurf if you run into problems with Wavebreaker.");
                            }
                            ui.add_space(8.);
                            ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                                if ui
                                    .add_sized([64., 32.], Button::new(RichText::new("Launch")))
                                    .clicked()
                                {
                                    ctx.send_viewport_cmd(ViewportCommand::Close);
                                    open::that("steam://rungameid/12900").unwrap();
                                }
                            });
                        }
                        ImmediateValueState::Error(err) => {
                            ui.label(
                                RichText::new(format!("Something went wrong: {}", err.to_string()))
                                    .color(WAVEBREAKER_THEME.red),
                            );
                            ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                                if ui
                                    .add_sized([64., 32.], Button::new(RichText::new("Exit")))
                                    .clicked()
                                {
                                    ctx.send_viewport_cmd(ViewportCommand::Close);
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }
        });
    }
}
