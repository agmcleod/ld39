use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;

use rand::{self, Rng};
use rodio::{Endpoint, Sink, Source, decoder::Decoder};

use loader;

pub struct MusicManager {
    tracks: HashMap<String, Decoder<BufReader<File>>>,
    current_track: String,
    thread_rng: rand::ThreadRng,
    sink: Sink,
}

impl MusicManager {
    pub fn new(audio_endpoint: &Endpoint, volume: f32) -> Self {
        let mut tracks = HashMap::new();

        let zen = loader::create_sound("resources/zen.ogg");
        let meloncholy = loader::create_sound("resources/meloncholy.ogg");
        let title = loader::create_sound("resources/ld39.ogg");

        tracks.insert("title".to_string(), title);
        tracks.insert("zen".to_string(), zen);
        tracks.insert("meloncholy".to_string(), meloncholy);

        let mut sink = Sink::new(audio_endpoint);
        sink.set_volume(volume);

        MusicManager{
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
        let source = self.tracks.get(&self.current_track).unwrap();
        let source = source.to_owned();
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

    pub fn play_random_game_track(&mut self) {
        let track_num: usize = self.thread_rng.gen_range(0, 2);
        if track_num == 0 {
            self.queue_track("zen", false);
        } else if track_num == 1{
            self.queue_track("meloncholy", false);
        }
    }
}