#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate gfx_glyph;
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
mod entities;
mod loader;
mod renderer;
mod scene;
mod spritesheet;
mod state;
mod systems;
mod utils;

use std::collections::HashMap;
use std::ops::{DerefMut};
use std::io::BufReader;
use std::fs::File;
use std::path::{Path, PathBuf};
use components::{AnimationSheet, BuildCost, Button, Camera, ClickSound, Color, CurrentPower, EntityLookup, Gatherer, HighlightTile, Input, PowerBar, Rect, ResourceCount, Resources, SelectedTile, Sprite, StateChange, Text, Tile, Transform, Wallet};
use components::ui::{WalletUI, TechTreeNode};
use entities::tech_tree;
use specs::{Entity, World, ReadStorage, WriteStorage};
use renderer::{ColorFormat, DepthFormat};
use spritesheet::Spritesheet;
use glutin::{Event, ElementState, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::GlContext;
use gfx::{Device};
use rodio::Source;
use rodio::decoder::Decoder;
use scene::Node;
use state::play_state::PlayState;
use state::StateManager;
use gfx_glyph::{font, GlyphBrush, GlyphBrushBuilder};

fn setup_world(world: &mut World, window: &glutin::Window) {
    world.add_resource::<Camera>(Camera(renderer::get_ortho()));
    world.add_resource::<StateChange>(StateChange::new());
    world.add_resource::<Input>(Input::new(window.hidpi_factor(), vec![VirtualKeyCode::W, VirtualKeyCode::A, VirtualKeyCode::S, VirtualKeyCode::D]));
    world.add_resource::<Resources>(Resources::new());
    world.add_resource::<ClickSound>(ClickSound{ play: false });
    world.add_resource::<Wallet>(Wallet::new());
    world.add_resource::<EntityLookup>(EntityLookup::new());
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
    world.register::<SelectedTile>();
    world.register::<Sprite>();
    world.register::<TechTreeNode>();
    world.register::<Text>();
    world.register::<Tile>();
    world.register::<Transform>();
    world.register::<WalletUI>();

    world.register::<tech_tree::Coal>();
    world.register::<tech_tree::Oil>();
    world.register::<tech_tree::Solar>();
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

fn render_entity<R: gfx::Resources, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>>(
    basic: &mut renderer::Basic<R>,
    encoder: &mut gfx::Encoder<R, C>,
    world: &World,
    factory: &mut F,
    spritesheet: &Spritesheet,
    asset_texture: &gfx::handle::ShaderResourceView<R, [f32; 4]>,
    glyph_brush: &mut GlyphBrush<R, F>,
    entity: &Entity,
    sprite_storage: &ReadStorage<Sprite>,
    transform_storage: &mut WriteStorage<Transform>,
    animation_storage: &ReadStorage<AnimationSheet>,
    color_storage: &ReadStorage<Color>,
    text_storage: &mut WriteStorage<Text>,
    rect_storage: &ReadStorage<Rect>,
    ) {

    if let Some(transform) = transform_storage.get_mut(*entity) {
        if transform.visible {
            if let Some(sprite) = sprite_storage.get(*entity) {
                basic.render(encoder, world, factory, &transform, Some(&sprite.frame_name), spritesheet, None, Some(asset_texture));
            }

            if let Some(animation) = animation_storage.get(*entity) {
                basic.render(encoder, world, factory, &transform, Some(animation.get_current_frame()), spritesheet, None, Some(asset_texture));
            }

            if let (Some(color), Some(_)) = (color_storage.get(*entity), rect_storage.get(*entity)) {
                basic.render(encoder, world, factory, &transform, None, spritesheet, Some(color.0), None);
            }

            if let (Some(color), Some(text)) = (color_storage.get(*entity), text_storage.get_mut(*entity)) {
                if text.text != "" && text.visible {
                    basic.render_text(encoder, &text, transform, color, glyph_brush);
                }
            }
        }
    }
}

fn render_node<R: gfx::Resources, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>>(
    node: &Node,
    basic: &mut renderer::Basic<R>,
    encoder: &mut gfx::Encoder<R, C>,
    world: &World,
    factory: &mut F,
    spritesheet: &Spritesheet,
    asset_texture: &gfx::handle::ShaderResourceView<R, [f32; 4]>,
    glyph_brush: &mut GlyphBrush<R, F>,
    sprites: &ReadStorage<Sprite>,
    transforms: &mut WriteStorage<Transform>,
    animation_sheets: &ReadStorage<AnimationSheet>,
    colors: &ReadStorage<Color>,
    texts: &mut WriteStorage<Text>,
    rects: &ReadStorage<Rect>,
    ) {
    if let Some(entity) = node.entity {
        if let Some(transform) = transforms.get(entity) {
            if !transform.visible {
                return
            }
            basic.transform(&transform, false);
        }
        render_entity(
            basic, encoder, world, factory, spritesheet, asset_texture,
            glyph_brush,
            &entity, sprites, transforms, animation_sheets, colors, texts, rects
        );
    }

    for node in &node.sub_nodes {
        render_node(
            node,
            basic, encoder, world, factory, spritesheet, asset_texture,
            glyph_brush, sprites, transforms, animation_sheets, colors, texts, rects
        );
    }

    if let Some(entity) = node.entity {
        if let Some(transform) = transforms.get(entity) {
            basic.transform(&transform, true);
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

    let exe_path = match std::env::current_exe() {
        Ok(mut p) => {
            p.pop();
            p
        },
        Err(_) => PathBuf::new(),
    };

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    let mut world = World::new();

    let (width, height, ..) = main_color.get_dimensions();

    let target = renderer::WindowTargets{
        color: main_color,
        depth: main_depth,
    };

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut basic = renderer::Basic::new(&mut factory, &target, window.hidpi_factor());

    let asset_data = loader::read_text_from_file(exe_path.join("resources/assets.json")).unwrap();
    let spritesheet: Spritesheet = serde_json::from_str(asset_data.as_ref()).unwrap();
    let asset_texture = loader::gfx_load_texture(exe_path.join("resources/assets.png"), &mut factory);

    let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(include_bytes!("../resources/MunroSmall.ttf") as &[u8])
        .build(factory.clone());

    let audio_endpoint = rodio::get_default_endpoint().unwrap();
    let click_sound_source = create_click_sound(&exe_path).buffered();
    let music = play_music(&exe_path, &audio_endpoint);

    setup_world(&mut world, &window);

    let mut state_manager = StateManager::new();
    let play_state = PlayState::new();
    state_manager.add_state(PlayState::get_name(), Box::new(play_state));
    state_manager.swap_state(PlayState::get_name(), &mut world);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent{ event, .. } => match event {
                    WindowEvent::CursorMoved{ position: (x, y), .. } => {
                        let mut input_res = world.write_resource::<Input>();
                        let input = input_res.deref_mut();
                        input.mouse_pos.0 = x as f32 / input.hidpi_factor;
                        input.mouse_pos.1 = y as f32 / input.hidpi_factor;
                    },
                    WindowEvent::MouseInput{ button: MouseButton::Left, state, .. } => {
                        let mut input_res = world.write_resource::<Input>();
                        let input = input_res.deref_mut();
                        match state {
                            ElementState::Pressed => input.mouse_pressed = true,
                            ElementState::Released => input.mouse_pressed = false,
                        };
                    },
                    WindowEvent::KeyboardInput{ input: glutin::KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Escape), .. }, .. } | glutin::WindowEvent::Closed => running = false,
                    WindowEvent::KeyboardInput{ input, .. } => {
                        let input_event = input;
                        let mut input_res = world.write_resource::<Input>();
                        let input = input_res.deref_mut();
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

        state_manager.update(&mut world);
        world.maintain();

        basic.reset_transform();

        encoder.clear(&target.color, [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0]);
        encoder.clear_depth(&target.depth, 1.0);

        {
            let sprites = world.read::<Sprite>();
            let mut transforms = world.write::<Transform>();
            let animation_sheets = world.read::<AnimationSheet>();
            let colors = world.read::<Color>();
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

            let scene = state_manager.get_current_scene();
            let scene = scene.lock().unwrap();

            for node in &scene.sub_nodes {
                render_node(node,
                &mut basic, &mut encoder, &world, &mut factory, &spritesheet, &asset_texture,
                &mut glyph_brush,
                &sprites, &mut transforms, &animation_sheets, &colors, &mut texts, &rects);
            }

            encoder.flush(&mut device);

            window.swap_buffers().unwrap();
            device.cleanup();
            basic.reset_transform();
        }

        let mut state_change = {
            let mut state_change_storage = world.write_resource::<StateChange>();
            let state_change = state_change_storage.deref_mut();
            let copy = state_change.clone();
            state_change.reset();
            copy
        };

        state_manager.process_state_change(&mut state_change, &mut world);
    }

    music.stop();
}
