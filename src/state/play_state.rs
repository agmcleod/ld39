use std::sync::{Arc, Mutex};
use specs::{Dispatcher, DispatcherBuilder, World};
use scene::Node;
use state::State;
use rusttype::Font;
use std::ops::DerefMut;

use components::{Button, Color, CurrentPower, EntityLookup, PowerBar, Rect, ResourceCount, Resources, ResourceType, SelectedTile, Sprite, Text, Tile, Transform, Wallet};
use components::ui::WalletUI;
use systems;
use tech_tree;
use renderer;

enum InternalState {
    Game,
    TechTree,
    Pause,
}

pub struct PlayState<'a> {
    dispatcher: Dispatcher<'a, 'a>,
    tech_tree_dispatcher: Dispatcher<'a, 'a>,
    scene: Arc<Mutex<Node>>,
    font: Arc<Font<'static>>,
    state: InternalState,
}

impl <'a>PlayState<'a> {
    pub fn new(font: &Arc<Font<'static>>) -> PlayState<'a> {
        let scene = Arc::new(Mutex::new(Node::new(None, None)));

        let dispatcher = DispatcherBuilder::new()
            .add(systems::AnimationSystem::new(), "animation_system", &[])
            .add(systems::PowerUsage::new(), "power_usage", &[])
            .add(systems::ButtonHover{ scene: scene.clone() }, "button_hover", &[])
            .add(systems::SellEnergy{}, "sell_energy", &["button_hover"])
            .add(systems::BuildGatherer{ scene: scene.clone() }, "build_gatherer", &["button_hover"])
            .add(systems::TileSelection::new(scene.clone()), "tile_selection", &["build_gatherer"])
            .add(systems::Gathering{}, "gathering", &[])
            .add(systems::ToggleTechTree::new(scene.clone()), "toggle_tech_tree", &["button_hover"])
            .build();

        let tech_tree_dispatcher = DispatcherBuilder::new()
            .add(systems::ButtonHover{ scene: scene.clone() }, "button_hover", &[])
            .add(systems::ToggleTechTree::new(scene.clone()), "toggle_tech_tree", &["button_hover"])
            .add(systems::TechTree::new(scene.clone()), "tech_tree", &[])
            .build();

        let ps = PlayState{
            dispatcher: dispatcher,
            tech_tree_dispatcher: tech_tree_dispatcher,
            scene: scene,
            font: font.clone(),
            state: InternalState::Game,
        };

        ps
    }

    pub fn get_name() -> String {
        "play_state".to_string()
    }
}

impl <'a>State for PlayState<'a> {
    fn get_scene(&self) -> Arc<Mutex<Node>> {
        self.scene.clone()
    }

    fn setup(&mut self, world: &mut World) {
        let mut scene = self.scene.lock().unwrap();
        scene.clear();

        let mut tile_nodes: Vec<Node> = Vec::with_capacity(100);
        for row in 0..10 {
            for col in 0..10 {
                let col = col as f32;
                let row = row as f32;
                let size = Tile::get_size();
                let tile = world.create_entity()
                    .with(Transform::visible(size * col, size * row, 0.0, size as u16, size as u16, 0.0, 1.0, 1.0))
                    .with(Button::new("tiles".to_string(), ["tiles.png".to_string(), "tiles_highlight.png".to_string()]))
                    .with(Sprite{ frame_name: "tiles.png".to_string() })
                    .with(Tile{})
                    .build();

                tile_nodes.push(Node::new(Some(tile), None));
            }
        }

        {
            let mut resources_storage = world.write_resource::<Resources>();
            let resources: &mut Resources = resources_storage.deref_mut();

            resources.reset();

            let mut wallet_storage = world.write_resource::<Wallet>();
            let wallet: &mut Wallet = wallet_storage.deref_mut();

            wallet.reset();
        }

        let font = &self.font;

        scene.sub_nodes.push(Node::new(None, Some(tile_nodes)));

        let dimensions = renderer::get_dimensions();

        let mut side_bar_container = Node::new(Some(world.create_entity()
            .with(Transform::visible(640.0, 0.0, 0.0, (dimensions[0] - 640.0) as u16, dimensions[1] as u16, 0.0, 1.0, 1.0))
            .build()), None);

        let entity = world.create_entity()
            .with(PowerBar::new())
            .with(Transform::visible(30.0, 32.0, 0.0, 260, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "powerbar.png".to_string() })
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(CurrentPower{})
            .with(Transform::visible(34.0, 36.0, 1.0, CurrentPower::get_max_with(), 24, 0.0, 1.0, 1.0))
            .with(Rect::new())
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // coal sprite
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Coal })
            .with(Transform::visible(30.0, 108.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "coal.png".to_string() })
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // coal text
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Coal })
            .with(Transform::visible(80.0, 108.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Text::new(&font, 32.0))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // oil sprite
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Oil })
            .with(Transform::visible(30.0, 142.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "oil.png".to_string() })
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // oil text
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Oil })
            .with(Transform::visible(80.0, 142.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Text::new(&font, 32.0))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // solar sprite
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Clean })
            .with(Transform::visible(30.0, 188.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "sun.png".to_string() })
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // solar text
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Clean })
            .with(Transform::visible(80.0, 188.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Text::new(&font, 32.0))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // money sprite
        let entity = world.create_entity()
            .with(WalletUI{})
            .with(Transform::visible(33.0, 228.0, 0.0, 26, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "dollarsign.png".to_string() })
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // money text
        let mut text = Text::new(&font, 32.0);
        text.set_text(format!("{}", Wallet::start_amount()));
        let entity = world.create_entity()
            .with(WalletUI{})
            .with(Transform::visible(80.0, 228.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(text)
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // selected
        let entity = world.create_entity()
            .with(SelectedTile{})
            .with(Transform::new(0.0, 0.0, 1.0, 64, 64, 0.0, 1.0, 1.0, false))
            .with(Rect::new())
            .with(Color([1.0, 1.0, 1.0, 0.6]))
            .build();
        scene.sub_nodes.push(Node::new(Some(entity), None));

        // sell button
        let entity = world.create_entity()
            .with(Button::new("power_btn".to_string(), ["power_btn.png".to_string(), "power_btn_hover.png".to_string()]))
            .with(Transform::visible(182.0, 576.0, 0.0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "power_btn.png".to_string() })
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        // tech tree button
        let entity = world.create_entity()
            .with(Button::new("show_tech".to_string(), ["show_tech.png".to_string(), "show_tech_hover.png".to_string()]))
            .with(Transform::visible(43.0, 576.0, 0.0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "show_tech.png".to_string() })
            .build();
        side_bar_container.sub_nodes.push(Node::new(Some(entity), None));

        {
            let mut lookup = world.write_resource::<EntityLookup>();
            lookup.entities.insert("show_button_entity".to_string(), entity);
            lookup.entities.insert("side_bar_container".to_string(), side_bar_container.entity.unwrap());
        }

        scene.sub_nodes.push(side_bar_container);

        let mut tech_tree_node = tech_tree::build_tech_tree(world);
        let mut tech_tree_container = Node::new(Some(world.create_entity()
            .with(Transform::new(640.0, 0.0, 2.0, (dimensions[0] - 640.0) as u16, dimensions[1] as u16, 0.0, 1.0, 1.0, false))
            .with(Rect{})
            .with(Color([16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0]))
            .build()), None);
        {
            let mut add_to_container = |node: &mut tech_tree::TechTreeNode| {
                tech_tree_container.sub_nodes.push(Node::new(Some(node.entity), None));
                false
            };
            tech_tree::traverse_tree(&mut tech_tree_node, &mut add_to_container);
        }

        let entity = world.create_entity()
            .with(Button::new("resume_from_upgrades".to_string(), ["resume.png".to_string(), "resume_hover.png".to_string()]))
            .with(Transform::visible(112.0, 576.0, 0.0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "resume.png".to_string() })
            .build();

        let mut lookup = world.write_resource::<EntityLookup>();

        lookup.entities.insert("resume_from_upgrades".to_string(), entity);
        tech_tree_container.sub_nodes.push(Node::new(Some(entity), None));

        lookup.entities.insert("tech_tree_container".to_string(), tech_tree_container.entity.unwrap());
        scene.sub_nodes.push(tech_tree_container);
    }

    fn update(&mut self, world: &mut World) {
        match self.state {
            InternalState::Game => self.dispatcher.dispatch(&mut world.res),
            InternalState::TechTree => self.tech_tree_dispatcher.dispatch(&mut world.res),
            _ => {},
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