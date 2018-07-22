use cgmath::Vector2;
use conrod;
use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Ui, UiBuilder, Widget};
use loader;
use specs::{Dispatcher, DispatcherBuilder, World};
use state::State;
use std::collections::HashSet;
use std::path::Path;

use components::{ui::{PollutionCount, WalletUI},
                 upgrade,
                 upgrade::Buff,
                 Actions,
                 Button,
                 CityPowerState,
                 Color,
                 EntityLookup,
                 GathererPositions,
                 GatheringRate,
                 Node,
                 PowerBar,
                 TileNodes,
                 Rect,
                 ResearchedBuffs,
                 ResearchingEntities,
                 ResourceCount,
                 ResourceType,
                 Resources,
                 SelectedTile,
                 Shape,
                 Sprite,
                 Text,
                 Tile,
                 TileType,
                 Transform,
                 TutorialStep,
                 Wallet,
                 CITY_POWER_STATE_COORDS};
use entities::{create_map, create_power_bar, create_text, tech_tree};
use rand::{thread_rng, Rng};
use renderer;
use settings::Settings;
use storage_types::*;
use systems;

widget_ids! {
    struct Ids {
        music_volume,
        sound_volume,
        mute_music,
        mute_sound_effects,
        mute_music_label,
        mute_sound_effects_label,
        settings_label,
        close_button,
    }
}

#[derive(PartialEq)]
pub enum InternalState {
    Game,
    TechTree,
    Pause,
}

impl Default for InternalState {
    fn default() -> Self {
        InternalState::Game
    }
}

pub struct PlayState<'a> {
    dispatcher: Dispatcher<'a, 'a>,
    tech_tree_dispatcher: Dispatcher<'a, 'a>,
    pause_dispatcher: Dispatcher<'a, 'a>,
    state: InternalState,
    ui: Ui,
    ids: Ids,
}

impl<'a> PlayState<'a> {
    pub fn new() -> PlayState<'a> {
        let dispatcher = DispatcherBuilder::new()
            .with(systems::AnimationSystem::new(), "animation_system", &[])
            .with(systems::ButtonHover {}, "button_hover", &[])
            .with(systems::SellEnergy::new(), "sell_energy", &["button_hover"])
            .with(
                systems::BuildGatherer {},
                "build_gatherer",
                &["button_hover"],
            )
            .with(systems::TextAbsoluteCache {}, "text_absolute_cache", &[])
            .with(
                systems::TileSelection::new(),
                "tile_selection",
                &["build_gatherer"],
            )
            .with(systems::Gathering::new(), "gathering", &[])
            .with(systems::PowerUsage::new(), "power_usage", &["gathering"])
            .with(
                systems::ToggleTechTree::new(),
                "toggle_tech_tree",
                &["button_hover"],
            )
            .with(systems::Research::new(), "research", &[])
            .with(systems::Pollution::new(), "pollution", &["gathering"])
            .with(
                systems::CitiesToPower {},
                "cities_to_power",
                &["button_hover"],
            )
            .with(
                systems::FloatingTextSystem::new(),
                "floating_text_system",
                &[],
            )
            .with(systems::TogglePause {}, "toggle_pause", &["button_hover"])
            .with(systems::Tutorial {}, "tutorial", &[])
            .with(systems::PulseSystem {}, "pulse", &[])
            .build();

        let tech_tree_dispatcher = DispatcherBuilder::new()
            .with(systems::ButtonHover {}, "button_hover", &[])
            .with(
                systems::ToggleTechTree::new(),
                "toggle_tech_tree",
                &["button_hover"],
            )
            .with(systems::TechTree::new(), "tech_tree", &[])
            .with(systems::TextAbsoluteCache {}, "text_absolute_cache", &[])
            .build();

        let pause_dispatcher = DispatcherBuilder::new()
            .with(systems::ButtonHover {}, "button_hover", &[])
            .with(systems::TextAbsoluteCache {}, "text_absolute_cache", &[])
            .with(systems::TogglePause {}, "toggle_pause", &["button_hover"])
            .build();

        let dim = renderer::get_dimensions();
        let mut ui = UiBuilder::new([dim[0] as f64, dim[1] as f64]).build();
        ui.fonts
            .insert_from_file(Path::new(&loader::get_exe_path().join("resources/MunroSmall.ttf")))
            .unwrap();

        let ids = Ids::new(ui.widget_id_generator());

        let ps = PlayState {
            dispatcher,
            tech_tree_dispatcher,
            pause_dispatcher,
            state: InternalState::Game,
            ui,
            ids,
        };

        ps
    }

    pub fn get_name() -> String {
        "play_state".to_string()
    }
}

impl<'a> State for PlayState<'a> {
    fn setup(&mut self, world: &mut World) {
        let mut set_nodes = create_map::create();
        let mut rng = thread_rng();

        let mut entities_under_root = Vec::new();

        for row in 0..10 {
            for col in 0..10 {
                let size = Tile::get_size();
                let tile_type = if let Some(&(tile_type, _)) = set_nodes.get(&(col, row)) {
                    tile_type.clone()
                } else {
                    let r = rng.gen_range(0, 10);
                    if r >= 9 {
                        TileType::River
                    } else if r >= 6 {
                        TileType::City
                    } else {
                        TileType::EcoSystem
                    }
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

                entities_under_root.push(tile_entity);
            }
        }

        world.add_resource(TileNodes { nodes: set_nodes });
        world.add_resource(GathererPositions::new());
        world.add_resource(GatheringRate::new());
        world.add_resource(Resources::new());
        world.add_resource(Wallet::new());
        world.add_resource(InternalState::Game);
        world.add_resource(TutorialStep::SelectTile);

        let dimensions = renderer::get_dimensions();

        let mut side_bar_container_node = Node::new();

        let entity = world
            .create_entity()
            .with(Transform::visible(
                dimensions[0] - 640.0 - 52.0,
                4.0,
                0.0,
                22,
                22,
                0.0,
                1.0,
                1.0,
            ))
            .with(Sprite {
                frame_name: "menu.png".to_string(),
            })
            .with(Button::new(
                "menu".to_string(),
                ["menu.png".to_string(), "menu_hover.png".to_string()],
            ))
            .build();

        side_bar_container_node.add(entity);

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
                    coords.0, coords.1, 0.0, 130, 16, 0.0, 1.0, 1.0,
                ))
                .with(Sprite { frame_name })
                .build();
            side_bar_container_node.add(entity);
            powerbar_frame_entities.push(entity);
        }

        world.add_resource(CityPowerState::new(powerbar_frame_entities));

        // create first ppower bar
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
            side_bar_container_node.add(entity);
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
        side_bar_container_node.add(power_additional_city);

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
        side_bar_container_node.add(entity);

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
        side_bar_container_node.add(entity);

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
        side_bar_container_node.add(entity);

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
        side_bar_container_node.add(entity);

        // money sprite
        let entity = world
            .create_entity()
            .with(WalletUI {})
            .with(Transform::visible(33.0, 344.0, 0.0, 26, 32, 0.0, 1.0, 1.0))
            .with(Sprite {
                frame_name: "dollarsign.png".to_string(),
            })
            .build();
        side_bar_container_node.add(entity);

        // pollution levels
        let entity = world
            .create_entity()
            .with(Transform::visible(33.0, 390.0, 0.0, 200, 32, 0.0, 1.0, 1.0))
            .with(PollutionCount { count: 0 })
            .with(Text::new_with_text(
                28.0,
                280,
                32,
                "Pollution tax: 0%".to_string(),
            ))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container_node.add(entity);

        // selected
        let entity = world
            .create_entity()
            .with(SelectedTile {})
            .with(Transform::new(0.0, 0.0, 1.0, 64, 64, 0.0, 1.0, 1.0, false))
            .with(Rect::new())
            .with(Color([1.0, 1.0, 1.0, 0.6]))
            .build();
        entities_under_root.push(entity);

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
        side_bar_container_node.add(entity);

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
        side_bar_container_node.add(entity);

        let mut lookup = EntityLookup::new();

        {
            lookup
                .entities
                .insert("show_button_entity".to_string(), entity);
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
            side_bar_container_node.add(entity);

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
            side_bar_container_node.add(entity);

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
            side_bar_container_node.add(entity);

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
            side_bar_container_node.add(water_text);

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
            side_bar_container_node.add(entity);

            // power gain text
            let entity = create_text::create(
                &mut text_storages,
                "Power: ".to_string(),
                24.0,
                30.0,
                120.0,
                0.0,
                160,
                32,
                Color([0.0, 0.6, 0.0, 1.0]),
            );

            lookup
                .entities
                .insert("power_gain_text".to_string(), entity.clone());
            side_bar_container_node.add(entity);
        }

        let side_bar_container = world
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
            .with(side_bar_container_node)
            .build();

        lookup
            .entities
            .insert("side_bar_container".to_string(), side_bar_container);

        entities_under_root.push(side_bar_container);

        let mut tech_tree_container = Node::new();
        let mut upgrade_lines_lookup = upgrade::UpgradeLinesLookup::new();
        let tech_tree_node =
            tech_tree::build_tech_tree(world, &mut tech_tree_container, &mut upgrade_lines_lookup);

        world.add_resource(upgrade_lines_lookup);

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
        let mut researched_buffs = ResearchedBuffs(HashSet::new());
        researched_buffs.0.insert(Buff::Coal);
        world.add_resource::<ResearchedBuffs>(researched_buffs);
        world.add_resource::<ResearchingEntities>(ResearchingEntities::new());

        lookup
            .entities
            .insert("resume_from_upgrades".to_string(), resume_from_upgrades);
        tech_tree_container.add(resume_from_upgrades);

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
            .with(tech_tree_container)
            .build();

        lookup.entities.insert(
            "tech_tree_container".to_string(),
            tech_tree_container_entity,
        );

        lookup.entities.insert(
            "tech_tree_container".to_string(),
            tech_tree_container_entity,
        );
        entities_under_root.push(tech_tree_container_entity);

        let mut root_node = Node::new();
        root_node.add_many(entities_under_root);

        let root_entity = world.create_entity().with(root_node).build();

        lookup.entities.insert("root".to_string(), root_entity);

        world.add_resource(lookup);
    }

    fn update(&mut self, world: &mut World) {
        match self.state {
            InternalState::Game => self.dispatcher.dispatch(&mut world.res),
            InternalState::TechTree => self.tech_tree_dispatcher.dispatch(&mut world.res),
            InternalState::Pause => self.pause_dispatcher.dispatch(&mut world.res),
        }
    }

    fn handle_custom_change(&mut self, action: &String, world: &mut World) {
        if action == "tech_tree_pause" {
            self.state = InternalState::TechTree;
            world.add_resource(InternalState::TechTree);
        } else if action == "resume" {
            self.state = InternalState::Game;
            world.add_resource(InternalState::Game);
        } else if action == "pause" {
            self.state = InternalState::Pause;
            println!("Set state to pause");
            world.add_resource(InternalState::Pause);
        }
    }

    fn get_ui_to_render(&mut self) -> &mut Ui {
        &mut self.ui
    }

    fn create_ui_widgets(&mut self, settings: &mut Settings, world: &mut World) {
        let ui = &mut self.ui.set_widgets();

        if widget::Button::new()
            .top_right_with_margin_on(ui.window, 20.0)
            .w_h(30.0, 30.0)
            .label("X")
            .label_color(conrod::color::rgb(0.0, 1.0, 0.0))
            .color(conrod::color::rgb(16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0))
            .set(self.ids.close_button, ui)
            .was_clicked()
        {
            let mut actions = world.write_resource::<Actions>();
            actions.dispatch("resume_game".to_string());
        }

        widget::Text::new("Settings")
            .mid_top_with_margin_on(ui.window, 50.0)
            .font_size(32)
            .rgb(0.0, 1.0, 0.0)
            .set(self.ids.settings_label, ui);

        if let Some(volume) = widget::Slider::new(settings.music_volume, 0.0, 1.0)
            .middle_of(ui.window)
            .down_from(self.ids.settings_label, 50.0)
            .x_align(conrod::position::Align::Middle)
            .color(conrod::color::rgb(0.0, 1.0, 0.0))
            .w_h(350.0, 35.0)
            .label("Music Volume")
            .set(self.ids.music_volume, ui)
        {
            settings.set_music_volume(volume);
        }

        if let Some(volume) = widget::Slider::new(settings.sound_volume, 0.0, 1.0)
            .middle_of(ui.window)
            .color(conrod::color::rgb(0.0, 1.0, 0.0))
            .w_h(350.0, 35.0)
            .label("Sound Volume")
            .down_from(self.ids.music_volume, 25.0)
            .set(self.ids.sound_volume, ui)
        {
            settings.set_sound_volume(volume);
        }

        widget::Text::new("Mute music")
            .down_from(self.ids.sound_volume, 25.0)
            .color(conrod::color::rgb(0.0, 1.0, 0.0))
            .font_size(20)
            .set(self.ids.mute_music_label, ui);

        if let Some(state) = widget::Toggle::new(settings.mute_music)
            .down_from(self.ids.mute_music_label, 25.0)
            .color(conrod::color::rgb(0.0, 1.0, 0.0))
            .w_h(35.0, 35.0)
            .label(if settings.mute_music { "X" } else { "" })
            .label_color(conrod::color::rgb(0.0, 0.0, 0.0))
            .set(self.ids.mute_music, ui)
            .last()
        {
            settings.set_mute_music(state);
        }

        widget::Text::new("Mute sound effects")
            .right_from(self.ids.mute_music_label, 100.0)
            .color(conrod::color::rgb(0.0, 1.0, 0.0))
            .font_size(20)
            .set(self.ids.mute_sound_effects_label, ui);

        if let Some(state) = widget::Toggle::new(settings.mute_sound_effects)
            .down_from(self.ids.mute_sound_effects_label, 25.0)
            .color(conrod::color::rgb(0.0, 1.0, 0.0))
            .w_h(35.0, 35.0)
            .label(if settings.mute_sound_effects { "X" } else { "" })
            .label_color(conrod::color::rgb(0.0, 0.0, 0.0))
            .set(self.ids.mute_sound_effects, ui)
            .last()
        {
            settings.set_mute_sound_effects(state);
        }
    }

    fn should_render_ui(&self) -> bool {
        self.state == InternalState::Pause
    }
}
