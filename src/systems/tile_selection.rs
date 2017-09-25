use std::ops::Deref;
use std::sync::{Arc, Mutex};
use specs::{Entity, Entities, ReadStorage, WriteStorage, Fetch, Join, System};
use components::{Button, Color, Gatherer, Input, Rect, Resources, SelectedTile, Sprite, Tile, Transform};
use scene::Scene;
use entities::build_ui;

pub struct TileSelection {
    pub scene: Arc<Mutex<Scene>>,
    pub build_ui_entity: Option<Entity>,
    pub mouse_pressed: bool,
}

impl TileSelection {
    pub fn new(scene: Arc<Mutex<Scene>>) -> TileSelection {
        TileSelection{
            scene: scene,
            build_ui_entity: None,
            mouse_pressed: false,
        }
    }
}

impl<'a> System<'a> for TileSelection {
    type SystemData = (
        WriteStorage<'a, Button>,
        WriteStorage<'a, Color>,
        Entities<'a>,
        ReadStorage<'a, Gatherer>,
        Fetch<'a, Input>,
        WriteStorage<'a, Rect>,
        Fetch<'a, Resources>,
        ReadStorage<'a, SelectedTile>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut button_storage, mut color_storage, entities, gatherer_storage, input_storage, mut rect_storage, resource_storage, selected_tile_storage, mut sprite_storage, mut transform_storage) = data;

        let input: &Input = input_storage.deref();
        let mouse_x = input.mouse_pos.0;
        let mouse_y = 640.0 - input.mouse_pos.1;
        let within_grid = mouse_x >= 0.0 && mouse_x <= 640.0 && mouse_y >= 0.0 && mouse_y <= 640.0;
        let tile_size = Tile::get_size();
        let tile_mouse_x = (mouse_x / tile_size).floor() * tile_size;
        let tile_mouse_y = (mouse_y / tile_size).floor() * tile_size;

        if input.mouse_pressed && within_grid && !self.mouse_pressed {
            self.mouse_pressed = true;
        } else if self.mouse_pressed && !input.mouse_pressed && within_grid {
            self.mouse_pressed = false;

            let mut collisions = false;
            for (_, transform) in (&gatherer_storage, &mut transform_storage).join() {
                if transform.pos.x == tile_mouse_x && transform.pos.y == tile_mouse_y {
                    collisions = true;
                    break
                }
            }

            if !collisions {
                for (selected_tile, rect, transform) in (&selected_tile_storage, &mut rect_storage, &mut transform_storage).join() {
                    rect.visible = true;
                    transform.pos.x = tile_mouse_x;
                    transform.pos.y = tile_mouse_y;
                }

                let mut scene = self.scene.lock().unwrap();

                if let Some(build_ui_entity) = self.build_ui_entity {
                    let mut transform = transform_storage.get_mut(build_ui_entity).unwrap();
                    transform.pos.x = tile_mouse_x + Tile::get_size();
                    transform.pos.y = tile_mouse_y;
                } else {
                    let resources: &Resources = resource_storage.deref();
                    let node = build_ui::create(tile_mouse_x + Tile::get_size(), tile_mouse_y, &entities, &mut button_storage, &mut color_storage, &mut rect_storage, &mut sprite_storage, &mut transform_storage, &resources.get_current_type());
                    self.build_ui_entity = Some(node.entity.unwrap().clone());
                    scene.nodes.push(node);
                }
            }
        } else if within_grid {
            for (selected_tile, rect) in (&selected_tile_storage, &rect_storage).join() {
                // clean up build UI if selected tile not visible, may want to add a flag for checking this
                // cou;ld maybe move build_ui_entity into the selected_tile component?
                if !rect.visible {
                    if let Some(build_ui_entity) = self.build_ui_entity {
                        let mut scene = self.scene.lock().unwrap();

                        let mut node_to_delete = -1i32;
                        for (i, node) in scene.nodes.iter().enumerate() {
                            if let Some(entity) = node.entity {
                                if entity == build_ui_entity {
                                    node_to_delete = i as i32;
                                }
                            }
                        }

                        if node_to_delete > -1i32 {
                            scene.nodes.remove(node_to_delete as usize);
                            entities.delete(build_ui_entity);
                            self.build_ui_entity = None;
                        }
                    }
                }
            }
        }

        if !input.mouse_pressed && self.mouse_pressed {
            self.mouse_pressed = false;
        }
    }
}

