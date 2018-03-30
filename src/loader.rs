extern crate image;
extern crate gfx;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Result;
use std::io::prelude::Read;
use gfx::texture::Mipmap;
use std::io::BufReader;
use rodio::{
    Decoder as SoundDecoder,
    Endpoint,
    Sink,
    Source,
    decoder::Decoder,
};

pub fn gfx_load_texture
<F, R>(path: &str, factory: &mut F) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
    where F: gfx::Factory<R>,
          R: gfx::Resources
{
    use gfx::format::Rgba8;
    let path = get_exe_path().join(path);
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, Mipmap::Allocated, &[&img]).unwrap();
    view
}

pub fn create_sound(sound_file_path: &str) -> Decoder<BufReader<File>> {
    let audio_file = File::open(&Path::new(&get_exe_path().join(sound_file_path))).unwrap();
    SoundDecoder::new(BufReader::new(audio_file)).unwrap()
}

pub fn create_music_sink(music_path: &str, endpoint: &Endpoint) -> Sink {
    let sink = Sink::new(&endpoint);

    let music_file = File::open(&Path::new(&get_exe_path().join(music_path))).unwrap();
    let source = SoundDecoder::new(BufReader::new(music_file)).unwrap();
    sink.append(source.repeat_infinite());
    sink
}

pub fn read_text_from_file(path: &str) -> Result<String> {
    let path = get_exe_path().join(path);
    let mut text = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn get_exe_path() -> PathBuf {
    match env::current_exe() {
        Ok(mut p) => {
            p.pop();
            p
        },
        Err(_) => PathBuf::new(),
    }
}
