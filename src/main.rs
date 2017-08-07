#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate specs;
extern crate cgmath;
extern crate serde;
extern crate image;
extern crate rodio;
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
use std::io::BufReader;
use std::fs::File;
use std::path::{Path, PathBuf};
use rusttype::{FontCollection, Font, Scale, point, PositionedGlyph};
use components::{AnimationSheet, BuildCost, Button, Camera, ClickSound, ResourceCount, Color, CurrentPower, Gatherer, GathererType, HighlightTile, Input, PowerBar, Rect, Resources, ResourceType, SelectedTile, SellCost, Sprite, Text, Tile, Transform, Upgrade, UpgradeCost, WinCount};
use specs::{DispatcherBuilder, Join, World};
use renderer::{ColorFormat, DepthFormat};
use spritesheet::Spritesheet;
use glutin::{Event, ElementState, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::GlContext;
use gfx::{Device, Factory};
use rodio::Source;
use rodio::decoder::Decoder;


fn setup_world(world: &mut World, window: &glutin::Window, font: &Arc<Font<'static>>) {
    world.add_resource::<Camera>(Camera(renderer::get_ortho()));
    world.add_resource::<Input>(Input::new(window.hidpi_factor(), vec![VirtualKeyCode::W, VirtualKeyCode::A, VirtualKeyCode::S, VirtualKeyCode::D]));
    world.add_resource::<Resources>(Resources::new());
    world.add_resource::<ClickSound>(ClickSound{ play: false });
    world.register::<AnimationSheet>();
    world.register::<BuildCost>();
    world.register::<Button>();
    world.register::<CurrentPower>();
    world.register::<Color>();
    world.register::<ResourceCount>();
    world.register::<Gatherer>();
    world.register::<HighlightTile>();
    world.register::<PowerBar>();
    world.register::<Rect>();
    world.register::<SellCost>();
    world.register::<SelectedTile>();
    world.register::<Sprite>();
    world.register::<Text>();
    world.register::<Tile>();
    world.register::<Transform>();
    world.register::<Upgrade>();
    world.register::<UpgradeCost>();
    world.register::<WinCount>();
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
        .with(ResourceCount{ resource_type: ResourceType::Coal })
        .with(Transform::new(670, 500, 32, 32, 0.0, 1.0, 1.0))
        .with(Sprite{ frame_name: "coal.png".to_string(), visible: true });

    world.create_entity()
        .with(ResourceCount{ resource_type: ResourceType::Coal })
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

    // upgrade stuff
    let mut text = Text::new(&font, 32.0);
    text.visible = false;
    text.set_text(format!("{}", Upgrade::new().get_cost()));
    world.create_entity()
        .with(UpgradeCost{})
        .with(text)
        .with(Transform::new(750, 100, 32, 32, 0.0, 1.0, 1.0))
        .with(Color([0.0, 1.0, 0.0, 1.0]));

    // build
    let mut text = Text::new(&font, 32.0);
    text.set_text(format!("{}", GathererType::Coal.get_build_cost()));
    world.create_entity()
        .with(BuildCost{})
        .with(Transform::new(775, 32, 0, 0, 0.0, 1.0, 1.0))
        .with(text)
        .with(Color([0.0, 1.0, 0.0, 1.0]));

    // sell
    let mut text = Text::new(&font, 32.0);
    text.set_text("10".to_string());
    world.create_entity()
        .with(SellCost{})
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

fn create_click_sound(root_path: &PathBuf) -> Decoder<BufReader<File>> {
    let audio_file = File::open(&Path::new(&root_path.join("resources/click.ogg"))).unwrap();
    rodio::Decoder::new(BufReader::new(audio_file)).unwrap()
}

fn play_music(root_path: &PathBuf, endpoint: &rodio::Endpoint) -> rodio::Sink {
    let sink = rodio::Sink::new(&endpoint);

    let music_file = File::open(&Path::new(&root_path.join("resources/ld39.ogg"))).unwrap();
    let source = rodio::Decoder::new(BufReader::new(music_file)).unwrap();
    sink.append(source.repeat_infinite());

    sink.play();
    sink
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let dim = renderer::get_dimensions();
    let builder = glutin::WindowBuilder::new()
        .with_title("ld39".to_string())
        .with_dimensions(dim[0] as u32, dim[1] as u32);
    let context = glutin::ContextBuilder::new();

    let exe_path = match std::env::current_exe() {
        Ok(mut p) => {
            p.pop();
            p
        },
        Err(_) => PathBuf::new(),
    };

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .add(systems::AnimationSystem::new(), "animation_system", &[])
        .add(systems::PowerUsage::new(), "power_system", &[])
        .add(systems::TileSelection{}, "tile_selection", &[])
        .add(systems::ButtonHover{}, "button_hover", &[])
        .add(systems::SellEnergy{}, "sell_energy", &["button_hover"])
        .add(systems::BuildGatherer{ built_one: false }, "build_gatherer", &["button_hover"])
        .add(systems::Gathering{}, "gathering", &[])
        .add(systems::UpgradeResource{}, "upgrade_resource", &[])
        .build();

    let target = renderer::WindowTargets{
        color: main_color,
        depth: main_depth,
    };

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut basic = renderer::Basic::new(&mut factory, &target);

    let asset_data = loader::read_text_from_file(exe_path.join("resources/assets.json")).unwrap();
    let spritesheet: Spritesheet = serde_json::from_str(asset_data.as_ref()).unwrap();
    let asset_texture = loader::gfx_load_texture(exe_path.join("resources/assets.png"), &mut factory);

    let font_data = include_bytes!("../resources/MunroSmall.ttf");
    let font_collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = Arc::new(font_collection.into_font().unwrap());
    let mut glyph_cache: HashMap<String, renderer::text::GlyphCacheEntry<gfx_device_gl::Resources>> = HashMap::new();

    let audio_endpoint = rodio::get_default_endpoint().unwrap();
    let click_sound_source = create_click_sound(&exe_path).buffered();
    let music = play_music(&exe_path, &audio_endpoint);

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
                        if let Some(key) = input_event.virtual_keycode {
                            if input.pressed_keys.contains_key(&key) {
                                match input_event.state {
                                    ElementState::Pressed => input.pressed_keys.insert(key, true),
                                    ElementState::Released => input.pressed_keys.insert(key, false),
                                };
                            }
                        }
                    },
                    _ => {}
                },
                _ => ()
            }
        });

        dispatcher.dispatch(&mut world.res);
        world.maintain();

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

        let mut click_sound_storage = world.write_resource::<ClickSound>();
        let click_sound: &mut ClickSound = click_sound_storage.deref_mut();
        if click_sound.play {
            click_sound.play = false;
            let sink = rodio::Sink::new(&audio_endpoint);

            sink.append(click_sound_source.clone());
            sink.play();
            sink.detach();
        }

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
                    renderer::text::create_texture_from_glyph(&mut glyph_cache, &font, text, &mut factory);
                }
                let entry = glyph_cache.get(&text.text).unwrap();
                transform.size.x = entry.width;
                transform.size.y = entry.height;
            }

            if text.text != "" && text.visible {
                basic.render(&mut encoder, &world, &mut factory, &transform, None, &spritesheet, Some(color.0), Some(&glyph_cache.get(&text.text).unwrap().view));
            }
        }

        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }

    music.stop();
}
