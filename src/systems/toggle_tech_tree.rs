use components::{Button, Color, EntityLookup, Input, Node, Rect, StateChange, Tile, Transform};
use entities::create_colored_rect;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use state::play_state::PlayState;
use std::ops::{Deref, DerefMut};
use systems::logic;

pub struct ToggleTechTree;

impl ToggleTechTree {
    pub fn new() -> ToggleTechTree {
        ToggleTechTree {}
    }

    fn check_show_tech_button(
        &mut self,
        lookup: &mut EntityLookup,
        input: &Input,
        entities: &Entities,
        button_storage: &mut WriteStorage<Button>,
        color_storage: &mut WriteStorage<Color>,
        node_storage: &mut WriteStorage<Node>,
        rect_storage: &mut WriteStorage<Rect>,
        transform_storage: &mut WriteStorage<Transform>,
        state_change_res: &mut Write<StateChange>,
        tile_storage: &ReadStorage<Tile>,
    ) {
        let mut was_clicked = false;
        {
            let button = button_storage
                .get_mut(*lookup.get("show_button_entity").unwrap())
                .unwrap();
            if button.clicked(&input) {
                {
                    let transform = transform_storage
                        .get_mut(*lookup.get("tech_tree_container").unwrap())
                        .unwrap();
                    transform.visible = true;
                }

                {
                    let transform = transform_storage
                        .get_mut(*lookup.get("side_bar_container").unwrap())
                        .unwrap();
                    transform.visible = false;
                }

                let state_change: &mut StateChange = state_change_res.deref_mut();
                state_change.set(PlayState::get_name(), "tech_tree_pause".to_string());

                let rect = create_colored_rect::create(
                    0.0,
                    0.0,
                    10.0,
                    640,
                    640,
                    [0.0, 0.0, 0.0, 0.8],
                    entities,
                    transform_storage,
                    color_storage,
                    rect_storage,
                );
                lookup.entities.insert("pause_black".to_string(), rect);
                let node = logic::get_root(&lookup, node_storage);
                node.add(rect);

                was_clicked = true;
            }
        }

        if was_clicked {
            for (_, button) in (tile_storage, button_storage).join() {
                button.set_disabled(true);
            }
        }
    }

    fn check_resume_from_upgrades_button(
        &mut self,
        lookup: &mut EntityLookup,
        input: &Input,
        entities: &Entities,
        button_storage: &mut WriteStorage<Button>,
        transform_storage: &mut WriteStorage<Transform>,
        state_change_res: &mut Write<StateChange>,
        tile_storage: &ReadStorage<Tile>,
    ) {
        let mut was_clicked = false;
        {
            let button = button_storage
                .get_mut(*lookup.get("resume_from_upgrades").unwrap())
                .unwrap();
            if button.clicked(&input) {
                {
                    let transform = transform_storage
                        .get_mut(*lookup.get("tech_tree_container").unwrap())
                        .unwrap();
                    transform.visible = false;
                }

                {
                    let transform = transform_storage
                        .get_mut(*lookup.get("side_bar_container").unwrap())
                        .unwrap();
                    transform.visible = true;
                }

                let state_change: &mut StateChange = state_change_res.deref_mut();
                state_change.set(PlayState::get_name(), "resume".to_string());
                let overlay_entity = *lookup.get("pause_black").unwrap();
                entities.delete(overlay_entity).unwrap();
                lookup.entities.remove("pause_black");
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
        Write<'a, EntityLookup>,
        Read<'a, Input>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Rect>,
        Write<'a, StateChange>,
        ReadStorage<'a, Tile>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut button_storage,
            mut color_storage,
            mut lookup,
            input,
            mut node_storage,
            mut rect_storage,
            mut state_change_res,
            tile_storage,
            mut transform_storage,
        ) = data;

        let mut lookup: &mut EntityLookup = lookup.deref_mut();
        let input: &Input = input.deref();
        self.check_show_tech_button(
            &mut lookup,
            &input,
            &entities,
            &mut button_storage,
            &mut color_storage,
            &mut node_storage,
            &mut rect_storage,
            &mut transform_storage,
            &mut state_change_res,
            &tile_storage,
        );
        self.check_resume_from_upgrades_button(
            &mut lookup,
            &input,
            &entities,
            &mut button_storage,
            &mut transform_storage,
            &mut state_change_res,
            &tile_storage,
        );
    }
}
