use std::sync::{Arc, Mutex};
use specs::{Dispatcher, DispatcherBuilder, World};
use scene::Scene;
use scene::node::Node;
use state::State;
use rusttype::Font;
use std::ops::DerefMut;

use components::{Button, Color, CurrentPower, PowerBar, Rect, ResourceCount, Resources, ResourceType, SelectedTile, Sprite, Text, Tile, Transform, Wallet, WalletUI};
use systems;
use tech_tree;

pub struct PlayState<'a> {
    dispatcher: Dispatcher<'a, 'a>,
    scene: Arc<Mutex<Scene>>,
    font: Arc<Font<'static>>,
}

impl <'a>PlayState<'a> {
    pub fn new(font: &Arc<Font<'static>>) -> PlayState<'a> {
        let scene = Arc::new(Mutex::new(Scene::new()));

        let dispatcher = DispatcherBuilder::new()
            .add(systems::AnimationSystem::new(), "animation_system", &[])
            .add(systems::PowerUsage::new(), "power_usage", &[])
            .add(systems::ButtonHover{ scene: scene.clone() }, "button_hover", &[])
            .add(systems::SellEnergy{}, "sell_energy", &["button_hover"])
            .add(systems::BuildGatherer{ built_one: false, scene: scene.clone() }, "build_gatherer", &["button_hover"])
            .add(systems::TileSelection::new(scene.clone()), "tile_selection", &["build_gatherer"])
            .add(systems::Gathering{}, "gathering", &[])
            .build();

        let ps = PlayState{
            dispatcher: dispatcher,
            scene: scene,
            font: font.clone(),
        };

        ps
    }

    pub fn get_name() -> String {
        "play_state".to_string()
    }
}

impl <'a>State for PlayState<'a> {
    fn get_scene(&self) -> Arc<Mutex<Scene>> {
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
                    .with(Transform::visible(size * col, size * row, 1.0, size as u16, size as u16, 0.0, 1.0, 1.0))
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

        scene.nodes.push(Node::new(None, Some(tile_nodes)));

        let entity = world.create_entity()
            .with(PowerBar::new())
            .with(Transform::visible(670.0, 32.0, 1.0, 260, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "powerbar.png".to_string() })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(CurrentPower{})
            .with(Transform::visible(674.0, 36.0, 0.0, CurrentPower::get_max_with(), 24, 0.0, 1.0, 1.0))
            .with(Rect::new())
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // coal sprite
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Coal })
            .with(Transform::visible(670.0, 108.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "coal.png".to_string() })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // coal text
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Coal })
            .with(Transform::visible(720.0, 108.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Text::new(&font, 32.0))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // oil sprite
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Oil })
            .with(Transform::visible(670.0, 142.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "oil.png".to_string() })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // oil text
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Oil })
            .with(Transform::visible(720.0, 142.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Text::new(&font, 32.0))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // solar sprite
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Clean })
            .with(Transform::visible(670.0, 188.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "sun.png".to_string() })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // solar text
        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Clean })
            .with(Transform::visible(720.0, 188.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(Text::new(&font, 32.0))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // money sprite
        let entity = world.create_entity()
            .with(WalletUI{})
            .with(Transform::visible(673.0, 228.0, 0.0, 26, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "dollarsign.png".to_string() })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // money text
        let mut text = Text::new(&font, 32.0);
        text.set_text(format!("{}", Wallet::start_amount()));
        let entity = world.create_entity()
            .with(WalletUI{})
            .with(Transform::visible(720.0, 228.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
            .with(text)
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // selected
        let entity = world.create_entity()
            .with(SelectedTile{})
            .with(Transform::new(0.0, 0.0, 0.0, 64, 64, 0.0, 1.0, 1.0, false))
            .with(Rect::new())
            .with(Color([1.0, 1.0, 1.0, 0.6]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // sell button
        let entity = world.create_entity()
            .with(Button::new("power-btn".to_string(), ["power-btn.png".to_string(), "power-btn-hover.png".to_string()]))
            .with(Transform::visible(770.0, 576.0, 0.0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "power-btn.png".to_string() })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        tech_tree::build_tech_tree(world);
    }

    fn update(&mut self, world: &mut World) {
        self.dispatcher.dispatch(&mut world.res);
    }
}