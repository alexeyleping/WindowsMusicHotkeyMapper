use log::{info, error};
use std::process::Command;

pub struct LinuxMediaController;

impl LinuxMediaController {
    pub fn new() -> Self {
        info!("Linux media controller initialized");
        info!("Note: This requires 'playerctl' to be installed on your system");
        info!("Install it with: sudo apt install playerctl");
        LinuxMediaController
    }

    /// Execute playerctl command
    fn execute_playerctl(&self, command: &str) {
        // Use -a flag to send command to all available players
        match Command::new("playerctl")
            .arg("-a")
            .arg(command)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    info!("Successfully executed: playerctl -a {}", command);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    // Ignore "No player could handle this command" error
                    // as it's normal when the player is stopped
                    if !stderr.contains("No player could handle this command") {
                        error!("playerctl command failed: {}", stderr);
                    } else {
                        info!("No active player to handle: {}", command);
                    }
                }
            }
            Err(e) => {
                error!("Failed to execute playerctl: {}", e);
                error!("Make sure playerctl is installed: sudo apt install playerctl");
            }
        }
    }

    pub fn play_pause(&self) {
        info!("Linux: Sending Play/Pause");
        self.execute_playerctl("play-pause");
    }

    pub fn next(&self) {
        info!("Linux: Sending Next Track");
        self.execute_playerctl("next");
    }

    pub fn previous(&self) {
        info!("Linux: Sending Previous Track");
        self.execute_playerctl("previous");
    }

    pub fn volume_up(&self) {
        info!("Linux: Sending Volume Up");
        // Increase volume by 5%
        self.execute_playerctl("volume");
        match Command::new("playerctl")
            .args(&["volume", "0.05+"])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    info!("Successfully increased volume");
                } else {
                    error!(
                        "Volume up failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                error!("Failed to execute volume up: {}", e);
            }
        }
    }

    pub fn volume_down(&self) {
        info!("Linux: Sending Volume Down");
        // Decrease volume by 5%
        match Command::new("playerctl")
            .args(&["volume", "0.05-"])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    info!("Successfully decreased volume");
                } else {
                    error!(
                        "Volume down failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                error!("Failed to execute volume down: {}", e);
            }
        }
    }

    pub fn stop(&self) {
        info!("Linux: Sending Stop (using pause instead)");
        // Use pause instead of stop, as many players
        // cannot resume playback after stop
        self.execute_playerctl("pause");
    }
}
