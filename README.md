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

### Option 1: Download Pre-built Binaries (Recommended)

Download the latest release from the [Releases page](https://github.com/yourusername/WindowsMusicHotkeyMapper/releases):

**Windows:**
- Download `windows_music_hotkey_mapper-windows-x64.zip`
- Extract the ZIP archive
- Run `windows_music_hotkey_mapper.exe`
- No installation required - fully portable!

**Linux:**
- Download `windows_music_hotkey_mapper-linux-x64.tar.gz`
- Extract: `tar -xzf windows_music_hotkey_mapper-linux-x64.tar.gz`
- Make executable: `chmod +x windows_music_hotkey_mapper`
- Install playerctl: `sudo apt install playerctl`
- Run: `./windows_music_hotkey_mapper`

### Option 2: Build from Source

#### 1. Installing Rust

If Rust is not already installed:

**Linux/Ubuntu:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and install from [rustup.rs](https://rustup.rs/)

#### 2. Installing dependencies

**Linux/Ubuntu:**
```bash
# The app needs playerctl
sudo apt install playerctl to work

# You may also need libraries for rdev
sudo apt install libx11-dev libxdo-dev
```

**Windows:**
No additional dependencies are required.

#### 3. Building the project

```bash
cd WindowsMusicHotKeyMapper
cargo build --release
```

#### 4. Building Windows MSI Installer (Optional)

If you want to create a Windows MSI installer:

**Prerequisites:**
- Install [WiX Toolset v5.0+](https://wixtoolset.org/)

**Build MSI:**
```bash
# First, build the release binary
cargo build --release --target x86_64-pc-windows-msvc

# Then build the MSI using WiX
wix build -arch x64 -out target\wix\windows_music_hotkey_mapper.msi wix\main.wxs
```

The installer will be created in `target/wix/windows_music_hotkey_mapper.msi`

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

## CI/CD and Releases

This project uses GitHub Actions for automated building and releases:

### Automatic Builds

Every push to the repository triggers:
- Windows build (x86_64-pc-windows-msvc) → ZIP archive
- Linux build (x86_64-unknown-linux-gnu) → tarball archive
- Fully portable, no installation required

### Creating a Release

To create a new release:

```bash
# Tag the commit
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions will automatically:
1. Build optimized binaries for Windows and Linux
2. Package them with README and LICENSE
3. Create ZIP (Windows) and tar.gz (Linux) archives
4. Upload artifacts to the release page

MSI installer can be built locally if needed (see Building section above).

### Runtime Dependencies

**Windows:**
- No additional runtime dependencies required
- All libraries are statically linked
- Works on Windows 10/11 out of the box

**Linux:**
- Requires `playerctl` for media control: `sudo apt install playerctl`
- Requires X11 libraries (usually pre-installed)
- Requires OpenGL support (usually pre-installed)

## Known issues

- Linux requires the installed `playerctl`
- The application must have permissions to read global keyboard events
- On some Linux distributions, you may need to run with administrator rights

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Created as the first learning project in Rust to study:
- Working with the system API
- Cross-platform development
- Keyboard event handling
- Modular architecture