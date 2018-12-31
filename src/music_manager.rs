use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use rand::{self, Rng};
use rodio::{decoder::Decoder, Endpoint, Sink, Source};

use loader;

pub struct MusicManager {
    tracks: HashMap<String, String>,
    current_track: String,
    thread_rng: rand::ThreadRng,
    sink: Sink,
}

impl MusicManager {
    pub fn new(audio_endpoint: &Endpoint, volume: f32) -> Self {
        let mut tracks = HashMap::new();

        tracks.insert("title".to_string(), "resources/ld39.ogg".to_string());
        tracks.insert("zen".to_string(), "resources/zen.ogg".to_string());
        tracks.insert(
            "meloncholy".to_string(),
            "resources/meloncholy.ogg".to_string(),
        );

        let mut sink = Sink::new(audio_endpoint);
        sink.set_volume(volume);

        MusicManager {
            tracks,
            current_track: "title".to_string(),
            thread_rng: rand::thread_rng(),
            sink,
        }
    }

    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn queue_track(&mut self, track_name: &str, infinite: bool) {
        self.current_track = track_name.to_string();
        let source = loader::create_sound(self.tracks.get(&self.current_track).unwrap());
        if infinite {
            self.sink.append(source.repeat_infinite());
        } else {
            self.sink.append(source);
        }
    }

    pub fn empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn stop(&mut self) {
        self.sink.stop();
    }

    pub fn setup_random_track_sink(&mut self, audio_endpoint: &Endpoint) {
        self.sink.stop();
        let volume = self.sink.volume();
        self.sink = Sink::new(audio_endpoint);
        self.sink.set_volume(volume);
    }

    pub fn play_random_game_track(&mut self) {
        let track_num: usize = self.thread_rng.gen_range(0, 2);
        if track_num == 0 {
            self.queue_track("zen", false);
        } else if track_num == 1 {
            self.queue_track("meloncholy", false);
        }
    }
}
