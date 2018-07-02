use loader::get_exe_path;
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub music_volume: f32,
    pub sound_volume: f32,
}

impl Settings {
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume;
        self.save();
    }

    pub fn set_sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume;
        self.save();
    }

    fn save(&self) {
        let text = serde_json::to_string(&self).unwrap();
        fs::write(get_exe_path().join("settings.json").to_str().unwrap(), text)
            .expect("Unable to write settings");
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            music_volume: 0.5,
            sound_volume: 0.5,
        }
    }
}
