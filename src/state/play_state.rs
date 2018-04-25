use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};
use specs::{Dispatcher, DispatcherBuilder, World};
use scene::Node;
use state::State;
use std::ops::DerefMut;

use components::{Button, Color, CurrentPower, EntityLookup, PollutionCount, PowerBar, ProtectedNodes, Rect,
                 ResearchedBuffs, ResearchingCount, ResourceCount, ResourceType, Resources,
                 SelectedTile, Sprite, Text, Tile, TileType, Transform, Wallet};
use components::ui::WalletUI;
use systems;
use renderer;
use entities::{create_text, tech_tree};
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
            .add(systems::AnimationSystem::new(), "animation_system", &[])
            .add(systems::PowerUsage::new(), "power_usage", &[])
            .add(
                systems::ButtonHover {
                    scene: scene.clone(),
                },
                "button_hover",
                &[],
            )
            .add(systems::SellEnergy {}, "sell_energy", &["button_hover"])
            .add(
                systems::BuildGatherer {
                    scene: scene.clone(),
                },
                "build_gatherer",
                &["button_hover"],
            )
            .add(
                systems::TextAbsoluteCache {
                    scene: scene.clone(),
                },
                "text_absolute_cache",
                &[],
            )
            .add(
                systems::TileSelection::new(scene.clone()),
                "tile_selection",
                &["build_gatherer"],
            )
            .add(systems::Gathering {}, "gathering", &[])
            .add(
                systems::ToggleTechTree::new(scene.clone()),
                "toggle_tech_tree",
                &["button_hover"],
            )
            .add(systems::Research::new(scene.clone()), "research", &[])
            .add(systems::Pollution::new(), "pollution", &[])
            .build();

        let tech_tree_dispatcher = DispatcherBuilder::new()
            .add(
                systems::ButtonHover {
                    scene: scene.clone(),
                },
                "button_hover",
                &[],
            )
            .add(
                systems::ToggleTechTree::new(scene.clone()),
                "toggle_tech_tree",
                &["button_hover"],
            )
            .add(systems::TechTree::new(scene.clone()), "tech_tree", &[])
            .add(
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
}

impl<'a> State for PlayState<'a> {
    fn get_scene(&self) -> Arc<Mutex<Node>> {
        self.scene.clone()
    }

    fn setup(&mut self, world: &mut World) {
        let mut scene = self.scene.lock().unwrap();
        scene.clear();

        let mut tile_nodes: Vec<Node> = Vec::with_capacity(100);
        let mut rng = thread_rng();

        let mut set_nodes = HashMap::new();
        for _ in 0..3 {
            let mut x = 0;
            let mut y = 0;
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

            let weight: u32 = rng.gen_range(0, 101);
            let mut highest = 1;
            let tile_type = if weight >= 90 {
                highest = 4;
                TileType::City
            } else if weight >= 75 {
                highest = 3;
                TileType::River
            } else {
                highest = 2;
                TileType::EcoSystem
            };

            set_nodes.insert((x, y), tile_type);

            let center_x = x;
            let center_y = y;

            x -= 1;
            y -= 1;

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

                    set_nodes.insert((x + i, y + j), tile_type);
                }
            }
        }

        for row in 0..10 {
            for col in 0..10 {
                let size = Tile::get_size();
                let tile_type = if let Some(tile_type) = set_nodes.get(&(col, row)) {
                    (*tile_type).clone()
                } else {
                    TileType::Open
                };
                let col = col as f32;
                let row = row as f32;
                let tile = Tile::new(tile_type);
                let sprite_frames = Tile::get_sprite_frames(&tile.tile_type);
                let frame_one = sprite_frames[0].clone();
                let tile_entity = world
                    .create_entity()
                    .with(Transform::visible(
                        size * col,
                        size * row,
                        0.0,
                        size as u16,
                        size as u16,
                        0.0,
                        1.0,
                        1.0,
                    ))
                    .with(Sprite {
                        frame_name: frame_one.clone(),
                    });

                let tile_entity = if tile.tile_type == TileType::Open {
                    tile_entity.with(Button::new(frame_one, sprite_frames))
                } else {
                    tile_entity
                };

                let tile_entity = tile_entity.with(tile).build();

                tile_nodes.push(Node::new(Some(tile_entity), None));
            }
        }

        world.add_resource(ProtectedNodes { nodes: set_nodes });

        {
            let mut resources_storage = world.write_resource::<Resources>();
            let resources: &mut Resources = resources_storage.deref_mut();

            resources.reset();

            let mut wallet_storage = world.write_resource::<Wallet>();
            let wallet: &mut Wallet = wallet_storage.deref_mut();

            wallet.reset();
        }

        scene.sub_nodes.push(Node::new(None, Some(tile_nodes)));

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

        let entity = world
            .create_entity()
            .with(PowerBar::new())
            .with(Transform::visible(30.0, 32.0, 0.0, 260, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "powerbar.png".to_string(),
            })
            .build();
        side_bar_container
            .sub_nodes
            .push(Node::new(Some(entity), None));

        let entity = world
            .create_entity()
            .with(CurrentPower {})
            .with(Transform::visible(
                34.0,
                36.0,
                1.0,
                CurrentPower::get_max_with(),
                24,
                0.0,
                1.0,
                1.0,
            ))
            .with(Rect::new())
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container
            .sub_nodes
            .push(Node::new(Some(entity), None));

        // coal sprite
        let entity = world
            .create_entity()
            .with(ResourceCount {
                resource_type: ResourceType::Coal,
            })
            .with(Transform::visible(30.0, 108.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "coal.png".to_string(),
            })
            .build();
        side_bar_container
            .sub_nodes
            .push(Node::new(Some(entity), None));

        // oil sprite
        let entity = world
            .create_entity()
            .with(ResourceCount {
                resource_type: ResourceType::Oil,
            })
            .with(Transform::visible(30.0, 142.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "oil.png".to_string(),
            })
            .build();
        side_bar_container
            .sub_nodes
            .push(Node::new(Some(entity), None));

        // solar sprite
        let entity = world
            .create_entity()
            .with(ResourceCount {
                resource_type: ResourceType::Solar,
            })
            .with(Transform::visible(30.0, 188.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "sun.png".to_string(),
            })
            .build();
        side_bar_container
            .sub_nodes
            .push(Node::new(Some(entity), None));

        // money sprite
        let entity = world
            .create_entity()
            .with(WalletUI {})
            .with(Transform::visible(33.0, 228.0, 0.0, 26, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "dollarsign.png".to_string(),
            })
            .build();
        side_bar_container
            .sub_nodes
            .push(Node::new(Some(entity), None));

        // pollution levels
        let entity = world
            .create_entity()
            .with(Transform::visible(33.0, 260.0, 0.0, 200, 32, 0.0, 1.0, 1.0))
            .with(PollutionCount{ count: 0 })
            .with(Text::new_with_text(32.0, 200, 32, "Pollution: 0".to_string()))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // selected
        let entity = world
            .create_entity()
            .with(SelectedTile {})
            .with(Transform::new(0.0, 0.0, 1.0, 64, 64, 0.0, 1.0, 1.0, false))
            .with(Rect::new())
            .with(Color([1.0, 1.0, 1.0, 0.6]))
            .build();
        scene.sub_nodes.push(Node::new(Some(entity), None));

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
        side_bar_container
            .sub_nodes
            .push(Node::new(Some(entity), None));

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
        side_bar_container
            .sub_nodes
            .push(Node::new(Some(entity), None));

        {
            let mut lookup = world.write_resource::<EntityLookup>();
            lookup
                .entities
                .insert("show_button_entity".to_string(), entity);
            lookup.entities.insert(
                "side_bar_container".to_string(),
                side_bar_container.entity.unwrap(),
            );

            let entities = world.entities();
            let mut color_storage = world.write::<Color>();
            let mut transform_storage = world.write::<Transform>();
            let mut text_storage = world.write::<Text>();
            let mut resource_count_storage = world.write::<ResourceCount>();
            let mut wallet_ui_storage = world.write::<WalletUI>();

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
                108.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            resource_count_storage.insert(
                entity.clone(),
                ResourceCount {
                    resource_type: ResourceType::Coal,
                },
            );
            side_bar_container
                .sub_nodes
                .push(Node::new(Some(entity), None));

            // oil text
            let entity = create_text::create(
                &mut text_storages,
                "".to_string(),
                32.0,
                80.0,
                142.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            resource_count_storage.insert(
                entity.clone(),
                ResourceCount {
                    resource_type: ResourceType::Oil,
                },
            );
            side_bar_container
                .sub_nodes
                .push(Node::new(Some(entity), None));

            // solar text
            let entity = create_text::create(
                &mut text_storages,
                "".to_string(),
                32.0,
                80.0,
                188.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            resource_count_storage.insert(
                entity.clone(),
                ResourceCount {
                    resource_type: ResourceType::Solar,
                },
            );
            side_bar_container
                .sub_nodes
                .push(Node::new(Some(entity), None));

            // money text
            let entity = create_text::create(
                &mut text_storages,
                format!("{}", Wallet::start_amount()),
                32.0,
                80.0,
                228.0,
                0.0,
                160,
                32,
                Color([0.0, 1.0, 0.0, 1.0]),
            );
            wallet_ui_storage.insert(entity, WalletUI {});
            side_bar_container
                .sub_nodes
                .push(Node::new(Some(entity), None));
        }

        scene.sub_nodes.push(side_bar_container);

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
        world.add_resource::<ResearchingCount>(ResearchingCount { count: 0 });

        let mut lookup = world.write_resource::<EntityLookup>();

        lookup.entities.insert(
            "tech_tree_container".to_string(),
            tech_tree_container_entity,
        );

        lookup
            .entities
            .insert("resume_from_upgrades".to_string(), resume_from_upgrades);
        tech_tree_container
            .sub_nodes
            .push(Node::new(Some(resume_from_upgrades), None));

        lookup.entities.insert(
            "tech_tree_container".to_string(),
            tech_tree_container.entity.unwrap(),
        );
        scene.sub_nodes.push(tech_tree_container);
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
