use crate::config::{Config, MediaAction};
use crossbeam_channel::{Receiver, Sender};
use eframe::egui;
use log::info;
use rdev::Key;
use std::collections::HashMap;

/// Messages from UI to main thread
#[derive(Debug, Clone)]
pub enum UiMessage {
    SaveConfig(Config),
    Exit,
}

/// Messages from main thread to UI
#[derive(Debug, Clone)]
pub enum AppMessage {
    HotkeyPressed(Key, MediaAction),
    ConfigUpdated(Config),
}

/// Main application with UI
pub struct HotkeyMapperApp {
    config: Config,
    editing_key: Option<Key>,
    listening_for_key: bool,
    last_pressed_info: Option<(Key, MediaAction, f64)>,
    ui_sender: Sender<UiMessage>,
    app_receiver: Receiver<AppMessage>,
}

impl HotkeyMapperApp {
    pub fn new(
        config: Config,
        ui_sender: Sender<UiMessage>,
        app_receiver: Receiver<AppMessage>,
    ) -> Self {
        Self {
            config,
            editing_key: None,
            listening_for_key: false,
            last_pressed_info: None,
            ui_sender,
            app_receiver,
        }
    }

    fn process_app_messages(&mut self, ctx: &egui::Context) {
        while let Ok(msg) = self.app_receiver.try_recv() {
            match msg {
                AppMessage::HotkeyPressed(key, action) => {
                    info!("UI received hotkey press: {:?} -> {:?}", key, action);
                    self.last_pressed_info = Some((key, action, ctx.input(|i| i.time)));
                    ctx.request_repaint();
                }
                AppMessage::ConfigUpdated(config) => {
                    self.config = config;
                    ctx.request_repaint();
                }
            }
        }
    }
}

impl eframe::App for HotkeyMapperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process messages from main thread
        self.process_app_messages(ctx);

        // Main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Music Hotkey Mapper");
            ui.add_space(10.0);

            // Status
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.colored_label(egui::Color32::GREEN, "● Active");
            });

            ui.add_space(5.0);

            // Show last key press
            if let Some((key, action, time)) = self.last_pressed_info {
                let current_time = ui.input(|i| i.time);
                if current_time - time < 3.0 {
                    ui.horizontal(|ui| {
                        ui.label("Last action:");
                        ui.colored_label(
                            egui::Color32::from_rgb(100, 200, 100),
                            format!("{:?} → {:?}", key, action),
                        );
                    });
                }
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Hotkey settings
            ui.heading("Hotkey Settings");
            ui.add_space(10.0);

            // Hotkeys table
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("hotkeys_grid")
                    .num_columns(3)
                    .spacing([10.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        // Headers
                        ui.label(egui::RichText::new("Key").strong());
                        ui.label(egui::RichText::new("Action").strong());
                        ui.label(egui::RichText::new("Control").strong());
                        ui.end_row();

                        // List of hotkeys
                        let mut keys_to_remove = Vec::new();
                        let mut keys_to_update = Vec::new();
                        let mut hotkeys: Vec<_> = self.config.hotkeys.iter().collect();
                        hotkeys.sort_by_key(|(k, _)| format!("{:?}", k));

                        for (key, action) in hotkeys {
                            ui.label(format!("{:?}", key));

                            // Action selection
                            let mut current_action = *action;
                            egui::ComboBox::from_id_salt(format!("action_{:?}", key))
                                .selected_text(format!("{:?}", current_action))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut current_action, MediaAction::PlayPause, "Play/Pause");
                                    ui.selectable_value(&mut current_action, MediaAction::Next, "Next");
                                    ui.selectable_value(&mut current_action, MediaAction::Previous, "Previous");
                                    ui.selectable_value(&mut current_action, MediaAction::Stop, "Stop");
                                    ui.selectable_value(&mut current_action, MediaAction::VolumeUp, "Volume Up");
                                    ui.selectable_value(&mut current_action, MediaAction::VolumeDown, "Volume Down");
                                });

                            // If action changed, remember for update
                            if current_action != *action {
                                keys_to_update.push((*key, current_action));
                            }

                            // Delete button
                            if ui.button("❌").clicked() {
                                keys_to_remove.push(*key);
                            }

                            ui.end_row();
                        }

                        // Apply changes
                        for (key, action) in keys_to_update {
                            self.config.hotkeys.insert(key, action);
                        }

                        // Remove hotkeys
                        for key in keys_to_remove {
                            self.config.hotkeys.remove(&key);
                        }
                    });
            });

            ui.add_space(10.0);

            // Add new hotkey button
            if ui.button("➕ Add Hotkey").clicked() {
                info!("Add hotkey button clicked");
                // Could add dialog for adding new hotkey
                // For now just add an example
                if !self.config.hotkeys.contains_key(&Key::F1) {
                    self.config.hotkeys.insert(Key::F1, MediaAction::PlayPause);
                }
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Control buttons
            ui.horizontal(|ui| {
                if ui.button("💾 Save").clicked() {
                    info!("Save button clicked");
                    if let Err(e) = self.config.save() {
                        log::error!("Failed to save config: {}", e);
                    } else {
                        info!("Configuration saved successfully");
                        // Send updated configuration to main thread
                        let _ = self.ui_sender.send(UiMessage::SaveConfig(self.config.clone()));
                    }
                }

                if ui.button("🔄 Reload").clicked() {
                    info!("Reload button clicked");
                    match Config::load() {
                        Ok(config) => {
                            self.config = config;
                            info!("Configuration reloaded");
                        }
                        Err(e) => {
                            log::error!("Failed to reload config: {}", e);
                        }
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❌ Exit").clicked() {
                        info!("Exit button clicked");
                        let _ = self.ui_sender.send(UiMessage::Exit);
                        std::process::exit(0);
                    }
                });
            });

            ui.add_space(10.0);

            // Instructions
            ui.group(|ui| {
                ui.label(egui::RichText::new("ℹ Instructions:").strong());
                ui.label("• Configure the desired keys and actions");
                ui.label("• Click 'Save' to apply changes");
                ui.label("• The application works in the background and intercepts key presses");
            });
        });
    }
}

/// Start UI
pub fn run_ui(
    config: Config,
    ui_sender: Sender<UiMessage>,
    app_receiver: Receiver<AppMessage>,
) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([500.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Music Hotkey Mapper",
        options,
        Box::new(|_cc| Ok(Box::new(HotkeyMapperApp::new(config, ui_sender, app_receiver)))),
    )
}
