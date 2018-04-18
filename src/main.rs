extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_glyph;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate lyon_path;
extern crate lyon_tessellation;
extern crate rodio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate specs;

extern crate rusttype;

mod components;
mod entities;
mod loader;
mod renderer;
mod scene;
mod spritesheet;
mod state;
mod storage_types;
mod systems;
mod utils;

use std::ops::DerefMut;
use std::time;
use components::{AnimationSheet, BuildCost, Button, Camera, ClickSound, Color, CurrentPower,
                 EntityLookup, Gatherer, HighlightTile, Input, PowerBar, Rect,
                 ResourceCount, Resources, SelectedTile, Shape, Sprite, StateChange, Text, Tile,
                 Transform, Wallet, upgrade::{LearnProgress, Upgrade}};
use components::ui::{TechTreeButton, WalletUI};
use specs::{Entity, ReadStorage, World, WriteStorage};
use renderer::{ColorFormat, DepthFormat};
use spritesheet::Spritesheet;
use glutin::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::GlContext;
use gfx::Device;
use rodio::Source;
use scene::Node;
use state::play_state::PlayState;
use state::StateManager;
use gfx_glyph::{GlyphBrush, GlyphBrushBuilder};
use utils::math;
use systems::FRAME_TIME;

fn setup_world(world: &mut World, window: &glutin::Window) {
    world.add_resource::<Camera>(Camera(renderer::get_ortho()));
    world.add_resource::<StateChange>(StateChange::new());
    world.add_resource::<Input>(Input::new(
        window.hidpi_factor(),
        vec![
            VirtualKeyCode::W,
            VirtualKeyCode::A,
            VirtualKeyCode::S,
            VirtualKeyCode::D,
        ],
    ));
    world.add_resource::<Resources>(Resources::new());
    world.add_resource::<ClickSound>(ClickSound { play: false });
    world.add_resource::<Wallet>(Wallet::new());
    world.add_resource::<EntityLookup>(EntityLookup::new());
    world.register::<AnimationSheet>();
    world.register::<BuildCost>();
    world.register::<Button>();
    world.register::<CurrentPower>();
    world.register::<Color>();
    world.register::<Gatherer>();
    world.register::<HighlightTile>();
    world.register::<LearnProgress>();
    world.register::<PowerBar>();
    world.register::<Rect>();
    world.register::<ResourceCount>();
    world.register::<SelectedTile>();
    world.register::<Shape>();
    world.register::<Sprite>();
    world.register::<TechTreeButton>();
    world.register::<Text>();
    world.register::<Tile>();
    world.register::<Transform>();
    world.register::<Upgrade>();
    world.register::<WalletUI>();
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
    shape_storage: &ReadStorage<Shape>,
) {
    if let Some(transform) = transform_storage.get_mut(*entity) {
        if transform.visible {
            if let Some(sprite) = sprite_storage.get(*entity) {
                basic.render(
                    encoder,
                    world,
                    factory,
                    &transform,
                    Some(&sprite.frame_name),
                    spritesheet,
                    color_storage.get(*entity),
                    Some(asset_texture),
                );
            }

            if let Some(animation) = animation_storage.get(*entity) {
                basic.render(
                    encoder,
                    world,
                    factory,
                    &transform,
                    Some(animation.get_current_frame()),
                    spritesheet,
                    color_storage.get(*entity),
                    Some(asset_texture),
                );
            }

            if let (Some(color), Some(_)) = (color_storage.get(*entity), rect_storage.get(*entity))
            {
                basic.render(
                    encoder,
                    world,
                    factory,
                    &transform,
                    None,
                    spritesheet,
                    Some(color),
                    None,
                );
            }

            if let (Some(color), Some(text)) =
                (color_storage.get(*entity), text_storage.get_mut(*entity))
            {
                if text.text != "" && text.visible {
                    basic.render_text(encoder, &text, transform, color, glyph_brush);
                }
            }

            if let Some(shape) = shape_storage.get(*entity) {
                basic.render_shape(encoder, world, factory, &shape);
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
    shapes: &ReadStorage<Shape>,
) {
    if let Some(entity) = node.entity {
        if let Some(transform) = transforms.get(entity) {
            if !transform.visible {
                return;
            }
            basic.transform(&transform, false);
        }
        render_entity(
            basic,
            encoder,
            world,
            factory,
            spritesheet,
            asset_texture,
            glyph_brush,
            &entity,
            sprites,
            transforms,
            animation_sheets,
            colors,
            texts,
            rects,
            shapes,
        );
    }

    for node in &node.sub_nodes {
        render_node(
            node,
            basic,
            encoder,
            world,
            factory,
            spritesheet,
            asset_texture,
            glyph_brush,
            sprites,
            transforms,
            animation_sheets,
            colors,
            texts,
            rects,
            shapes,
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

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    let mut world = World::new();

    let target = renderer::WindowTargets {
        color: main_color,
        depth: main_depth,
    };

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut basic = renderer::Basic::new(&mut factory, &target, window.hidpi_factor());

    let asset_data = loader::read_text_from_file("resources/assets.json").unwrap();
    let spritesheet: Spritesheet = serde_json::from_str(asset_data.as_ref()).unwrap();
    let asset_texture = loader::gfx_load_texture("resources/assets.png", &mut factory);

    let mut glyph_brush =
        GlyphBrushBuilder::using_font_bytes(include_bytes!("../resources/MunroSmall.ttf") as &[u8])
            .build(factory.clone());

    let audio_endpoint = rodio::default_endpoint().unwrap();
    let click_sound_source = loader::create_sound("resources/click.ogg").buffered();
    let music = loader::create_music_sink("resources/ld39.ogg", &audio_endpoint);
    music.play();

    setup_world(&mut world, &window);

    let mut state_manager = StateManager::new();
    let play_state = PlayState::new();
    state_manager.add_state(PlayState::get_name(), Box::new(play_state));
    state_manager.swap_state(PlayState::get_name(), &mut world);

    let mut running = true;
    let mut frame_start = time::Instant::now();
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved {
                    position: (x, y), ..
                } => {
                    let mut input_res = world.write_resource::<Input>();
                    let input = input_res.deref_mut();
                    input.mouse_pos.0 = x as f32 / input.hidpi_factor;
                    input.mouse_pos.1 = y as f32 / input.hidpi_factor;
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,
                    ..
                } => {
                    let mut input_res = world.write_resource::<Input>();
                    let input = input_res.deref_mut();
                    match state {
                        ElementState::Pressed => input.mouse_pressed = true,
                        ElementState::Released => input.mouse_pressed = false,
                    };
                }
                WindowEvent::KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | glutin::WindowEvent::Closed => running = false,
                WindowEvent::KeyboardInput { input, .. } => {
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
                }
                _ => {}
            },
            _ => (),
        });

        let duration = time::Instant::now() - frame_start;
        if math::get_seconds(&duration) < FRAME_TIME {
            continue;
        }

        frame_start = time::Instant::now();

        state_manager.update(&mut world);
        world.maintain();

        basic.reset_transform();

        encoder.clear(
            &target.color,
            [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0],
        );
        encoder.clear_depth(&target.depth, 1.0);

        {
            let sprites = world.read::<Sprite>();
            let mut transforms = world.write::<Transform>();
            let animation_sheets = world.read::<AnimationSheet>();
            let colors = world.read::<Color>();
            let mut texts = world.write::<Text>();
            let rects = world.read::<Rect>();
            let shapes = world.read::<Shape>();

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
                render_node(
                    node,
                    &mut basic,
                    &mut encoder,
                    &world,
                    &mut factory,
                    &spritesheet,
                    &asset_texture,
                    &mut glyph_brush,
                    &sprites,
                    &mut transforms,
                    &animation_sheets,
                    &colors,
                    &mut texts,
                    &rects,
                    &shapes,
                );
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
