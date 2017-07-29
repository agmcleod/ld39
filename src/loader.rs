extern crate image;
extern crate gfx;
use std::fs::File;
use std::io::Result;
use std::io::prelude::Read;

pub fn gfx_load_texture<F, R>(path: &str, factory: &mut F) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
    where F: gfx::Factory<R>,
          R: gfx::Resources
{
    use gfx::format::Rgba8;
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]).unwrap();
    view
}

pub fn read_text_from_file(path: &str) -> Result<String> {
  let mut text = String::new();
  let mut file = try!(File::open(path));
  try!(file.read_to_string(&mut text));
  Ok(text)
}
