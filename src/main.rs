#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate specs;
extern crate cgmath;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod components;
mod loader;
mod renderer;
mod spritesheet;
mod systems;
mod utils;

use std::ops::{DerefMut};

use components::{AnimationSheet, Camera, Color, CurrentPower, Input, PowerBar, Sprite, Tile, Transform};
use specs::{DispatcherBuilder, Join, World};
use renderer::{ColorFormat, DepthFormat};
use spritesheet::Spritesheet;
use glutin::{Event, ElementState, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::GlContext;
use gfx::Device;

fn setup_world(world: &mut World, window: &glutin::Window) {
    world.add_resource::<Camera>(Camera(renderer::get_ortho()));
    world.add_resource::<Input>(Input::new(window.hidpi_factor(), vec![VirtualKeyCode::W, VirtualKeyCode::A, VirtualKeyCode::S, VirtualKeyCode::D]));
    world.register::<PowerBar>();
    world.register::<CurrentPower>();
    world.register::<Color>();
    world.register::<AnimationSheet>();
    world.register::<Sprite>();
    world.register::<Tile>();
    world.register::<Transform>();
    world.create_entity()
        .with(PowerBar::new())
        .with(Transform::new(670, 576, 260, 32, 0.0, 1.0, 1.0))
        .with(Sprite{ frame_name: "powerbar.png".to_string(), visible: true });

    world.create_entity()
        .with(CurrentPower{})
        .with(Transform::new(674, 580, CurrentPower::get_max_with(), 24, 0.0, 1.0, 1.0))
        .with(Color([0.0, 1.0, 0.0, 1.0]));
    for row in 0i32..10i32 {
        for col in 0i32..10i32 {
            world.create_entity()
                .with(Transform::new(64 * col, 64 * row, 64, 64, 0.0, 1.0, 1.0))
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
    setup_world(&mut world, &window);

    let mut dispatcher = DispatcherBuilder::new()
        .add(systems::AnimationSystem::new(), "animation_system", &[])
        .add(systems::PowerUsage::new(), "power_system", &[])
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
        let transforms = world.read::<Transform>();
        let animation_sheets = world.read::<AnimationSheet>();
        let colors = world.read::<Color>();

        for (sprite, transform) in (&sprites, &transforms).join() {
            if sprite.visible {
                basic.render(&mut encoder, &world, &mut factory, &transform, Some(&sprite.frame_name), &spritesheet, None, &asset_texture);
            }
        }

        for (animation_sheet, transform) in (&animation_sheets, &transforms).join() {
            basic.render(&mut encoder, &world, &mut factory, &transform, Some(animation_sheet.get_current_frame()), &spritesheet, None, &asset_texture);
        }

        for (color, transform) in (&colors, &transforms).join() {
            basic.render(&mut encoder, &world, &mut factory, &transform, None, &spritesheet, Some(color.0), &asset_texture);
        }

        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
