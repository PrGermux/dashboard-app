#![windows_subsystem = "windows"]

use eframe::{egui, epaint::TextureHandle, App, IconData, NativeOptions};
use egui::TextureOptions;
use std::process::Command;

struct DashboardApp {
    textures: Vec<(TextureHandle, String)>,
}

impl DashboardApp {
    fn new(ctx: &egui::Context) -> Self {
        // Load all textures once and store them
        let textures = BUTTONS
            .iter()
            .map(|&(_, name, icon_path)| {
                let texture_handle = load_image(ctx, icon_path, [64, 64])
                    .unwrap_or_else(|| ctx.load_texture("failed", egui::ColorImage::example(), Default::default()));
                (texture_handle, name.to_string())
            })
            .collect();

        DashboardApp { textures }
    }
}

impl App for DashboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::Grid::new("dashboard_grid")
                    .spacing([40.0, 20.0])
                    .show(ui, |ui| {
                        for (index, &(ref texture, ref name)) in self.textures.iter().enumerate() {
                            if index > 0 && index % 3 == 0 {
                                ui.end_row();
                            }

                            ui.vertical_centered(|ui| {
                                // Icon as a clickable button with hover effect
                                let image_button = ui.add(egui::ImageButton::new(texture));
                                if image_button.clicked() {
                                    run_exe(BUTTONS[index].0);
                                }
                                image_button.on_hover_cursor(egui::CursorIcon::PointingHand);

                                // Name below the icon with centered text
                                let text_button = ui.button(egui::RichText::new(name).strong());
                                if text_button.clicked() {
                                    run_exe(BUTTONS[index].0);
                                }
                                text_button.on_hover_cursor(egui::CursorIcon::PointingHand);
                            });
                        }
                    });
            });
        });
    }
}

fn main() {
    let icon_data = include_bytes!("../res/icon.ico");
    
    // Load the .ico file as the application icon
    let icon = image::load_from_memory(icon_data).unwrap().to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let icon_rgba = icon.into_raw();

    let icon_data = IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    };

    let native_options = NativeOptions {
        initial_window_size: Some(egui::vec2(312.0, 400.0)),
        resizable: false,
        fullscreen: false,
        icon_data: Some(icon_data), // Set the application icon here
        ..Default::default()
    };

    eframe::run_native("Dashboard App", native_options, Box::new(|cc| {
        Box::new(DashboardApp::new(&cc.egui_ctx))
    }))
    .unwrap();
}

// Function to load the PNG images
fn load_image(ctx: &egui::Context, path: &str, size: [usize; 2]) -> Option<TextureHandle> {
    use eframe::epaint::ColorImage;

    let img = image::open(path).ok()?.resize_exact(size[0] as u32, size[1] as u32, image::imageops::FilterType::Lanczos3);
    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.to_rgba8().into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(path, color_image, TextureOptions::default()))
}

// Function to run the .exe file
fn run_exe(path: &str) {
    if let Err(e) = Command::new(path).spawn() {
        eprintln!("Failed to start {}: {}", path, e);
    }
}

// Define the buttons with the paths to the executables, their names, and the paths to their icons
const BUTTONS: [(&str, &str, &str); 7] = [
    ("PATH TO EXE FILE", "NAME", "PATH TO ICON"),
    ("PATH TO EXE FILE", "NAME", "PATH TO ICON"),
];
