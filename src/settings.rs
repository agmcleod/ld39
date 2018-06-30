#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub music_volume: f32,
    pub sound_volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            music_volume: 0.5,
            sound_volume: 0.5,
        }
    }
}
