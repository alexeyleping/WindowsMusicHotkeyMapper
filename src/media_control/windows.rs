use log::{info, error};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, KEYBD_EVENT_FLAGS, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP,
};

// Virtual key codes for media control
const VK_MEDIA_NEXT_TRACK: u8 = 0xB0;
const VK_MEDIA_PREV_TRACK: u8 = 0xB1;
const VK_MEDIA_STOP: u8 = 0xB2;
const VK_MEDIA_PLAY_PAUSE: u8 = 0xB3;
const VK_VOLUME_MUTE: u8 = 0xAD;
const VK_VOLUME_DOWN: u8 = 0xAE;
const VK_VOLUME_UP: u8 = 0xAF;

pub struct WindowsMediaController;

impl WindowsMediaController {
    pub fn new() -> Self {
        info!("Windows media controller initialized");
        WindowsMediaController
    }

    /// Send media key press
    fn send_media_key(&self, key_code: u8) {
        unsafe {
            // Key press
            keybd_event(
                key_code,
                0,
                KEYBD_EVENT_FLAGS(KEYEVENTF_EXTENDEDKEY.0),
                0,
            );

            // Key release
            keybd_event(
                key_code,
                0,
                KEYBD_EVENT_FLAGS(KEYEVENTF_EXTENDEDKEY.0 | KEYEVENTF_KEYUP.0),
                0,
            );
        }

        info!("Sent media key: 0x{:X}", key_code);
    }

    pub fn play_pause(&self) {
        info!("Windows: Sending Play/Pause");
        self.send_media_key(VK_MEDIA_PLAY_PAUSE);
    }

    pub fn next(&self) {
        info!("Windows: Sending Next Track");
        self.send_media_key(VK_MEDIA_NEXT_TRACK);
    }

    pub fn previous(&self) {
        info!("Windows: Sending Previous Track");
        self.send_media_key(VK_MEDIA_PREV_TRACK);
    }

    pub fn volume_up(&self) {
        info!("Windows: Sending Volume Up");
        self.send_media_key(VK_VOLUME_UP);
    }

    pub fn volume_down(&self) {
        info!("Windows: Sending Volume Down");
        self.send_media_key(VK_VOLUME_DOWN);
    }

    pub fn stop(&self) {
        info!("Windows: Sending Stop (using Play/Pause to pause)");
        // Use Play/Pause instead of Stop for better compatibility
        // Many players (especially browser-based) don't respond to Play after Stop
        self.send_media_key(VK_MEDIA_PLAY_PAUSE);
    }
}
