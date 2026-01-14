use crossbeam_channel::{unbounded, Receiver, Sender};
use log::{error, info};
use std::sync::{Arc, Mutex};
use std::thread;

mod config;
mod hotkey_listener;
mod media_control;
mod ui;

use config::Config;
use hotkey_listener::HotkeyListener;
use media_control::MediaController;
use ui::{AppMessage, UiMessage};

fn main() {
    // Initialize logging
    env_logger::init();

    info!("Starting Music HotKey Mapper");

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            info!("Creating default configuration");
            let cfg = Config::default();
            if let Err(e) = cfg.save() {
                error!("Failed to save default configuration: {}", e);
            }
            cfg
        }
    };

    // Create channels for communication between UI and main thread
    let (ui_sender, ui_receiver): (Sender<UiMessage>, Receiver<UiMessage>) = unbounded();
    let (app_sender, app_receiver): (Sender<AppMessage>, Receiver<AppMessage>) = unbounded();

    // Clone configuration for different threads
    let config_for_ui = config.clone();
    let config_shared = Arc::new(Mutex::new(config));

    // Start thread for keyboard event processing
    let config_for_listener = config_shared.clone();
    let app_sender_for_listener = app_sender.clone();

    thread::spawn(move || {
        info!("Starting keyboard listener thread");

        // Create media controller
        let media_controller = MediaController::new();

        // Create and start hotkey listener (platform-specific implementation)
        let listener = HotkeyListener::new(
            config_for_listener,
            media_controller,
            app_sender_for_listener,
        );
        listener.start();
    });

    // Start thread for processing messages from UI
    let config_for_messages = config_shared.clone();
    thread::spawn(move || {
        info!("Starting UI message handler thread");

        loop {
            match ui_receiver.recv() {
                Ok(UiMessage::SaveConfig(new_config)) => {
                    info!("Received new configuration from UI");
                    let mut config = config_for_messages.lock().unwrap();
                    *config = new_config;
                    info!("Configuration updated in listener thread");
                }
                Ok(UiMessage::Exit) => {
                    info!("Received exit message from UI");
                    std::process::exit(0);
                }
                Err(e) => {
                    error!("Error receiving UI message: {}", e);
                    break;
                }
            }
        }
    });

    // Start UI in main thread
    info!("Starting UI");
    if let Err(e) = ui::run_ui(config_for_ui, ui_sender, app_receiver) {
        error!("Failed to run UI: {}", e);
        std::process::exit(1);
    }
}
