use log::{info, error};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS,
    KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, VIRTUAL_KEY,
};

// Virtual key codes for media control
const VK_MEDIA_NEXT_TRACK: u16 = 0xB0;
const VK_MEDIA_PREV_TRACK: u16 = 0xB1;
const VK_MEDIA_STOP: u16 = 0xB2;
const VK_MEDIA_PLAY_PAUSE: u16 = 0xB3;
const VK_VOLUME_DOWN: u16 = 0xAE;
const VK_VOLUME_UP: u16 = 0xAF;

pub struct WindowsMediaController;

impl WindowsMediaController {
    pub fn new() -> Self {
        info!("Windows media controller initialized");
        WindowsMediaController
    }

    /// Send media key press using SendInput (modern replacement for keybd_event)
    fn send_media_key(&self, key_code: u16) {
        unsafe {
            let inputs = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(key_code),
                            wScan: 0,
                            dwFlags: KEYEVENTF_EXTENDEDKEY,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(key_code),
                            wScan: 0,
                            dwFlags: KEYBD_EVENT_FLAGS(
                                KEYEVENTF_EXTENDEDKEY.0 | KEYEVENTF_KEYUP.0,
                            ),
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ];

            let sent = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
            if sent != inputs.len() as u32 {
                error!("SendInput failed for key 0x{:X}: sent {}/{}", key_code, sent, inputs.len());
            }
        }

        info!("Sent media key via SendInput: 0x{:X}", key_code);
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
        info!("Windows: Sending Stop");
        self.send_media_key(VK_MEDIA_STOP);
    }
}
