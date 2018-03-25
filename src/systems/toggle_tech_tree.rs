use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use specs::{Entities, System, Join, Fetch, FetchMut, ReadStorage, WriteStorage};
use components::{Button, Color, EntityLookup, Input, Rect, StateChange, Tile, Transform};
use state::play_state::PlayState;
use entities::create_colored_rect;
use scene::Node;

pub struct ToggleTechTree {
    scene: Arc<Mutex<Node>>,
}

impl ToggleTechTree {
    pub fn new(scene: Arc<Mutex<Node>>) -> ToggleTechTree {
        ToggleTechTree{
            scene: scene,
        }
    }

    fn check_show_tech_button(&mut self, lookup: &mut EntityLookup, input: &Input, entities: &Entities, button_storage: &mut WriteStorage<Button>, color_storage: &mut WriteStorage<Color>, rect_storage: &mut WriteStorage<Rect>, transform_storage: &mut WriteStorage<Transform>, state_change_res: &mut FetchMut<StateChange>, tile_storage: &ReadStorage<Tile>) {
        let mut was_clicked = false;
        {
            let button = button_storage.get_mut(*lookup.get("show_button_entity").unwrap()).unwrap();
            if button.clicked(&input) {
                {
                    let transform = transform_storage.get_mut(*lookup.get("tech_tree_container").unwrap()).unwrap();
                    transform.visible = true;
                }

                {
                    let transform = transform_storage.get_mut(*lookup.get("side_bar_container").unwrap()).unwrap();
                    transform.visible = false;
                }

                let state_change: &mut StateChange = state_change_res.deref_mut();
                state_change.set(PlayState::get_name(), "tech_tree_pause".to_string());

                let node = create_colored_rect::create(0.0, 0.0, 10.0, 640, 640, [0.0, 0.0, 0.0, 0.8], entities, transform_storage, color_storage, rect_storage);
                lookup.entities.insert("pause_black".to_string(), node.entity.unwrap());
                let mut scene = self.scene.lock().unwrap();
                scene.sub_nodes.push(node);

                was_clicked = true;
            }
        }

        if was_clicked {
            for (_, button) in (tile_storage, button_storage).join() {
                button.set_disabled(true);
            }
        }
    }

    fn check_resume_from_upgrades_button(&mut self, lookup: &mut EntityLookup, input: &Input, entities: &Entities, button_storage: &mut WriteStorage<Button>, transform_storage: &mut WriteStorage<Transform>, state_change_res: &mut FetchMut<StateChange>, tile_storage: &ReadStorage<Tile>) {
        let mut was_clicked = false;
        {
            let button = button_storage.get_mut(*lookup.get("resume_from_upgrades").unwrap()).unwrap();
            if button.clicked(&input) {
                {
                    let transform = transform_storage.get_mut(*lookup.get("tech_tree_container").unwrap()).unwrap();
                    transform.visible = false;
                }

                {
                    let transform = transform_storage.get_mut(*lookup.get("side_bar_container").unwrap()).unwrap();
                    transform.visible = true;
                }

                let state_change: &mut StateChange = state_change_res.deref_mut();
                state_change.set(PlayState::get_name(), "tech_tree_resume".to_string());
                let overlay_entity = *lookup.get(&"pause_black".to_string()).unwrap();
                entities.delete(overlay_entity);
                let mut scene = self.scene.lock().unwrap();
                scene.remove_node_with_entity(overlay_entity);
                lookup.entities.remove(&"pause_black".to_string());
                was_clicked = true;
            }
        }

        if was_clicked {
            for (_, button) in (tile_storage, button_storage).join() {
                button.set_disabled(false);
            }
        }
    }
}

impl<'a> System<'a> for ToggleTechTree {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Button>,
        WriteStorage<'a, Color>,
        FetchMut<'a, EntityLookup>,
        Fetch<'a, Input>,
        WriteStorage<'a, Rect>,
        FetchMut<'a, StateChange>,
        ReadStorage<'a, Tile>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut button_storage, mut color_storage, mut lookup, input, mut rect_storage, mut state_change_res, tile_storage, mut transform_storage) = data;

        let mut lookup: &mut EntityLookup = lookup.deref_mut();
        let input: &Input = input.deref();
        self.check_show_tech_button(&mut lookup, &input, &entities, &mut button_storage, &mut color_storage, &mut rect_storage, &mut transform_storage, &mut state_change_res, &tile_storage);
        self.check_resume_from_upgrades_button(&mut lookup, &input, &entities, &mut button_storage, &mut transform_storage, &mut state_change_res, &tile_storage);
    }
}
