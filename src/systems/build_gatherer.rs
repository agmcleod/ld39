use components::ui::WalletUI;
use components::{ui::TutorialUI, upgrade::Buff, Actions, AnimationSheet, Button, ClickSound,
                 Color, EffectedByPollutionTiles, EntityLookup, Gatherer, GathererPositions,
                 GathererType, Input, Node, PollutedTiles, ResearchedBuffs, SelectedTile, Sprite, Text, Tile,
                 TileNodes, TileType, Transform, TutorialStep, Wallet};
use entities::tutorial;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use systems::logic;

pub struct BuildGatherer;

impl BuildGatherer {
    fn remove_effected_by_pollution_tiles_entities(
        &self,
        entities: &Entities,
        effected_by_pollution_tiles: &mut EffectedByPollutionTiles,
    ) {
        for entity in &effected_by_pollution_tiles.tiles {
            entities.delete(*entity).unwrap();
        }
        effected_by_pollution_tiles.clear();
    }
}

impl<'a> System<'a> for BuildGatherer {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        WriteStorage<'a, AnimationSheet>,
        WriteStorage<'a, Button>,
        Write<'a, ClickSound>,
        WriteStorage<'a, Color>,
        WriteStorage<'a, EffectedByPollutionTiles>,
        Read<'a, EntityLookup>,
        WriteStorage<'a, Gatherer>,
        Write<'a, GathererPositions>,
        Read<'a, Input>,
        WriteStorage<'a, Node>,
        Write<'a, PollutedTiles>,
        Read<'a, TileNodes>,
        Read<'a, ResearchedBuffs>,
        ReadStorage<'a, SelectedTile>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
        Write<'a, TutorialStep>,
        ReadStorage<'a, TutorialUI>,
        Write<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut actions_storage,
            mut animation_sheet_storage,
            mut button_storage,
            mut click_sound_storage,
            mut color_storage,
            mut effected_by_pollution_tiles_storage,
            entity_lookup_storage,
            mut gatherer_storage,
            mut gatherer_positions_storage,
            input_storage,
            mut nodes_storage,
            mut polluted_tiles_storage,
            tile_nodes_storage,
            researched_buffs_storage,
            selected_tile_storage,
            mut sprite_storage,
            mut text_storage,
            mut transform_storage,
            tutorial_step_storage,
            tutorial_ui_storage,
            mut wallet_storage,
            wallet_ui_storage,
        ) = data;

        let input: &Input = input_storage.deref();
        let click_sound: &mut ClickSound = click_sound_storage.deref_mut();
        let wallet: &mut Wallet = wallet_storage.deref_mut();
        let lookup = entity_lookup_storage.deref();

        let mut selected_tile_x = 0.0;
        let mut selected_tile_y = 0.0;

        for (_, transform) in (&selected_tile_storage, &mut transform_storage).join() {
            if transform.visible {
                selected_tile_x = transform.get_pos().x;
                selected_tile_y = transform.get_pos().y;
            }
        }

        let selected_tile_col = (selected_tile_x / Tile::get_size()) as i32;
        let selected_tile_row = (selected_tile_y / Tile::get_size()) as i32;

        let tile_nodes = tile_nodes_storage.deref();

        let mut button_pressed = false;
        let mut gatherer_type = None;
        for (button, effected_by_pollution_tiles) in (
            &mut button_storage,
            &mut effected_by_pollution_tiles_storage,
        ).join()
        {
            if button.name != "build_solar" {
                if button.mouse_is_over && !effected_by_pollution_tiles.has_entities() {
                    for i in -1..2 {
                        for j in -1..2 {
                            if button.name == "build_hydro" && (i != 0 || j != 0) {
                                continue;
                            } else if button.name != "build_hydro" && i == 0 && j == 0 {
                                continue;
                            }

                            if let Some(&(tile_type, _)) = tile_nodes
                                .nodes
                                .get(&(selected_tile_col + i, selected_tile_row + j))
                            {
                                if tile_type != TileType::Open {
                                    let entity = entities.create();
                                    transform_storage
                                        .insert(
                                            entity,
                                            Transform::visible(
                                                selected_tile_x + (i as f32) * Tile::get_size(),
                                                selected_tile_y + (j as f32) * Tile::get_size(),
                                                4.0,
                                                64,
                                                64,
                                                0.0,
                                                1.0,
                                                1.0,
                                            ),
                                        )
                                        .unwrap();
                                    sprite_storage
                                        .insert(
                                            entity,
                                            Sprite {
                                                frame_name: "pollution_warning.png".to_string(),
                                            },
                                        )
                                        .unwrap();

                                    effected_by_pollution_tiles.tiles.push(entity.clone());

                                    let node = logic::get_root(&lookup, &mut nodes_storage);
                                    node.add(entity);
                                }
                            }
                        }
                    }
                } else if !button.mouse_is_over && effected_by_pollution_tiles.has_entities() {
                    self.remove_effected_by_pollution_tiles_entities(
                        &entities,
                        effected_by_pollution_tiles,
                    );
                }
            }

            if button.name == "build_coal" && button.clicked(&input) {
                button_pressed = true;
                gatherer_type = Some(GathererType::Coal);
            } else if button.name == "build_oil" && button.clicked(&input) {
                button_pressed = true;
                gatherer_type = Some(GathererType::Oil);
            } else if button.name == "build_solar" && button.clicked(&input) {
                button_pressed = true;
                gatherer_type = Some(GathererType::Solar);
            } else if button.name == "build_hydro" && button.clicked(&input) {
                button_pressed = true;
                gatherer_type = Some(GathererType::Hydro);
            }

            if button_pressed {
                click_sound.play = true;
            }
        }

        let researched_buffs = researched_buffs_storage.deref();

        let mut create = false;
        // spend the money, and hide selected tile
        if button_pressed {
            let mut amount = gatherer_type.unwrap().clone().get_build_cost();
            if gatherer_type.unwrap() == GathererType::Solar
                && researched_buffs
                    .0
                    .contains_key(&Buff::PurchaseSolarCellCompany)
            {
                amount -= amount * 20 / 100;
            }
            for (_, transform) in (&selected_tile_storage, &mut transform_storage).join() {
                if transform.visible {
                    if wallet.spend(amount) {
                        transform.visible = false;
                        create = true;

                        selected_tile_x = transform.get_pos().x;
                        selected_tile_y = transform.get_pos().y;
                        logic::update_text(
                            format!("Wallet: ${}", wallet.get_money()),
                            &mut text_storage,
                            &wallet_ui_storage,
                        );
                    } else {
                        actions_storage.dispatch(
                            "display_error".to_string(),
                            "Not enough money to build".to_string(),
                        );
                    }
                }
            }
        }

        if create {
            tutorial::clear_ui(
                &entities,
                &tutorial_step_storage,
                &tutorial_ui_storage,
                &nodes_storage,
                TutorialStep::BuildCoal(0.0, 0.0),
            );
            tutorial::clear_ui(
                &entities,
                &tutorial_step_storage,
                &tutorial_ui_storage,
                &nodes_storage,
                TutorialStep::ResourcesSold,
            );

            for effected_by_pollution_tiles in (&mut effected_by_pollution_tiles_storage).join() {
                self.remove_effected_by_pollution_tiles_entities(
                    &entities,
                    effected_by_pollution_tiles,
                );
            }

            // create gatherer
            let gatherer_type = gatherer_type.unwrap();

            let mut pollution = 0i32;

            // calculate pollution, and add pollution sprites on top
            // this will at present overlap polluting animations
            // Solar doesn't pollute
            if gatherer_type != GathererType::Solar {
                for i in -1..2 {
                    for j in -1..2 {
                        if gatherer_type == GathererType::Hydro && (i != 0 || j != 0) {
                            continue;
                        } else if gatherer_type != GathererType::Hydro && i == 0 && j == 0 {
                            continue;
                        }
                        if let Some(&(tile_type, _)) = tile_nodes
                            .nodes
                            .get(&(selected_tile_col + i, selected_tile_row + j))
                        {
                            // if its a non open tile
                            if tile_type != TileType::Open {
                                pollution += gatherer_type.get_pollution_amount();

                                let selected_tile_x = selected_tile_x + (i as f32) * Tile::get_size();
                                let selected_tile_y = selected_tile_y + (j as f32) * Tile::get_size();

                                if polluted_tiles_storage.contains(&(selected_tile_x as i32, selected_tile_y as i32)) {
                                    continue
                                }

                                polluted_tiles_storage.insert((selected_tile_x as i32, selected_tile_y as i32));

                                let pollution_entity = entities.create();
                                transform_storage
                                    .insert(
                                        pollution_entity,
                                        Transform::visible(
                                            selected_tile_x,
                                            selected_tile_y,
                                            3.0,
                                            64,
                                            64,
                                            0.0,
                                            1.0,
                                            1.0,
                                        ),
                                    )
                                    .unwrap();
                                let mut animation = AnimationSheet::new(0.1);
                                animation.add_animation(
                                    "default".to_string(),
                                    ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12"]
                                        .iter()
                                        .map(|n| format!("pollution_grey_{}.png", n))
                                        .collect(),
                                );
                                animation.set_current_animation("default".to_string());
                                animation_sheet_storage
                                    .insert(pollution_entity, animation)
                                    .unwrap();
                                color_storage
                                    .insert(pollution_entity, Color([1.0, 1.0, 1.0, 1.0]))
                                    .unwrap();

                                let node = logic::get_root(&lookup, &mut nodes_storage);
                                node.add(pollution_entity);
                            }
                        }
                    }
                }
            }

            let gatherer = Gatherer::new(gatherer_type, pollution);
            let mut anim = AnimationSheet::new(0.35);
            anim.add_animation("default".to_string(), gatherer.gatherer_type.get_frames());
            anim.set_current_animation("default".to_string());
            let gatherer_entity = entities.create();
            gatherer_storage.insert(gatherer_entity, gatherer).unwrap();
            animation_sheet_storage
                .insert(gatherer_entity, anim)
                .unwrap();
            transform_storage
                .insert(
                    gatherer_entity,
                    Transform::visible(
                        selected_tile_x,
                        selected_tile_y,
                        2.0,
                        64,
                        64,
                        0.0,
                        1.0,
                        1.0,
                    ),
                )
                .unwrap();
            nodes_storage.insert(gatherer_entity, Node::new()).unwrap();

            let gatherer_positions = gatherer_positions_storage.deref_mut();
            gatherer_positions.gatherers.insert(
                (selected_tile_col, selected_tile_row),
                (gatherer_type.clone(), gatherer_entity.clone()),
            );

            // check for adjacent gatherers
            let mut at_least_one_adjacent = false;
            for i in -1..2 {
                for j in -1..2 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if let Some(&(other_gatherer_type, entity)) = gatherer_positions
                        .gatherers
                        .get(&(selected_tile_col + i, selected_tile_row + j))
                    {
                        if gatherer_type == other_gatherer_type {
                            gatherer_storage
                                .get_mut(entity)
                                .unwrap()
                                .has_adjancent_of_same_type = true;
                            at_least_one_adjacent = true;
                        }
                    }
                }
            }

            if at_least_one_adjacent {
                gatherer_storage
                    .get_mut(gatherer_entity)
                    .unwrap()
                    .has_adjancent_of_same_type = true;
            }

            let node = logic::get_root(&lookup, &mut nodes_storage);
            node.add(gatherer_entity);
        }
    }
}
