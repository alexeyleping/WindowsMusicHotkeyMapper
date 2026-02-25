use crate::config::{Config, MediaAction};
use crate::media_control::MediaController;
use crate::ui::AppMessage;
use crossbeam_channel::Sender;
use log::{error, info, warn};
use std::sync::{Arc, Mutex};

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS,
};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG, WM_HOTKEY,
};

use super::{hotkey_id_to_key, key_to_hotkey_id, key_to_vk};

pub struct WindowsHotkeyListener {
    config: Arc<Mutex<Config>>,
    media_controller: MediaController,
    app_sender: Sender<AppMessage>,
}

impl WindowsHotkeyListener {
    pub fn new(
        config: Arc<Mutex<Config>>,
        media_controller: MediaController,
        app_sender: Sender<AppMessage>,
    ) -> Self {
        info!("Windows hotkey listener created");
        WindowsHotkeyListener {
            config,
            media_controller,
            app_sender,
        }
    }

    pub fn start(&self) {
        info!("Starting Windows hotkey listener with RegisterHotKey");

        // Register all configured hotkeys
        let registered_ids = self.register_hotkeys();

        if registered_ids.is_empty() {
            error!("No hotkeys were registered successfully");
            return;
        }

        info!("Registered {} hotkeys, starting message loop", registered_ids.len());

        // Message loop
        unsafe {
            let mut msg: MSG = std::mem::zeroed();

            // GetMessageW returns 0 for WM_QUIT, -1 for error, positive for other messages
            loop {
                let result = GetMessageW(&mut msg, HWND::default(), 0, 0);
                if result.0 == 0 {
                    // WM_QUIT received
                    break;
                }
                if result.0 == -1 {
                    error!("GetMessageW returned error");
                    break;
                }

                if msg.message == WM_HOTKEY {
                    let hotkey_id = msg.wParam.0 as i32;
                    self.handle_hotkey(hotkey_id);
                }

                // Required for correct message processing
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            // Unregister hotkeys on exit
            for id in &registered_ids {
                let _ = UnregisterHotKey(HWND::default(), *id);
            }
        }
    }

    fn register_hotkeys(&self) -> Vec<i32> {
        let config = self.config.lock().unwrap();
        let mut registered = Vec::new();

        for (key, _action) in &config.hotkeys {
            if let (Some(id), Some(vk)) = (key_to_hotkey_id(key), key_to_vk(key)) {
                unsafe {
                    // MOD_NOREPEAT (0x4000) prevents repeated firing while key is held
                    let result = RegisterHotKey(
                        HWND::default(),
                        id,
                        HOT_KEY_MODIFIERS(0x4000),
                        vk,
                    );

                    if result.is_ok() {
                        info!("Registered hotkey: {:?} (id={}, vk=0x{:X})", key, id, vk);
                        registered.push(id);
                    } else {
                        warn!(
                            "Failed to register hotkey {:?}: key might be in use by another application",
                            key
                        );
                    }
                }
            }
        }

        registered
    }

    fn handle_hotkey(&self, hotkey_id: i32) {
        if let Some(key) = hotkey_id_to_key(hotkey_id) {
            // Lock only to read the action, then release before executing
            let action = {
                let config = self.config.lock().unwrap();
                config.hotkeys.get(&key).copied()
            };

            if let Some(action) = action {
                info!("Hotkey pressed: {:?} -> {:?}", key, action);

                // Send message to UI
                let _ = self.app_sender.send(AppMessage::HotkeyPressed(key, action));

                // Execute media action (mutex already released)
                match action {
                    MediaAction::PlayPause => self.media_controller.play_pause(),
                    MediaAction::Next => self.media_controller.next(),
                    MediaAction::Previous => self.media_controller.previous(),
                    MediaAction::VolumeUp => self.media_controller.volume_up(),
                    MediaAction::VolumeDown => self.media_controller.volume_down(),
                    MediaAction::Stop => self.media_controller.stop(),
                }
            }
        }
    }
}
