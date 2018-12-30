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
mod music_manager;
mod renderer;
mod settings;
mod spritesheet;
mod state;
mod storage_types;
mod systems;
mod utils;

use std::collections::HashMap;
use std::ops::DerefMut;
use std::time;

use gfx::Device;
use gfx_glyph::{GlyphBrush, GlyphBrushBuilder};
use glutin::{dpi::LogicalSize, ElementState, Event, GlContext, MouseButton, VirtualKeyCode,
             WindowEvent};
use rodio::Source;
use specs::{Entity, ReadStorage, World, WriteStorage};

use components::ui::{TechTreeButton, TutorialUI, WalletUI};
use components::{upgrade::{LearnProgress, Upgrade},
                 Actions,
                 AnimationSheet,
                 Button,
                 Camera,
                 ClickSound,
                 Color,
                 DeltaTime,
                 EffectedByPollutionTiles,
                 EntityLookup,
                 Error,
                 Fade,
                 FloatingText,
                 Gatherer,
                 HighlightTile,
                 Input,
                 MenuScreen,
                 Node,
                 PowerBar,
                 Pulse,
                 Rect,
                 SelectedTile,
                 Shape,
                 Sprite,
                 StateChange,
                 Text,
                 Texture,
                 Tile,
                 Transform,
                 TransitionToState,
                 TutorialStep};
use renderer::{ColorFormat, DepthFormat};
use settings::Settings;
use spritesheet::Spritesheet;
use state::{menu_state::MenuState, play_state::PlayState, StateManager};
use utils::math;

fn setup_world(world: &mut World, window: &glutin::Window) {
    let dim = renderer::get_dimensions();
    world.add_resource::<Camera>(Camera(renderer::get_ortho(dim[0], dim[1])));
    world.add_resource::<StateChange>(StateChange::new());
    world.add_resource::<Input>(Input::new(
        window.get_hidpi_factor() as f32,
        vec![VirtualKeyCode::Escape],
    ));
    world.add_resource::<ClickSound>(ClickSound { play: false });
    world.add_resource::<DeltaTime>(DeltaTime { dt: 0.0 });
    world.add_resource(Actions::new());
    world.register::<AnimationSheet>();
    world.register::<Button>();
    world.register::<Color>();
    world.register::<EffectedByPollutionTiles>();
    world.register::<Error>();
    world.register::<Fade>();
    world.register::<FloatingText>();
    world.register::<Gatherer>();
    world.register::<HighlightTile>();
    world.register::<LearnProgress>();
    world.register::<MenuScreen>();
    world.register::<Node>();
    world.register::<PowerBar>();
    world.register::<Pulse>();
    world.register::<Rect>();
    world.register::<SelectedTile>();
    world.register::<Shape>();
    world.register::<Sprite>();
    world.register::<TechTreeButton>();
    world.register::<Text>();
    world.register::<Texture>();
    world.register::<Tile>();
    world.register::<Transform>();
    world.register::<TransitionToState>();
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
    texture_map: &HashMap<String, gfx::handle::ShaderResourceView<R, [f32; 4]>>,
    glyph_brush: &mut GlyphBrush<R, F>,
    entity: &Entity,
    sprite_storage: &ReadStorage<Sprite>,
    transform_storage: &mut WriteStorage<Transform>,
    animation_storage: &ReadStorage<AnimationSheet>,
    color_storage: &ReadStorage<Color>,
    text_storage: &mut WriteStorage<Text>,
    rect_storage: &ReadStorage<Rect>,
    shape_storage: &ReadStorage<Shape>,
    texture_storage: &ReadStorage<Texture>,
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

            if let Some(texture) = texture_storage.get(*entity) {
                basic.render_single_texture(
                    encoder,
                    world,
                    factory,
                    &transform,
                    texture_map.get(&texture.name).unwrap(),
                    color_storage.get(*entity).unwrap(),
                )
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
                    basic.render_text(
                        encoder,
                        &text,
                        transform,
                        color,
                        glyph_brush,
                        world.read_resource::<Input>().hidpi_factor,
                        scale_from_base_res,
                    );
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
    texture_map: &HashMap<String, gfx::handle::ShaderResourceView<R, [f32; 4]>>,
    glyph_brush: &mut GlyphBrush<R, F>,
    sprites: &ReadStorage<Sprite>,
    transforms: &mut WriteStorage<Transform>,
    animation_sheets: &ReadStorage<AnimationSheet>,
    colors: &ReadStorage<Color>,
    texts: &mut WriteStorage<Text>,
    rects: &ReadStorage<Rect>,
    shapes: &ReadStorage<Shape>,
    nodes: &mut WriteStorage<Node>,
    textures: &ReadStorage<Texture>,
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
        texture_map,
        glyph_brush,
        &entity,
        sprites,
        transforms,
        animation_sheets,
        colors,
        texts,
        rects,
        shapes,
        textures,
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
            texture_map,
            glyph_brush,
            sprites,
            transforms,
            animation_sheets,
            colors,
            texts,
            rects,
            shapes,
            nodes,
            textures,
            scale_from_base_res,
        );
    }

    if let Some(transform) = transforms.get(entity) {
        basic.transform(&transform, true);
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let dim = renderer::get_dimensions();
    let builder = glutin::WindowBuilder::new()
        .with_title("ld39".to_string())
        .with_dimensions(LogicalSize::new(dim[0] as f64, dim[1] as f64));
    let context = glutin::ContextBuilder::new().with_vsync(true);

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    let hidpi_factor = window.get_hidpi_factor();

    // macos mojave hack
    {
        events_loop.poll_events(|_| {});
        let size = window.get_inner_size().unwrap();
        window.resize(size.to_physical(hidpi_factor));
    }

    let mut world = World::new();

    let target = renderer::WindowTargets {
        color: main_color,
        depth: main_depth,
    };

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut basic = renderer::Basic::new(&mut factory, target);

    let asset_data = loader::read_text_from_file("resources/assets.json").unwrap();
    let spritesheet: Spritesheet = serde_json::from_str(asset_data.as_ref()).unwrap();
    let (asset_texture, _, _) = loader::gfx_load_texture("resources/assets.png", &mut factory);

    let mut screen_sizes = [(0u16, 0u16); 4];
    let screen_textures = [
        loader::gfx_load_texture("resources/startscreens/screenone.png", &mut factory),
        loader::gfx_load_texture("resources/startscreens/screentwo.png", &mut factory),
        loader::gfx_load_texture("resources/startscreens/screenthree.png", &mut factory),
        loader::gfx_load_texture("resources/startscreens/screenfour.png", &mut factory),
    ];

    let mut texture_map = HashMap::new();
    texture_map.insert("screenone.png".to_string(), screen_textures[0].0.clone());
    texture_map.insert("screentwo.png".to_string(), screen_textures[1].0.clone());
    texture_map.insert("screenthree.png".to_string(), screen_textures[2].0.clone());
    texture_map.insert("screenfour.png".to_string(), screen_textures[3].0.clone());

    for (i, (_, w, h)) in screen_textures.iter().enumerate() {
        screen_sizes[i] = (*w, *h);
    }

    let mut glyph_brush =
        GlyphBrushBuilder::using_font_bytes(include_bytes!("../resources/MunroSmall.ttf") as &[u8])
            .build(factory.clone());

    let audio_endpoint = rodio::default_endpoint().unwrap();
    let click_sound_source = loader::create_sound("resources/click.ogg").buffered();
    let settings = loader::load_settings();
    let mut music = music_manager::MusicManager::new(&audio_endpoint, settings.music_volume);

    if !settings.mute_music {
        music.play();
    }

    setup_world(&mut world, &window);

    let mut state_manager = StateManager::new();
    let play_state = PlayState::new();
    state_manager.add_state(PlayState::get_name(), Box::new(play_state));
    state_manager.add_state(
        MenuState::get_name(),
        Box::new(MenuState::new(screen_sizes)),
    );
    state_manager.swap_state(MenuState::get_name(), &mut world);

    let mut running = true;
    let mut frame_start = time::Instant::now();

    let mut conrod_renderer =
        conrod::backend::gfx::Renderer::new(&mut factory, &basic.target.color, hidpi_factor as f64)
            .unwrap();
    let image_map = conrod::image::Map::new();

    {
        if !settings.completed_tutorial {
            {
                let mut actions = world.write_resource::<Actions>();
                actions.dispatch(TutorialStep::SelectTile.as_string(), "".to_string());
            }
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
                let ui = state_manager.get_ui_to_render().unwrap();
                if let Some(event) =
                    conrod::backend::winit::convert_event(event.clone(), window.window())
                {
                    ui.handle_event(event);
                }
            }

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CursorMoved { position: pos, .. } => {
                        let mut input_res = world.write_resource::<Input>();
                        let input = input_res.deref_mut();
                        input.mouse_pos.0 = pos.x as f32 * scale_to_base_res.0;
                        input.mouse_pos.1 = pos.y as f32 * scale_to_base_res.1;
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
                    glutin::WindowEvent::CloseRequested => running = false,
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
                    WindowEvent::HiDpiFactorChanged(factor) => {
                        let mut input_res = world.write_resource::<Input>();
                        input_res.hidpi_factor = factor as f32;
                        let size = window.get_inner_size().unwrap();
                        window.resize(size.to_physical(factor as f64));

                        conrod_renderer.on_resize(basic.target.color.clone());

                        let target = &mut basic.target;
                        gfx_window_glutin::update_views(
                            &window,
                            &mut target.color,
                            &mut target.depth,
                        );
                    }
                    WindowEvent::Resized(size) => {
                        let input = world.read_resource::<Input>();
                        let hidpi_factor = input.hidpi_factor;
                        window.resize(size.to_physical(hidpi_factor as f64));

                        conrod_renderer.on_resize(basic.target.color.clone());

                        let target = &mut basic.target;
                        gfx_window_glutin::update_views(
                            &window,
                            &mut target.color,
                            &mut target.depth,
                        );

                        let w = size.width as f32;
                        let h = size.height as f32;
                        scale_from_base_res = (w / dim[0], h / dim[1]);
                        scale_to_base_res = (dim[0] / w, dim[1] / h);
                    }
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
            let textures = world.read_storage::<Texture>();

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
                    &texture_map,
                    &mut glyph_brush,
                    &sprites,
                    &mut transforms,
                    &animation_sheets,
                    &colors,
                    &mut texts,
                    &rects,
                    &shapes,
                    &mut nodes,
                    &textures,
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
                actions.dispatch(action_name, String::new());
            }

            let ui = state_manager.get_ui_to_render().unwrap();
            let primitives = ui.draw();
            conrod_renderer.clear(
                &mut encoder,
                [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0],
            );
            conrod_renderer.fill(
                &mut encoder,
                (
                    dim[0] * scale_from_base_res.0 * hidpi_factor as f32,
                    dim[1] * scale_from_base_res.1 * hidpi_factor as f32,
                ),
                hidpi_factor,
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

        if state_change.state == PlayState::get_name() && (state_change.action == "restart" || state_change.action == "start") {
            music_manager.play_random_game_track();
        }

        state_manager.process_state_change(&mut state_change, &mut world);
    }

    music.stop();
}
