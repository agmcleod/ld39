use std::collections::HashMap;

use rodio::{Endpoint, Sink};

use loader;

pub struct MusicManager {
    tracks: HashMap<String, Sink>,
    current_track: String,
}

impl MusicManager {
    pub fn new(audio_endpoint: &Endpoint, volume: f32) -> Self {
        let mut tracks = HashMap::new();

        let zen = loader::create_music_sink("resources/zen.ogg", &audio_endpoint, volume);
        let meloncholy = loader::create_music_sink("resources/meloncholy.ogg", &audio_endpoint, volume);
        let title = loader::create_music_sink("resources/ld39.ogg", &audio_endpoint, volume);

        tracks.insert("title".to_string(), title);
        tracks.insert("zen".to_string(), zen);
        tracks.insert("meloncholy".to_string(), meloncholy);

        MusicManager{
            tracks,
            current_track: "title".to_string()
        }
    }

    pub fn play(&mut self) {
        self.tracks.get_mut(&self.current_track).unwrap().play();
    }

    pub fn switch_track(&mut self, track_name: &str) {
        self.stop();
        self.current_track = track_name.to_string();
        self.play();
    }

    pub fn pause(&mut self) {
        self.tracks.get_mut(&self.current_track).unwrap().pause();
    }

    pub fn is_paused(&self) -> bool {
        self.tracks.get(&self.current_track).unwrap().is_paused()
    }

    pub fn set_volume(&mut self, volume: f32) {
        for (_, track) in &mut self.tracks {
            track.set_volume(volume);
        }
    }

    pub fn volume(&self) -> f32 {
        self.tracks.get(&self.current_track).unwrap().volume()
    }

    pub fn stop(&mut self) {
        self.tracks.get_mut(&self.current_track).unwrap().stop();
    }
}