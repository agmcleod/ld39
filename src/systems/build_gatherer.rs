use components::ui::WalletUI;
use components::{ui::TutorialUI, upgrade::Buff, AnimationSheet, Button, ClickSound, Color,
                 EntityLookup, Gatherer, GathererPositions, GathererType, Input, Node,
                 ResearchedBuffs, SelectedTile, Text, Tile, TileNodes, TileType, Transform,
                 TutorialStep, Wallet};
use entities::tutorial;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use systems::logic;

pub struct BuildGatherer;

impl<'a> System<'a> for BuildGatherer {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AnimationSheet>,
        WriteStorage<'a, Button>,
        Write<'a, ClickSound>,
        WriteStorage<'a, Color>,
        Read<'a, EntityLookup>,
        WriteStorage<'a, Gatherer>,
        Write<'a, GathererPositions>,
        Read<'a, Input>,
        WriteStorage<'a, Node>,
        Read<'a, TileNodes>,
        Read<'a, ResearchedBuffs>,
        ReadStorage<'a, SelectedTile>,
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
            mut animation_sheet_storage,
            mut button_storage,
            mut click_sound_storage,
            mut color_storage,
            entity_lookup_storage,
            mut gatherer_storage,
            mut gatherer_positions_storage,
            input_storage,
            mut nodes_storage,
            tile_nodes_storage,
            researched_buffs_storage,
            selected_tile_storage,
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

        let mut button_pressed = false;
        let mut gatherer_type = None;
        for button in (&mut button_storage).join() {
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
        let mut selected_tile_x = 0.0;
        let mut selected_tile_y = 0.0;
        // spend the money, and hide selected tile
        if button_pressed {
            let mut amount = gatherer_type.unwrap().clone().get_build_cost();
            if gatherer_type.unwrap() == GathererType::Solar
                && researched_buffs.0.contains(&Buff::PurchaseSolarCellCompany)
            {
                amount -= amount * 20 / 100;
            }
            for (_, transform) in (&selected_tile_storage, &mut transform_storage).join() {
                if transform.visible && wallet.spend(amount) {
                    transform.visible = false;
                    create = true;

                    selected_tile_x = transform.get_pos().x;
                    selected_tile_y = transform.get_pos().y;
                    logic::update_text(
                        format!("{}", wallet.money),
                        &mut text_storage,
                        &wallet_ui_storage,
                    );
                }
            }
        }

        if create {
            tutorial::clear_ui(
                &entities,
                &tutorial_step_storage,
                &tutorial_ui_storage,
                TutorialStep::BuildCoal(0.0, 0.0),
            );
            tutorial::clear_ui(
                &entities,
                &tutorial_step_storage,
                &tutorial_ui_storage,
                TutorialStep::ResourcesSold,
            );
            // create gatherer
            let tile_nodes = tile_nodes_storage.deref();
            let selected_tile_col = (selected_tile_x / Tile::get_size()) as i32;
            let selected_tile_row = (selected_tile_y / Tile::get_size()) as i32;

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

                                let pollution_entity = entities.create();
                                transform_storage
                                    .insert(
                                        pollution_entity,
                                        Transform::visible(
                                            selected_tile_x + (i as f32) * Tile::get_size(),
                                            selected_tile_y + (j as f32) * Tile::get_size(),
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
                                    [1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2]
                                        .iter()
                                        .map(|n| format!("pollution_{}.png", n))
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
            let mut anim = AnimationSheet::new(0.5);
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
