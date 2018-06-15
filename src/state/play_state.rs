use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};
use specs::{Dispatcher, DispatcherBuilder, Entity, World};
use scene::Node;
use state::State;

use components::{Button, CityPowerState, Color, EntityLookup, GathererPositions, GatheringRate,
                 PowerBar, ProtectedNodes, Rect, ResearchedBuffs, ResearchingEntities,
                 ResourceCount, ResourceType, Resources, SelectedTile, Sprite, Text, Tile,
                 TileType, Transform, Wallet, ui::PollutionCount, CITY_POWER_STATE_COORDS};
use components::ui::WalletUI;
use systems;
use renderer;
use entities::{create_power_bar, create_text, tech_tree};
use storage_types::*;
use rand::{thread_rng, Rng};

enum InternalState {
    Game,
    TechTree,
    Pause,
}

pub struct PlayState<'a> {
    dispatcher: Dispatcher<'a, 'a>,
    tech_tree_dispatcher: Dispatcher<'a, 'a>,
    scene: Arc<Mutex<Node>>,
    state: InternalState,
}

impl<'a> PlayState<'a> {
    pub fn new() -> PlayState<'a> {
        let scene = Arc::new(Mutex::new(Node::new(None, None)));

        let dispatcher = DispatcherBuilder::new()
            .with(systems::AnimationSystem::new(), "animation_system", &[])
            .with(
                systems::ButtonHover {
                    scene: scene.clone(),
                },
                "button_hover",
                &[],
            )
            .with(systems::SellEnergy::new(), "sell_energy", &["button_hover"])
            .with(
                systems::BuildGatherer {
                    scene: scene.clone(),
                },
                "build_gatherer",
                &["button_hover"],
            )
            .with(
                systems::TextAbsoluteCache {
                    scene: scene.clone(),
                },
                "text_absolute_cache",
                &[],
            )
            .with(
                systems::TileSelection::new(scene.clone()),
                "tile_selection",
                &["build_gatherer"],
            )
            .with(systems::Gathering::new(), "gathering", &[])
            .with(systems::PowerUsage::new(), "power_usage", &["gathering"])
            .with(
                systems::ToggleTechTree::new(scene.clone()),
                "toggle_tech_tree",
                &["button_hover"],
            )
            .with(systems::Research::new(scene.clone()), "research", &[])
            .with(systems::Pollution::new(), "pollution", &[])
            .with(
                systems::CitiesToPower {
                    scene: scene.clone(),
                },
                "cities_to_power",
                &["button_hover"],
            )
            .build();

        let tech_tree_dispatcher = DispatcherBuilder::new()
            .with(
                systems::ButtonHover {
                    scene: scene.clone(),
                },
                "button_hover",
                &[],
            )
            .with(
                systems::ToggleTechTree::new(scene.clone()),
                "toggle_tech_tree",
                &["button_hover"],
            )
            .with(systems::TechTree::new(scene.clone()), "tech_tree", &[])
            .with(
                systems::TextAbsoluteCache {
                    scene: scene.clone(),
                },
                "text_absolute_cache",
                &[],
            )
            .build();

        let ps = PlayState {
            dispatcher,
            tech_tree_dispatcher,
            scene,
            state: InternalState::Game,
        };

        ps
    }

    pub fn get_name() -> String {
        "play_state".to_string()
    }

    pub fn create_random_map(&self) -> HashMap<(i32, i32), (TileType, Option<Entity>)> {
        let mut rng = thread_rng();

        let mut set_nodes = HashMap::new();
        // we'll build pockets of protected nodes
        for _ in 0..4 {
            let mut x;
            let mut y;
            // find the center first
            loop {
                x = rng.gen_range(1, 9);
                y = rng.gen_range(1, 9);

                let mut all_nodes_free = true;

                'check_nodes: for i in 0..3 {
                    for j in 0..3 {
                        if set_nodes.contains_key(&(x + i, y + j)) {
                            all_nodes_free = false;
                            break 'check_nodes;
                        }
                    }
                }

                if all_nodes_free {
                    break;
                }
            }

            // set the type for the center
            let weight: u32 = rng.gen_range(0, 101);
            let mut highest = 2;
            let tile_type = if weight >= 90 {
                highest = 4;
                TileType::City
            } else if weight >= 75 {
                highest = 3;
                TileType::River
            } else {
                TileType::EcoSystem
            };

            set_nodes.insert((x, y), (tile_type, None));

            let center_x = x;
            let center_y = y;

            x -= 1;
            y -= 1;

            // fill the surrounding tiles from the center with other types
            for i in 0..3 {
                for j in 0..3 {
                    if x + i == center_x && y + j == center_y {
                        continue;
                    }
                    let tile_type = if highest == 4 {
                        let weight: u32 = rng.gen_range(0, 101);
                        if weight >= 90 {
                            TileType::City
                        } else if weight >= 75 {
                            TileType::River
                        } else if weight >= 55 {
                            TileType::EcoSystem
                        } else {
                            TileType::Open
                        }
                    } else if highest == 3 {
                        let weight: u32 = rng.gen_range(0, 101);
                        if weight >= 75 {
                            TileType::River
                        } else if weight >= 50 {
                            TileType::EcoSystem
                        } else {
                            TileType::Open
                        }
                    } else if highest == 2 {
                        let weight: u32 = rng.gen_range(0, 101);
                        if weight >= 60 {
                            TileType::EcoSystem
                        } else {
                            TileType::Open
                        }
                    } else {
                        TileType::Open
                    };

                    set_nodes.insert((x + i, y + j), (tile_type, None));
                }
            }
        }

        set_nodes
    }
}

impl<'a> State for PlayState<'a> {
    fn get_scene(&self) -> Arc<Mutex<Node>> {
        self.scene.clone()
    }

    fn setup(&mut self, world: &mut World) {
        let mut scene = self.scene.lock().unwrap();
        scene.clear();

        let mut tile_nodes: Vec<Node> = Vec::with_capacity(100);
        let mut set_nodes = self.create_random_map();

        for row in 0..10 {
            for col in 0..10 {
                let size = Tile::get_size();
                let tile_type = if let Some(&(tile_type, _)) = set_nodes.get(&(col, row)) {
                    tile_type.clone()
                } else {
                    TileType::Open
                };
                let tile = Tile::new(tile_type);
                let sprite_frames = Tile::get_sprite_frames(&tile.tile_type);
                let frame_one = sprite_frames[0].clone();
                let tile_type = tile.tile_type.clone();
                let tile_entity = world
                    .create_entity()
                    .with(Transform::visible(
                        size * col as f32,
                        size * row as f32,
                        0.0,
                        size as u16,
                        size as u16,
                        0.0,
                        1.0,
                        1.0,
                    ))
                    .with(Sprite {
                        frame_name: frame_one.clone(),
                    })
                    .with(Button::new(frame_one, sprite_frames))
                    .with(tile)
                    .build();

                if tile_type != TileType::Open {
                    // replace the empty entity at this position with the entity
                    set_nodes.insert((col, row), (tile_type, Some(tile_entity.clone())));
                }

                tile_nodes.push(Node::new(Some(tile_entity), None));
            }
        }

        world.add_resource(ProtectedNodes { nodes: set_nodes });
        world.add_resource(GathererPositions::new());
        world.add_resource(GatheringRate::new());
        world.add_resource(Resources::new());
        world.add_resource(Wallet::new());

        scene.add_many(tile_nodes);

        let dimensions = renderer::get_dimensions();

        let mut side_bar_container = Node::new(
            Some(
                world
                    .create_entity()
                    .with(Transform::visible(
                        640.0,
                        0.0,
                        0.0,
                        (dimensions[0] - 640.0) as u16,
                        dimensions[1] as u16,
                        0.0,
                        1.0,
                        1.0,
                    ))
                    .build(),
            ),
            None,
        );

        let mut powerbar_frame_entities = Vec::new();

        for (i, coords) in CITY_POWER_STATE_COORDS.iter().enumerate() {
            let frame_name = if i == 0 {
                "powerbar.png".to_string()
            } else {
                "powerbar_disabled.png".to_string()
            };

            let entity = world
                .create_entity()
                .with(Transform::visible(
                    coords.0,
                    coords.1,
                    0.0,
                    130,
                    16,
                    0.0,
                    1.0,
                    1.0,
                ))
                .with(Sprite { frame_name })
                .build();
            side_bar_container.add(Node::new(Some(entity.clone()), None));
            powerbar_frame_entities.push(entity);
        }

        world.add_resource(CityPowerState::new(powerbar_frame_entities));

        {
            let entities = world.entities();
            let mut color_storage = world.write_storage::<Color>();
            let mut power_bar_storage = world.write_storage::<PowerBar>();
            let mut rect_storage = world.write_storage::<Rect>();
            let mut transform_storage = world.write_storage::<Transform>();
            let mut storages = PowerBarStorage {
                entities: &entities,
                color_storage: &mut color_storage,
                power_bar_storage: &mut power_bar_storage,
                rect_storage: &mut rect_storage,
                transform_storage: &mut transform_storage,
            };

            let entity = create_power_bar::create(&mut storages, 33.0, 35.0, 40);
            side_bar_container.add(Node::new(Some(entity), None));
        }

        // add city power target
        let power_additional_city = world
            .create_entity()
            .with(Sprite {
                frame_name: "power_additional_city.png".to_string(),
            })
            .with(Button::new(
                "power_additional_city".to_string(),
                [
                    "power_additional_city.png".to_string(),
                    "power_additional_city_hover.png".to_string(),
                ],
            ))
            .with(Transform::visible(30.0, 80.0, 0.0, 96, 32, 0.0, 1.0, 1.0))
            .build();
        side_bar_container.add(Node::new(Some(power_additional_city.clone()), None));

        // coal sprite
        let entity = world
            .create_entity()
            .with(ResourceCount {
                resource_type: ResourceType::Coal,
            })
            .with(Transform::visible(30.0, 158.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "coal.png".to_string(),
            })
            .build();
        side_bar_container.add(Node::new(Some(entity), None));

        // oil sprite
        let entity = world
            .create_entity()
            .with(ResourceCount {
                resource_type: ResourceType::Oil,
            })
            .with(Transform::visible(30.0, 204.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "oil.png".to_string(),
            })
            .build();
        side_bar_container.add(Node::new(Some(entity), None));

        // solar sprite
        let entity = world
            .create_entity()
            .with(ResourceCount {
                resource_type: ResourceType::Solar,
            })
            .with(Transform::visible(30.0, 250.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "sun.png".to_string(),
            })
            .build();
        side_bar_container.add(Node::new(Some(entity), None));

        // water sprite
        let entity = world
            .create_entity()
            .with(ResourceCount {
                resource_type: ResourceType::Hydro,
            })
            .with(Transform::visible(33.0, 296.0, 0.0, 26, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "water.png".to_string(),
            })
            .build();
        side_bar_container.add(Node::new(Some(entity), None));

        // money sprite
        let entity = world
            .create_entity()
            .with(WalletUI {})
            .with(Transform::visible(33.0, 344.0, 0.0, 26, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "dollarsign.png".to_string(),
            })
            .build();
        side_bar_container.add(Node::new(Some(entity), None));

        // pollution levels
        let entity = world
            .create_entity()
            .with(Transform::visible(33.0, 390.0, 0.0, 200, 32, 0.0, 1.0, 1.0))
            .with(PollutionCount { count: 0 })
            .with(Text::new_with_text(
                32.0,
                200,
                32,
                "Pollution: 0".to_string(),
            ))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container.add(Node::new(Some(entity), None));

        // selected
        let entity = world
            .create_entity()
            .with(SelectedTile {})
            .with(Transform::new(0.0, 0.0, 1.0, 64, 64, 0.0, 1.0, 1.0, false))
            .with(Rect::new())
            .with(Color([1.0, 1.0, 1.0, 0.6]))
            .build();
        scene.add(Node::new(Some(entity), None));

        // sell button
        let entity = world
            .create_entity()
            .with(Button::new(
                "power_btn".to_string(),
                [
                    "power_btn.png".to_string(),
                    "power_btn_hover.png".to_string(),
                ],
            ))
            .with(Transform::visible(182.0, 576.0, 0.0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "power_btn.png".to_string(),
            })
            .build();
        side_bar_container.add(Node::new(Some(entity), None));

        // tech tree button
        let entity = world
            .create_entity()
            .with(Button::new(
                "show_tech".to_string(),
                [
                    "show_tech.png".to_string(),
                    "show_tech_hover.png".to_string(),
                ],
            ))
            .with(Transform::visible(43.0, 576.0, 0.0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "show_tech.png".to_string(),
            })
            .build();
        side_bar_container.add(Node::new(Some(entity), None));

        {
            let mut lookup = world.write_resource::<EntityLookup>();
            lookup
                .entities
                .insert("show_button_entity".to_string(), entity);
            lookup.entities.insert(
                "side_bar_container".to_string(),
                side_bar_container.entity.unwrap(),
            );

            lookup
                .entities
                .insert("power_additional_city".to_string(), power_additional_city);

            let entities = world.entities();
            let mut color_storage = world.write_storage::<Color>();
            let mut transform_storage = world.write_storage::<Transform>();
            let mut text_storage = world.write_storage::<Text>();
            let mut resource_count_storage = world.write_storage::<ResourceCount>();
            let mut wallet_ui_storage = world.write_storage::<WalletUI>();

            let mut text_storages = TextStorage {
                entities: &entities,
                color_storage: &mut color_storage,
                text_storage: &mut text_storage,
                transform_storage: &mut transform_storage,
            };

            // coal text
            let entity = create_text::create(
                &mut text_storages,
                "".to_string(),
                32.0,
                80.0,
                158.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            resource_count_storage
                .insert(
                    entity.clone(),
                    ResourceCount {
                        resource_type: ResourceType::Coal,
                    },
                )
                .unwrap();
            side_bar_container.add(Node::new(Some(entity), None));

            // oil text
            let entity = create_text::create(
                &mut text_storages,
                "".to_string(),
                32.0,
                80.0,
                204.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            resource_count_storage
                .insert(
                    entity.clone(),
                    ResourceCount {
                        resource_type: ResourceType::Oil,
                    },
                )
                .unwrap();
            side_bar_container.add(Node::new(Some(entity), None));

            // solar text
            let entity = create_text::create(
                &mut text_storages,
                "".to_string(),
                32.0,
                80.0,
                250.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            resource_count_storage
                .insert(
                    entity.clone(),
                    ResourceCount {
                        resource_type: ResourceType::Solar,
                    },
                )
                .unwrap();
            side_bar_container.add(Node::new(Some(entity), None));

            let water_text = create_text::create(
                &mut text_storages,
                format!("{}", Wallet::start_amount()),
                32.0,
                80.0,
                296.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            resource_count_storage
                .insert(
                    water_text.clone(),
                    ResourceCount {
                        resource_type: ResourceType::Hydro,
                    },
                )
                .unwrap();
            side_bar_container.add(Node::new(Some(water_text), None));

            // money text
            let entity = create_text::create(
                &mut text_storages,
                format!("{}", Wallet::start_amount()),
                32.0,
                80.0,
                344.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            wallet_ui_storage.insert(entity, WalletUI {}).unwrap();
            side_bar_container.add(Node::new(Some(entity), None));
        }

        scene.add(side_bar_container);

        let tech_tree_container_entity = world
            .create_entity()
            .with(Transform::new(
                640.0,
                0.0,
                2.0,
                (dimensions[0] - 640.0) as u16,
                dimensions[1] as u16,
                0.0,
                1.0,
                1.0,
                false,
            ))
            .with(Rect {})
            .with(Color([16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0]))
            .build();
        let mut tech_tree_container = Node::new(Some(tech_tree_container_entity.clone()), None);
        let tech_tree_node = tech_tree::build_tech_tree(world, &mut tech_tree_container);

        let resume_from_upgrades = world
            .create_entity()
            .with(Button::new(
                "resume_from_upgrades".to_string(),
                ["resume.png".to_string(), "resume_hover.png".to_string()],
            ))
            .with(Transform::visible(112.0, 576.0, 0.0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "resume.png".to_string(),
            })
            .build();

        world.add_resource::<tech_tree::TechTreeNode>(tech_tree_node);
        world.add_resource::<ResearchedBuffs>(ResearchedBuffs(HashSet::new()));
        world.add_resource::<ResearchingEntities>(ResearchingEntities::new());

        let mut lookup = world.write_resource::<EntityLookup>();

        lookup.entities.insert(
            "tech_tree_container".to_string(),
            tech_tree_container_entity,
        );

        lookup
            .entities
            .insert("resume_from_upgrades".to_string(), resume_from_upgrades);
        tech_tree_container.add(Node::new(Some(resume_from_upgrades), None));

        lookup.entities.insert(
            "tech_tree_container".to_string(),
            tech_tree_container.entity.unwrap(),
        );
        scene.add(tech_tree_container);
    }

    fn update(&mut self, world: &mut World) {
        match self.state {
            InternalState::Game => self.dispatcher.dispatch(&mut world.res),
            InternalState::TechTree => self.tech_tree_dispatcher.dispatch(&mut world.res),
            _ => {}
        }
    }

    fn handle_custom_change(&mut self, action: &String) {
        if action == "tech_tree_pause" {
            self.state = InternalState::TechTree;
        } else if action == "tech_tree_resume" {
            self.state = InternalState::Game;
        }
    }
}
