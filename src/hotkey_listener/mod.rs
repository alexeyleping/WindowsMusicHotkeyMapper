#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

use crate::config::{Config, MediaAction};
use crate::media_control::MediaController;
use crate::ui::AppMessage;
use crossbeam_channel::Sender;
use log::info;
use rdev::Key;
use std::sync::{Arc, Mutex};

pub struct HotkeyListener {
    #[cfg(target_os = "windows")]
    inner: windows::WindowsHotkeyListener,

    #[cfg(target_os = "linux")]
    inner: linux::LinuxHotkeyListener,
}

impl HotkeyListener {
    pub fn new(
        config: Arc<Mutex<Config>>,
        media_controller: MediaController,
        app_sender: Sender<AppMessage>,
    ) -> Self {
        info!("Creating hotkey listener");

        #[cfg(target_os = "windows")]
        let inner = windows::WindowsHotkeyListener::new(config, media_controller, app_sender);

        #[cfg(target_os = "linux")]
        let inner = linux::LinuxHotkeyListener::new(config, media_controller, app_sender);

        HotkeyListener { inner }
    }

    /// Start listening for hotkeys (blocking call)
    pub fn start(&self) {
        self.inner.start();
    }
}

/// Convert rdev::Key to a unique hotkey ID for Windows RegisterHotKey
pub fn key_to_hotkey_id(key: &Key) -> Option<i32> {
    match key {
        Key::F1 => Some(1),
        Key::F2 => Some(2),
        Key::F3 => Some(3),
        Key::F4 => Some(4),
        Key::F5 => Some(5),
        Key::F6 => Some(6),
        Key::F7 => Some(7),
        Key::F8 => Some(8),
        Key::F9 => Some(9),
        Key::F10 => Some(10),
        Key::F11 => Some(11),
        Key::F12 => Some(12),
        _ => None,
    }
}

/// Convert rdev::Key to Windows virtual key code
#[cfg(target_os = "windows")]
pub fn key_to_vk(key: &Key) -> Option<u32> {
    match key {
        Key::F1 => Some(0x70),  // VK_F1
        Key::F2 => Some(0x71),
        Key::F3 => Some(0x72),
        Key::F4 => Some(0x73),
        Key::F5 => Some(0x74),
        Key::F6 => Some(0x75),
        Key::F7 => Some(0x76),
        Key::F8 => Some(0x77),
        Key::F9 => Some(0x78),
        Key::F10 => Some(0x79),
        Key::F11 => Some(0x7A),
        Key::F12 => Some(0x7B),
        _ => None,
    }
}

/// Convert hotkey ID back to rdev::Key
pub fn hotkey_id_to_key(id: i32) -> Option<Key> {
    match id {
        1 => Some(Key::F1),
        2 => Some(Key::F2),
        3 => Some(Key::F3),
        4 => Some(Key::F4),
        5 => Some(Key::F5),
        6 => Some(Key::F6),
        7 => Some(Key::F7),
        8 => Some(Key::F8),
        9 => Some(Key::F9),
        10 => Some(Key::F10),
        11 => Some(Key::F11),
        12 => Some(Key::F12),
        _ => None,
    }
}
