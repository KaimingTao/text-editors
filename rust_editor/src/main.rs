use eframe::egui;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Simple Text Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Text Editor",
        options,
        Box::new(|_cc| Ok(Box::new(TextEditor::default()))),
    )
}

#[derive(Default)]
struct TextEditor {
    content: String,
    filepath: Option<PathBuf>,
    status_message: String,
}

impl TextEditor {
    fn new_file(&mut self) {
        self.content.clear();
        self.filepath = None;
        self.status_message = "New file created".to_string();
    }

    fn open_file(&mut self, path: PathBuf) {
        match fs::read_to_string(&path) {
            Ok(content) => {
                self.content = content;
                self.filepath = Some(path.clone());
                self.status_message = format!("Opened: {}", path.display());
            }
            Err(e) => {
                self.status_message = format!("Error opening file: {}", e);
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(path) = &self.filepath {
            match fs::write(path, &self.content) {
                Ok(_) => {
                    self.status_message = format!("Saved: {}", path.display());
                }
                Err(e) => {
                    self.status_message = format!("Error saving: {}", e);
                }
            }
        } else {
            self.status_message = "No file path set. Use Save As.".to_string();
        }
    }

    fn save_as(&mut self, path: PathBuf) {
        match fs::write(&path, &self.content) {
            Ok(_) => {
                self.filepath = Some(path.clone());
                self.status_message = format!("Saved as: {}", path.display());
            }
            Err(e) => {
                self.status_message = format!("Error saving: {}", e);
            }
        }
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.new_file();
                        ui.close_menu();
                    }

                    if ui.button("Open...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.open_file(path);
                        }
                        ui.close_menu();
                    }

                    if ui.button("Save").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }

                    if ui.button("Save As...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.save_as(path);
                        }
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Clear").clicked() {
                        self.content.clear();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    ui.label("Simple Text Editor v1.0");
                    ui.label("Built with egui");
                });
            });
        });

        // Status bar at bottom
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(path) = &self.filepath {
                        ui.label(format!("File: {}", path.display()));
                    } else {
                        ui.label("Untitled");
                    }
                    ui.separator();
                    ui.label(format!("Lines: {}", self.content.lines().count()));
                    ui.label(format!("Chars: {}", self.content.len()));
                });
            });
        });

        // Main text editor area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let text_edit = egui::TextEdit::multiline(&mut self.content)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .lock_focus(true);

                ui.add(text_edit);
            });
        });

        // Keyboard shortcuts
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::N)) {
            self.new_file();
        }

        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::O)) {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.open_file(path);
            }
        }

        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S)) {
            if ctx.input(|i| i.modifiers.shift) {
                // Save As (Cmd/Ctrl + Shift + S)
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    self.save_as(path);
                }
            } else {
                // Save (Cmd/Ctrl + S)
                self.save_file();
            }
        }
    }
}
