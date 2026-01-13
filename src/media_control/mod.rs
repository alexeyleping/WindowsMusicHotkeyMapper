#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

use log::info;

/// Controller for media player management
pub struct MediaController {
    #[cfg(target_os = "windows")]
    inner: windows::WindowsMediaController,

    #[cfg(target_os = "linux")]
    inner: linux::LinuxMediaController,
}

impl MediaController {
    pub fn new() -> Self {
        info!("Initializing MediaController for {}", std::env::consts::OS);

        #[cfg(target_os = "windows")]
        {
            MediaController {
                inner: windows::WindowsMediaController::new(),
            }
        }

        #[cfg(target_os = "linux")]
        {
            MediaController {
                inner: linux::LinuxMediaController::new(),
            }
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            panic!("Unsupported operating system");
        }
    }

    pub fn play_pause(&self) {
        info!("MediaController: Play/Pause");
        self.inner.play_pause();
    }

    pub fn next(&self) {
        info!("MediaController: Next");
        self.inner.next();
    }

    pub fn previous(&self) {
        info!("MediaController: Previous");
        self.inner.previous();
    }

    pub fn volume_up(&self) {
        info!("MediaController: Volume Up");
        self.inner.volume_up();
    }

    pub fn volume_down(&self) {
        info!("MediaController: Volume Down");
        self.inner.volume_down();
    }

    pub fn stop(&self) {
        info!("MediaController: Stop");
        self.inner.stop();
    }
}

impl Default for MediaController {
    fn default() -> Self {
        Self::new()
    }
}
