extern crate gfx;
extern crate image;
use gfx::texture::Mipmap;
use rodio::{decoder::Decoder, Decoder as SoundDecoder};
use serde_json;
use settings::Settings;
use std::env;
use std::fs::File;
use std::io::prelude::Read;
use std::io::BufReader;
use std::io::Result;
use std::path::{Path, PathBuf};

pub fn gfx_load_texture<F, R>(
    path: &str,
    factory: &mut F,
) -> (gfx::handle::ShaderResourceView<R, [f32; 4]>, u16, u16)
where
    F: gfx::Factory<R>,
    R: gfx::Resources,
{
    use gfx::format::Rgba8;
    let path = get_exe_path().join(path);
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory
        .create_texture_immutable_u8::<Rgba8>(kind, Mipmap::Allocated, &[&img])
        .unwrap();
    (view, width as u16, height as u16)
}

pub fn create_sound(sound_file_path: &str) -> Decoder<BufReader<File>> {
    let audio_file = File::open(&Path::new(&get_exe_path().join(sound_file_path))).unwrap();
    SoundDecoder::new(BufReader::new(audio_file)).unwrap()
}

pub fn read_text_from_file(path: &str) -> Result<String> {
    let path = get_exe_path().join(path);
    let mut text = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

pub fn get_exe_path() -> PathBuf {
    match env::current_exe() {
        Ok(mut p) => {
            p.pop();
            p
        }
        Err(_) => PathBuf::new(),
    }
}

pub fn load_settings() -> Settings {
    let file_name = "settings.json";
    if get_exe_path().join(file_name).exists() {
        let settings_text = read_text_from_file(file_name).unwrap();
        serde_json::from_str(settings_text.as_ref()).unwrap()
    } else {
        Settings::default()
    }
}
