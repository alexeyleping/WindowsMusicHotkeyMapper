# Windows Music HotKey Mapper

An application for binding keyboard keys to control a media player on Windows and Linux.

## Description

This application allows you to use any keys on your keyboard to control the media player (play, pause, next/previous track, volume). It is useful if your keyboard does not have special media keys.

## Features

- ✅ **Graphical interface** - convenient configuration of hotkeys via the UI
- ✅ Global hotkeys (work even when the app is in the background)
- ✅ Windows and Linux support
- ✅ Customizable key bindings
- ✅ Playback control (Play/Pause, Next, Previous, Stop)
- ✅ Volume control (Volume Up/Down)
- ✅ Visual indication of hotkey taps

## Installation

### 1. Installing Rust

If Rust is not already installed:

**Linux/Ubuntu:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and install from [rustup.rs ](https://rustup.rs /)

### 2. Installing dependencies

**Linux/Ubuntu:**
```bash
# The app needs playerctl
sudo apt install playerctl to work

# You may also need libraries for rdev
sudo apt install libx11-dev libxdo-dev
```

**Windows:**
No additional dependencies are required.

### 3. Building the project

```bash
cd WindowsMusicHotKeyMapper
cargo build --release
```

## Usage

### Launching the app

```bash
# Detailed logs mode
RUST_LOG=info cargo run --release

# Or run the compiled binary
./target/release/windows_music_hotkey_mapper
``

At startup, a graphical window with the settings will open.:

- **View hotkeys** - all configured keys and their actions
- **Change actions** - select from the drop-down list
- **Adding hotkeys** - the "➕ Add hotkey" button
- **Deleting hotkeys** - the "❌" button next to each hotkey
- **Save** - the "💾 Save" button applies the changes
- **Indication** - when pressing the hotkey, an action is displayed in the UI

### Configuration

The first time you run it, a configuration file will be created:
- **Linux:** `~/.config/music_hotkey_mapper/config.json`
- **Windows:** `%APPDATA%\music_hotkey_mapper\config.json`

Example of the default configuration:

```json
{
  "hotkeys": {
    "F7": "Previous",
    "F8": "PlayPause",
    "F9": "Next",
    "F10": "Stop",
    "F11": "VolumeDown",
    "F12": "VolumeUp"
  }
}
```

### Available actions

- `PlayPause` - Playback/Pause
- `Next` - Next track
- `Previous` - Previous track
- `Stop` - Stop playback
- `VolumeUp` - Increase the volume
- `VolumeDown` - Turn down the volume

### Available keys

The following keys can be used in the configuration:
- Functional: `F1` - `F12`
- You can extend the support for other keys by editing `src/config.rs`

## How it works

### Windows
The application uses the Windows API (`keybd_event`) to send virtual media keys to the system. These commands are processed by the active media player (Spotify, VLC, browser, etc.).

### Linux
The application uses the playerctl utility to control MPRIS-compatible players (Spotify, VLC, Rhythmbox, browsers, etc.).

## Project structure

```
WindowsMusicHotKeyMapper/
├── src/
├─── main.rs # Entry point, keyboard event handling
│   ├── config.rs # Configuration and serialization
│   └── media_control/
├─── mod.rs # Common MediaController interface
│ ├── windows.rs # Windows Implementation (WinAPI)
│ └── linux.rs # Implementation for Linux (playerctl)
├── Cargo.toml # Project dependencies
├─── README.md # This file
```

## Development

### Adding new keys

1. Edit `src/config.rs', the `parse_key()' function`
2. Add a new key option
3. Rebuild the project

### Adding new actions

1. Add a new option to the `MediaAction` enum in `src/config.rs`
2. Implement the method in `src/media_control/windows.rs`
3. Implement the method in `src/media_control/linux.rs`
4. Add the processing to `src/main.rs`

## Known issues

- Linux requires the installed `playerctl`
- The application must have permissions to read global keyboard events
- On some Linux distributions, you may need to run with administrator rights.

## License

Educational project, free use.

## Author

Created as the first learning project in Rust to study:
- Working with the system API
- Cross-platform development
- Keyboard event handling
- Modular architecture