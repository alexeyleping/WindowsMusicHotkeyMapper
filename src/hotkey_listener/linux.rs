use crate::config::{Config, MediaAction};
use crate::media_control::MediaController;
use crate::ui::AppMessage;
use crossbeam_channel::Sender;
use log::{error, info};
use rdev::{listen, Event, EventType};
use std::sync::{Arc, Mutex};

pub struct LinuxHotkeyListener {
    config: Arc<Mutex<Config>>,
    media_controller: MediaController,
    app_sender: Sender<AppMessage>,
}

impl LinuxHotkeyListener {
    pub fn new(
        config: Arc<Mutex<Config>>,
        media_controller: MediaController,
        app_sender: Sender<AppMessage>,
    ) -> Self {
        info!("Linux hotkey listener created (using rdev)");
        LinuxHotkeyListener {
            config,
            media_controller,
            app_sender,
        }
    }

    pub fn start(&self) {
        info!("Starting Linux hotkey listener with rdev");

        // Clone Arc references for the closure
        let config = self.config.clone();
        let app_sender = self.app_sender.clone();

        // We need to create a new MediaController because rdev::listen takes ownership
        // of the closure and MediaController is not Clone
        let media_controller = MediaController::new();

        if let Err(error) = listen(move |event: Event| {
            Self::handle_event(&event, &config, &media_controller, &app_sender);
        }) {
            error!("Error listening to keyboard events: {:?}", error);
        }
    }

    fn handle_event(
        event: &Event,
        config: &Arc<Mutex<Config>>,
        media_controller: &MediaController,
        app_sender: &Sender<AppMessage>,
    ) {
        if let EventType::KeyPress(key) = event.event_type {
            let config = config.lock().unwrap();

            if let Some(action) = config.hotkeys.get(&key) {
                info!("Hotkey pressed: {:?} -> {:?}", key, action);

                // Send message to UI
                let _ = app_sender.send(AppMessage::HotkeyPressed(key, *action));

                // Execute action
                match action {
                    MediaAction::PlayPause => media_controller.play_pause(),
                    MediaAction::Next => media_controller.next(),
                    MediaAction::Previous => media_controller.previous(),
                    MediaAction::VolumeUp => media_controller.volume_up(),
                    MediaAction::VolumeDown => media_controller.volume_down(),
                    MediaAction::Stop => media_controller.stop(),
                }
            }
        }
    }
}
