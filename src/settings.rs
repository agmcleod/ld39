use loader::get_exe_path;
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub music_volume: f32,
    #[serde(default)]
    pub sound_volume: f32,
    #[serde(default)]
    pub mute_music: bool,
    #[serde(default)]
    pub mute_sound_effects: bool,
    #[serde(default)]
    pub completed_tutorial: bool,
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

    pub fn set_mute_music(&mut self, mute: bool) {
        self.mute_music = mute;
        self.save();
    }

    pub fn set_mute_sound_effects(&mut self, mute: bool) {
        self.mute_sound_effects = mute;
        self.save();
    }

    pub fn set_completed_tutorial(&mut self, completed: bool) {
        self.completed_tutorial = completed;
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
            mute_music: false,
            mute_sound_effects: false,
            completed_tutorial: false,
        }
    }
}
