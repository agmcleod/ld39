use components::{ui::TutorialUI, Actions, Button, Color, EntityLookup, Gatherer, Input, Node,
                 Rect, ResearchedBuffs, SelectedTile, Sprite, Text, Tile, Transform, TutorialStep};
use entities::{create_build_ui, recursive_delete, tutorial};
use specs::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::Deref;
use systems::logic;

pub struct TileSelection {
    build_ui_entity: Option<Entity>,
}

impl TileSelection {
    pub fn new() -> TileSelection {
        TileSelection {
            build_ui_entity: None,
        }
    }
}

impl<'a> System<'a> for TileSelection {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        WriteStorage<'a, Button>,
        WriteStorage<'a, Color>,
        Read<'a, EntityLookup>,
        ReadStorage<'a, Gatherer>,
        Read<'a, Input>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Rect>,
        Read<'a, ResearchedBuffs>,
        ReadStorage<'a, SelectedTile>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Text>,
        ReadStorage<'a, Tile>,
        WriteStorage<'a, Transform>,
        Write<'a, TutorialStep>,
        ReadStorage<'a, TutorialUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut actions_storage,
            mut button_storage,
            mut color_storage,
            entity_lookup_storage,
            gatherer_storage,
            input_storage,
            mut node_storage,
            mut rect_storage,
            researched_buffs,
            selected_tile_storage,
            mut sprite_storage,
            mut text_storage,
            tile_storage,
            mut transform_storage,
            mut tutorial_step_storage,
            tutorial_ui_storage,
        ) = data;

        let input: &Input = input_storage.deref();
        let mut tile_mouse_x = 0.0;
        let mut tile_mouse_y = 0.0;
        let mut clicked = false;
        let researched_buffs: &ResearchedBuffs = researched_buffs.deref();

        let mut tile_type_selected = None;

        for (tile, button, transform) in
            (&tile_storage, &mut button_storage, &transform_storage).join()
        {
            if button.clicked(&input) {
                tile_mouse_x = transform.get_pos().x;
                tile_mouse_y = transform.get_pos().y;
                clicked = true;
                tile_type_selected = Some(tile.tile_type.clone());
            }
        }

        if clicked {
            let mut tile_already_taken = false;
            // check if tile already selected
            for (_, transform) in (&gatherer_storage, &mut transform_storage).join() {
                if transform.get_pos().x == tile_mouse_x && transform.get_pos().y == tile_mouse_y {
                    tile_already_taken = true;
                    break;
                }
            }

            if !tile_already_taken {
                for (_, transform) in (&selected_tile_storage, &mut transform_storage).join() {
                    transform.visible = true;
                    transform.set_pos2(tile_mouse_x, tile_mouse_y);
                }

                // if build UI showing, clean it up, as tile type may be different
                if let Some(build_ui_entity) = self.build_ui_entity {
                    recursive_delete(&entities, &node_storage, &build_ui_entity);
                    self.build_ui_entity = None;
                }
                // create build ui
                let entity = create_build_ui::create(
                    tile_mouse_x + Tile::get_size(),
                    tile_mouse_y,
                    &tile_type_selected.unwrap(),
                    &entities,
                    &mut button_storage,
                    &mut color_storage,
                    &mut node_storage,
                    &mut rect_storage,
                    &mut sprite_storage,
                    &mut text_storage,
                    &mut transform_storage,
                    &researched_buffs,
                );
                self.build_ui_entity = Some(entity);

                let lookup = entity_lookup_storage.deref();

                {
                    let node = logic::get_root(lookup, &mut node_storage);
                    node.add(entity);
                }

                let build_entity_pos = transform_storage.get(entity).unwrap().get_pos();

                let changed = tutorial::next_step(
                    &entities,
                    &mut actions_storage,
                    &mut tutorial_step_storage,
                    &tutorial_ui_storage,
                    &node_storage,
                    TutorialStep::SelectTile,
                    TutorialStep::BuildCoal(build_entity_pos.x, build_entity_pos.y),
                );

                if !changed {
                    // could maybe do this by checking current state here.
                    // Passing array of possible current states not super comparable :(
                    tutorial::next_step(
                        &entities,
                        &mut actions_storage,
                        &mut tutorial_step_storage,
                        &tutorial_ui_storage,
                        &node_storage,
                        TutorialStep::BuildCoal(10.0, 10.0),
                        TutorialStep::BuildCoal(build_entity_pos.x, build_entity_pos.y),
                    );
                }
            }
        } else {
            for (_, transform) in (&selected_tile_storage, &transform_storage).join() {
                // if selected tile as hidden, clear out build entity
                if !transform.visible {
                    if let Some(build_ui_entity) = self.build_ui_entity {
                        recursive_delete(&entities, &node_storage, &build_ui_entity);
                        self.build_ui_entity = None;
                    }
                }
            }
        }
    }
}
