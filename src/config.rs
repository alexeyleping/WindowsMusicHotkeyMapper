use rdev::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Media control actions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MediaAction {
    PlayPause,
    Next,
    Previous,
    VolumeUp,
    VolumeDown,
    Stop,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "key_map_serde")]
    pub hotkeys: HashMap<Key, MediaAction>,
}

impl Default for Config {
    fn default() -> Self {
        let mut hotkeys = HashMap::new();

        // Default hotkey bindings
        // F7 - Previous track
        hotkeys.insert(Key::F7, MediaAction::Previous);
        // F8 - Play/Pause
        hotkeys.insert(Key::F8, MediaAction::PlayPause);
        // F9 - Next track
        hotkeys.insert(Key::F9, MediaAction::Next);
        // F10 - Stop
        hotkeys.insert(Key::F10, MediaAction::Stop);
        // F11 - Volume Down
        hotkeys.insert(Key::F11, MediaAction::VolumeDown);
        // F12 - Volume Up
        hotkeys.insert(Key::F12, MediaAction::VolumeUp);

        Config { hotkeys }
    }
}

impl Config {
    /// Get the path to the configuration file
    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("music_hotkey_mapper");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    /// Load configuration from file
    pub fn load() -> Result<Self, io::Error> {
        let path = Self::config_path();
        let contents = fs::read_to_string(path)?;
        serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), io::Error> {
        let path = Self::config_path();
        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, contents)?;
        Ok(())
    }
}

// Module for serialization of HashMap<Key, MediaAction>
mod key_map_serde {
    use super::*;
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::Serializer;
    use std::collections::HashMap;

    pub fn serialize<S>(
        map: &HashMap<Key, MediaAction>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let mut s = serializer.serialize_map(Some(map.len()))?;
        for (key, value) in map {
            let key_str = format!("{:?}", key);
            s.serialize_entry(&key_str, value)?;
        }
        s.end()
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<Key, MediaAction>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: HashMap<String, MediaAction> = HashMap::deserialize(deserializer)?;
        let mut result = HashMap::new();

        for (key_str, action) in map {
            let key = parse_key(&key_str).ok_or_else(|| {
                serde::de::Error::custom(format!("Invalid key: {}", key_str))
            })?;
            result.insert(key, action);
        }

        Ok(result)
    }

    fn parse_key(s: &str) -> Option<Key> {
        // Simple function to parse string to Key
        // Can be extended in a real project
        match s {
            "F1" => Some(Key::F1),
            "F2" => Some(Key::F2),
            "F3" => Some(Key::F3),
            "F4" => Some(Key::F4),
            "F5" => Some(Key::F5),
            "F6" => Some(Key::F6),
            "F7" => Some(Key::F7),
            "F8" => Some(Key::F8),
            "F9" => Some(Key::F9),
            "F10" => Some(Key::F10),
            "F11" => Some(Key::F11),
            "F12" => Some(Key::F12),
            _ => None,
        }
    }
}
