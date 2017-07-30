#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate specs;
extern crate cgmath;
extern crate serde;
extern crate image;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate rusttype;

mod components;
mod loader;
mod renderer;
mod spritesheet;
mod systems;
mod utils;

use std::collections::HashMap;
use std::sync::Arc;
use std::ops::{DerefMut};
use rusttype::{FontCollection, Font, Scale, point, PositionedGlyph};
use components::{AnimationSheet, Button, Camera, CoalCount, Color, CurrentPower, Gatherer, HighlightTile, Input, PowerBar, Rect, Resources, SelectedTile, Sprite, Text, Tile, Transform};
use specs::{DispatcherBuilder, Join, World};
use renderer::{ColorFormat, DepthFormat};
use spritesheet::Spritesheet;
use glutin::{Event, ElementState, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::GlContext;
use gfx::{Device, Factory};

fn setup_world(world: &mut World, window: &glutin::Window, font: &Arc<Font<'static>>) {
    world.add_resource::<Camera>(Camera(renderer::get_ortho()));
    world.add_resource::<Input>(Input::new(window.hidpi_factor(), vec![VirtualKeyCode::W, VirtualKeyCode::A, VirtualKeyCode::S, VirtualKeyCode::D]));
    world.add_resource::<Resources>(Resources::new());
    world.register::<AnimationSheet>();
    world.register::<Button>();
    world.register::<CurrentPower>();
    world.register::<Color>();
    world.register::<CoalCount>();
    world.register::<Gatherer>();
    world.register::<HighlightTile>();
    world.register::<PowerBar>();
    world.register::<Rect>();
    world.register::<SelectedTile>();
    world.register::<Sprite>();
    world.register::<Text>();
    world.register::<Tile>();
    world.register::<Transform>();
    world.create_entity()
        .with(PowerBar::new())
        .with(Transform::new(670, 576, 260, 32, 0.0, 1.0, 1.0))
        .with(Sprite{ frame_name: "powerbar.png".to_string(), visible: true });

    world.create_entity()
        .with(CurrentPower{})
        .with(Transform::new(674, 580, CurrentPower::get_max_with(), 24, 0.0, 1.0, 1.0))
        .with(Rect{})
        .with(Color([0.0, 1.0, 0.0, 1.0]));

    world.create_entity()
        .with(CoalCount{})
        .with(Transform::new(670, 500, 32, 32, 0.0, 1.0, 1.0))
        .with(Sprite{ frame_name: "coal.png".to_string(), visible: true });

    world.create_entity()
        .with(CoalCount{})
        .with(Transform::new(720, 500, 32, 32, 0.0, 1.0, 1.0))
        .with(Text::new(&font, 32.0))
        .with(Color([0.0, 1.0, 0.0, 1.0]));

    world.create_entity()
        .with(HighlightTile{ visible: false })
        .with(Transform::new(0, 0, 64, 64, 0.0, 1.0, 1.0))
        .with(Color([1.0, 1.0, 1.0, 0.3]));

    world.create_entity()
        .with(SelectedTile{ visible: false })
        .with(Transform::new(0, 0, 64, 64, 0.0, 1.0, 1.0))
        .with(Color([1.0, 1.0, 1.0, 0.6]));

    world.create_entity()
        .with(Button::new("build".to_string(), ["build.png".to_string(), "build_hover.png".to_string()]))
        .with(Transform::new(670, 32, 96, 32, 0.0, 1.0, 1.0))
        .with(Sprite{ frame_name: "build.png".to_string(), visible: true });

    world.create_entity()
        .with(Button::new("sell".to_string(), ["sell.png".to_string(), "sell_hover.png".to_string()]))
        .with(Transform::new(820, 32, 96, 32, 0.0, 1.0, 1.0))
        .with(Sprite{ frame_name: "sell.png".to_string(), visible: true });

    let mut text = Text::new(&font, 32.0);
    text.set_text("20".to_string());
    world.create_entity()
        .with(Transform::new(775, 32, 0, 0, 0.0, 1.0, 1.0))
        .with(text)
        .with(Color([0.0, 1.0, 0.0, 1.0]));

    let mut text = Text::new(&font, 32.0);
    text.set_text("10".to_string());
    world.create_entity()
        .with(Transform::new(925, 32, 0, 0, 0.0, 1.0, 1.0))
        .with(text)
        .with(Color([0.0, 1.0, 0.0, 1.0]));

    for row in 0i32..10i32 {
        for col in 0i32..10i32 {
            let size = Tile::get_size();
            world.create_entity()
                .with(Transform::new(size * col, size * row, size as u16, size as u16, 0.0, 1.0, 1.0))
                .with(Sprite{ frame_name: "tiles.png".to_string(), visible: true })
                .with(Tile{});
        }
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let dim = renderer::get_dimensions();
    let builder = glutin::WindowBuilder::new()
        .with_title("ld39".to_string())
        .with_dimensions(dim[0] as u32, dim[1] as u32);
    let context = glutin::ContextBuilder::new();

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .add(systems::AnimationSystem::new(), "animation_system", &[])
        .add(systems::PowerUsage::new(), "power_system", &[])
        .add(systems::TileSelection{}, "tile_selection", &[])
        .add(systems::ButtonHover{}, "button_hover", &[])
        .add(systems::SellEnergy{}, "sell_energy", &["button_hover"])
        .add(systems::BuildGatherer{}, "build_gatherer", &["button_hover"])
        .build();

    let target = renderer::WindowTargets{
        color: main_color,
        depth: main_depth,
    };

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut basic = renderer::Basic::new(&mut factory, &target);

    let asset_data = loader::read_text_from_file("./resources/assets.json").unwrap();
    let spritesheet: Spritesheet = serde_json::from_str(asset_data.as_ref()).unwrap();
    let asset_texture = loader::gfx_load_texture("./resources/assets.png", &mut factory);

    let font_data = include_bytes!("../resources/MunroSmall.ttf");
    let font_collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = Arc::new(font_collection.into_font().unwrap());
    let mut glyph_cache: HashMap<String, gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>> = HashMap::new();

    setup_world(&mut world, &window, &font);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent{ event, .. } => match event {
                    WindowEvent::MouseMoved{ position: (x, y), .. } => {
                        let mut input_res = world.write_resource::<Input>();
                        let mut input = input_res.deref_mut();
                        input.mouse_pos.0 = (x as f32 / input.hidpi_factor) as i32;
                        input.mouse_pos.1 = (y as f32 / input.hidpi_factor) as i32;
                    },
                    WindowEvent::MouseInput{ button: MouseButton::Left, state, .. } => {
                        let mut input_res = world.write_resource::<Input>();
                        let mut input = input_res.deref_mut();
                        match state {
                            ElementState::Pressed => input.mouse_pressed = true,
                            ElementState::Released => input.mouse_pressed = false,
                        };
                    },
                    WindowEvent::KeyboardInput{ input: glutin::KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Escape), .. }, .. } | glutin::WindowEvent::Closed => running = false,
                    WindowEvent::KeyboardInput{ input, .. } => {
                        let input_event = input;
                        let mut input_res = world.write_resource::<Input>();
                        let mut input = input_res.deref_mut();
                        let key = input_event.virtual_keycode.unwrap();
                        if input.pressed_keys.contains_key(&key) {
                            match input_event.state {
                                ElementState::Pressed => input.pressed_keys.insert(key, true),
                                ElementState::Released => input.pressed_keys.insert(key, false),
                            };
                        }
                    },
                    _ => {}
                },
                _ => ()
            }
        });

        dispatcher.dispatch(&mut world.res);

        basic.reset_transform();

        encoder.clear(&target.color, [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0]);
        encoder.clear_depth(&target.depth, 1.0);

        let sprites = world.read::<Sprite>();
        let mut transforms = world.write::<Transform>();
        let animation_sheets = world.read::<AnimationSheet>();
        let colors = world.read::<Color>();
        let highlight_tiles = world.read::<HighlightTile>();
        let selected_tiles = world.read::<SelectedTile>();
        let mut texts = world.write::<Text>();
        let rects = world.read::<Rect>();

        for (sprite, transform) in (&sprites, &transforms).join() {
            if sprite.visible {
                basic.render(&mut encoder, &world, &mut factory, &transform, Some(&sprite.frame_name), &spritesheet, None, Some(&asset_texture));
            }
        }

        for (animation_sheet, transform) in (&animation_sheets, &transforms).join() {
            basic.render(&mut encoder, &world, &mut factory, &transform, Some(animation_sheet.get_current_frame()), &spritesheet, None, Some(&asset_texture));
        }

        for (highlight_tile, color, transform) in (&highlight_tiles, &colors, &transforms).join() {
            if highlight_tile.visible {
                basic.render(&mut encoder, &world, &mut factory, &transform, None, &spritesheet, Some(color.0), None);
            }
        }

        for (selected_tile, color, transform) in (&selected_tiles, &colors, &transforms).join() {
            if selected_tile.visible {
                basic.render(&mut encoder, &world, &mut factory, &transform, None, &spritesheet, Some(color.0), None);
            }
        }

        for (color, transform, _) in (&colors, &transforms, &rects).join() {
            basic.render(&mut encoder, &world, &mut factory, &transform, None, &spritesheet, Some(color.0), None);
        }

        for (color, transform, text) in (&colors, &mut transforms, &mut texts).join() {
            if text.new_data {
                text.new_data = false;
                if !glyph_cache.contains_key(&text.text) {
                    let glyphs: Vec<_> = font.layout(text.text.as_ref(), text.scale, text.offset).collect();
                    let pixel_height = text.scale.y.ceil() as usize;
                    let width = text.calc_text_width(&glyphs) as usize;
                    let mut pixel_data = vec![0u8; 4 * width * pixel_height];
                    let mapping_scale = 255.0;
                    for g in glyphs {
                        if let Some(bb) = g.pixel_bounding_box() {
                            // v is the amount of the pixel covered
                            // by the glyph, in the range 0.0 to 1.0
                            g.draw(|x, y, v| {
                                let v = (v * mapping_scale + 0.5) as u8;
                                let x = x as i32 + bb.min.x;
                                let y = y as i32 + bb.min.y;
                                // There's still a possibility that the glyph clips the boundaries of the bitmap
                                if v > 0 && x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
                                    let i = (x as usize + y as usize * width) * 4;
                                    pixel_data[i] = 255;
                                    pixel_data[i + 1] = 255;
                                    pixel_data[i + 2] = 255;
                                    pixel_data[i + 3] = v;
                                }
                            })
                        }
                    }

                    transform.size.x = width as u16;
                    transform.size.y = pixel_height as u16;

                    let kind = gfx::texture::Kind::D2(
                        width as gfx::texture::Size,
                        pixel_height as gfx::texture::Size,
                        gfx::texture::AaMode::Single,
                    );
                    let (_, view) = factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&pixel_data]).unwrap();
                    glyph_cache.insert(text.text.clone(), view);
                }
            }

            basic.render(&mut encoder, &world, &mut factory, &transform, None, &spritesheet, Some(color.0), Some(glyph_cache.get(&text.text).unwrap()));
        }

        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
