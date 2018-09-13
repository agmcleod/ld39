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
extern crate rand;
#[macro_use]
extern crate rand_derive;
extern crate rodio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate conrod;
extern crate serde_json;
extern crate specs;

extern crate rusttype;
extern crate winit;

mod components;
mod entities;
mod loader;
mod renderer;
mod settings;
mod spritesheet;
mod state;
mod storage_types;
mod systems;
mod utils;

use components::ui::{PollutionCount, TechTreeButton, TutorialUI, WalletUI};
use components::{upgrade::{LearnProgress, Upgrade},
                 Actions,
                 AnimationSheet,
                 Button,
                 Camera,
                 ClickSound,
                 Color,
                 DeltaTime,
                 EntityLookup,
                 FloatingText,
                 Gatherer,
                 HighlightTile,
                 Input,
                 Node,
                 PowerBar,
                 Pulse,
                 Rect,
                 ResourceCount,
                 SelectedTile,
                 Shape,
                 Sprite,
                 StateChange,
                 Text,
                 Tile,
                 Transform,
                 TutorialStep};

use gfx::Device;
use gfx_glyph::{GlyphBrush, GlyphBrushBuilder};
use glutin::{ElementState, Event, GlContext, MouseButton, VirtualKeyCode, WindowEvent};
use renderer::{ColorFormat, DepthFormat};
use rodio::Source;
use settings::Settings;
use specs::{Entity, ReadStorage, World, WriteStorage};
use spritesheet::Spritesheet;
use state::play_state::PlayState;
use state::StateManager;
use std::ops::DerefMut;
use std::time;
use utils::math;

fn setup_world(world: &mut World, window: &glutin::Window) {
    let dim = renderer::get_dimensions();
    world.add_resource::<Camera>(Camera(renderer::get_ortho(dim[0], dim[1])));
    world.add_resource::<StateChange>(StateChange::new());
    world.add_resource::<Input>(Input::new(
        window.hidpi_factor(),
        vec![VirtualKeyCode::Escape],
    ));
    world.add_resource::<ClickSound>(ClickSound { play: false });
    world.add_resource::<DeltaTime>(DeltaTime { dt: 0.0 });
    world.add_resource(Actions::new());
    world.register::<AnimationSheet>();
    world.register::<Button>();
    world.register::<Color>();
    world.register::<FloatingText>();
    world.register::<Gatherer>();
    world.register::<HighlightTile>();
    world.register::<LearnProgress>();
    world.register::<Node>();
    world.register::<PollutionCount>();
    world.register::<PowerBar>();
    world.register::<Pulse>();
    world.register::<Rect>();
    world.register::<ResourceCount>();
    world.register::<SelectedTile>();
    world.register::<Shape>();
    world.register::<Sprite>();
    world.register::<TechTreeButton>();
    world.register::<Text>();
    world.register::<Tile>();
    world.register::<Transform>();
    world.register::<TutorialUI>();
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
    scale_from_base_res: &(f32, f32),
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
                    basic.render_text(encoder, &text, transform, color, glyph_brush, world.read_resource::<Input>().hidpi_factor, scale_from_base_res);
                }
            }

            if let Some(shape) = shape_storage.get(*entity) {
                basic.render_shape(encoder, world, factory, &shape);
            }
        }
    }
}

fn render_node<R: gfx::Resources, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>>(
    basic: &mut renderer::Basic<R>,
    encoder: &mut gfx::Encoder<R, C>,
    entity: Entity,
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
    nodes: &mut WriteStorage<Node>,
    scale_from_base_res: &(f32, f32),
) {
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
        scale_from_base_res,
    );

    let mut entities = Vec::new();
    if let Some(node) = nodes.get_mut(entity) {
        node.sort_children(world, transforms);
        entities.append(&mut node.entities.iter().cloned().collect());
    }

    for entity in &entities {
        render_node(
            basic,
            encoder,
            *entity,
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
            nodes,
            scale_from_base_res,
        );
    }

    if let Some(transform) = transforms.get(entity) {
        basic.transform(&transform, true);
    }
}

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let dim = renderer::get_dimensions();
    let builder = glutin::WindowBuilder::new()
        .with_title("ld39".to_string())
        .with_dimensions(dim[0] as u32, dim[1] as u32);
    let context = glutin::ContextBuilder::new().with_vsync(true);

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    let mut world = World::new();

    let mut target = renderer::WindowTargets {
        color: main_color,
        depth: main_depth,
    };

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let hidpi_factor = window.hidpi_factor();
    let mut basic = renderer::Basic::new(&mut factory, target);

    let asset_data = loader::read_text_from_file("resources/assets.json").unwrap();
    let spritesheet: Spritesheet = serde_json::from_str(asset_data.as_ref()).unwrap();
    let asset_texture = loader::gfx_load_texture("resources/assets.png", &mut factory);

    let mut glyph_brush =
        GlyphBrushBuilder::using_font_bytes(include_bytes!("../resources/MunroSmall.ttf") as &[u8])
            .build(factory.clone());

    let audio_endpoint = rodio::default_endpoint().unwrap();
    let click_sound_source = loader::create_sound("resources/click.ogg").buffered();
    let settings = loader::load_settings();
    let mut music =
        loader::create_music_sink("resources/ld39.ogg", &audio_endpoint, settings.music_volume);

    if settings.mute_music {
        music.pause();
    }

    setup_world(&mut world, &window);

    let mut state_manager = StateManager::new();
    let play_state = PlayState::new();
    state_manager.add_state(PlayState::get_name(), Box::new(play_state));
    state_manager.swap_state(PlayState::get_name(), &mut world);

    let mut running = true;
    let mut frame_start = time::Instant::now();

    let mut conrod_renderer = conrod::backend::gfx::Renderer::new(
        &mut factory,
        &basic.target.color,
        window.hidpi_factor() as f64,
    ).unwrap();
    let image_map = conrod::image::Map::new();

    {
        let mut actions = world.write_resource::<Actions>();
        if !settings.completed_tutorial {
            actions.dispatch(TutorialStep::SelectTile.as_string());
            let mut tutorial_step = world.write_resource::<TutorialStep>();
            *tutorial_step = TutorialStep::SelectTile;
        }
    }

    world.add_resource(settings);

    // let mut frame_time_text = components::Text::new(25.0, 200, 30);
    // let frame_time_transform = components::Transform::visible(20.0, 20.0, 10.0, 200, 30, 0.0, 1.0, 1.0);
    // let frame_time_color = components::Color([1.0, 0.0, 0.0, 1.0]);

    let mut scale_from_base_res = (1.0, 1.0);
    let mut scale_to_base_res = (1.0, 1.0);

    while running {
        let duration = time::Instant::now() - frame_start;

        frame_start = time::Instant::now();

        events_loop.poll_events(|event| {
            if state_manager.should_render_ui() {
                let ui = state_manager.get_ui_to_render();
                if let Some(event) =
                    conrod::backend::winit::convert_event(event.clone(), window.window())
                {
                    ui.handle_event(event);
                }
            }

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CursorMoved {
                        position: (x, y), ..
                    } => {
                        let mut input_res = world.write_resource::<Input>();
                        let input = input_res.deref_mut();
                        input.mouse_pos.0 = x as f32 / input.hidpi_factor * scale_to_base_res.0;
                        input.mouse_pos.1 = y as f32 / input.hidpi_factor * scale_to_base_res.1;
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
                    glutin::WindowEvent::Closed => running = false,
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
                    },
                    WindowEvent::HiDPIFactorChanged(factor) => {
                        let mut input_res = world.write_resource::<Input>();
                        input_res.hidpi_factor = factor;
                    },
                    WindowEvent::Resized(w, h) => {
                        window.resize(w, h);
                        let target = &mut basic.target;
                        gfx_window_glutin::update_views(&window, &mut target.color, &mut target.depth);
                        let input = world.read_resource::<Input>();
                        let w = w as f32;
                        let h = h as f32;
                        let hidpi_factor = input.hidpi_factor;
                        scale_from_base_res = (w / hidpi_factor / dim[0], h / hidpi_factor / dim[1]);
                        scale_to_base_res = (dim[0] / (w / hidpi_factor), dim[1] / (h / hidpi_factor));
                        conrod_renderer.on_resize(basic.target.color);
                    },
                    _ => {}
                },
                _ => (),
            }
        });

        {
            let mut dt = world.write_resource::<DeltaTime>();
            dt.dt = math::get_seconds(&duration);
        }

        state_manager.update(&mut world);
        world.maintain();

        {
            let mut actions = world.write_resource::<Actions>();
            actions.clear();
        }

        basic.reset_transform();

        encoder.clear(
            &basic.target.color,
            [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0],
        );
        encoder.clear_depth(&basic.target.depth, 1.0);

        {
            let root_node = {
                let lookup = world.read_resource::<EntityLookup>();
                lookup.entities.get("root").unwrap().clone()
            };

            let sprites = world.read_storage::<Sprite>();
            let mut transforms = world.write_storage::<Transform>();
            let animation_sheets = world.read_storage::<AnimationSheet>();
            let colors = world.read_storage::<Color>();
            let mut texts = world.write_storage::<Text>();
            let rects = world.read_storage::<Rect>();
            let shapes = world.read_storage::<Shape>();
            let mut nodes = world.write_storage::<Node>();

            let mut click_sound_storage = world.write_resource::<ClickSound>();
            let click_sound: &mut ClickSound = click_sound_storage.deref_mut();
            let settings = world.read_resource::<Settings>();
            if click_sound.play && !settings.mute_sound_effects {
                click_sound.play = false;
                let mut sink = rodio::Sink::new(&audio_endpoint);

                sink.set_volume(settings.sound_volume);
                sink.append(click_sound_source.clone());
                sink.play();
                sink.detach();
            }

            if settings.mute_music && !music.is_paused() {
                music.pause();
            } else if !settings.mute_music && music.is_paused() {
                music.play();
            }

            let mut entities = Vec::new();
            {
                let node = nodes.get_mut(root_node).unwrap();
                node.sort_children(&world, &mut transforms);
                entities.append(&mut node.entities.iter().cloned().collect());
            }

            for entity in &entities {
                render_node(
                    &mut basic,
                    &mut encoder,
                    *entity,
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
                    &mut nodes,
                    &scale_from_base_res,
                );
            }
        }

        // frame_time_text.set_text(format!("dt: {}", math::get_milliseconds(&duration)));
        // basic.render_text(&mut encoder, &frame_time_text, &frame_time_transform, &frame_time_color, &mut glyph_brush);

        encoder.flush(&mut device);

        basic.reset_transform();

        if state_manager.should_render_ui() {
            let mut settings_res = world.write_resource::<Settings>();
            let mut settings = settings_res.deref_mut();
            if let Some(action_name) = state_manager.create_ui_widgets(&mut settings) {
                let mut actions = world.write_resource::<Actions>();
                actions.dispatch(action_name);
            }

            let ui = state_manager.get_ui_to_render();
            let primitives = ui.draw();
            conrod_renderer.fill(
                &mut encoder,
                (dim[0] * hidpi_factor, dim[1] * hidpi_factor),
                primitives,
                &image_map,
            );
            conrod_renderer.draw(&mut factory, &mut encoder, &image_map);

            encoder.flush(&mut device);

            if music.volume() != settings.music_volume {
                music.set_volume(settings.music_volume);
            }
        }

        window.swap_buffers().unwrap();
        device.cleanup();

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
