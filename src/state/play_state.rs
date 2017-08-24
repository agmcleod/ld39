use std::sync::{Arc, Mutex};
use specs::World;
use scene::Scene;
use scene::node::Node;
use state::State;
use rusttype::Font;

use components::{BuildCost, Button, Color, CurrentPower, GathererType, HighlightTile, PowerBar, Rect, ResourceCount, ResourceType, SelectedTile, SellCost, Sprite, Text, Tile, Transform, Upgrade, UpgradeCost};

pub struct PlayState {
    scene: Arc<Mutex<Scene>>,
    font: Arc<Font<'static>>,
}

impl PlayState {
    pub fn new(font: &Arc<Font<'static>>) -> PlayState {
        PlayState{
            scene: Arc::new(Mutex::new(Scene::new())),
            font: font.clone(),
        }
    }
}

impl State for PlayState {
    fn get_scene(&self) -> Arc<Mutex<Scene>> {
        self.scene.clone()
    }

    fn setup(&mut self, world: &mut World) {
        let mut scene = Scene::new();

        let mut tile_nodes: Vec<Node> = Vec::with_capacity(100);
        for row in 0i32..10i32 {
            for col in 0i32..10i32 {
                let size = Tile::get_size();
                let tile = world.create_entity()
                    .with(Transform::new(size * col, size * row, 1, size as u16, size as u16, 0.0, 1.0, 1.0))
                    .with(Sprite{ frame_name: "tiles.png".to_string(), visible: true })
                    .with(Tile{})
                    .build();

                tile_nodes.push(Node::new(Some(tile), None));
            }
        }

        let font = &self.font;

        scene.nodes.push(Node::new(None, Some(tile_nodes)));

        let entity = world.create_entity()
            .with(PowerBar::new())
            .with(Transform::new(670, 576, 1, 260, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "powerbar.png".to_string(), visible: true })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(CurrentPower{})
            .with(Transform::new(674, 580, 0, CurrentPower::get_max_with(), 24, 0.0, 1.0, 1.0))
            .with(Rect{})
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Coal })
            .with(Transform::new(670, 500, 0, 32, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "coal.png".to_string(), visible: true })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(ResourceCount{ resource_type: ResourceType::Coal })
            .with(Transform::new(720, 500, 0, 32, 32, 0.0, 1.0, 1.0))
            .with(Text::new(&font, 32.0))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(HighlightTile{ visible: false })
            .with(Transform::new(0, 0, 0, 64, 64, 0.0, 1.0, 1.0))
            .with(Color([1.0, 1.0, 1.0, 0.3]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(SelectedTile{ visible: false })
            .with(Transform::new(0, 0, 0, 64, 64, 0.0, 1.0, 1.0))
            .with(Color([1.0, 1.0, 1.0, 0.6]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(Button::new("build".to_string(), ["build.png".to_string(), "build_hover.png".to_string()]))
            .with(Transform::new(670, 32, 0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "build.png".to_string(), visible: true })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        let entity = world.create_entity()
            .with(Button::new("power-btn".to_string(), ["power-btn.png".to_string(), "power-btn-hover.png".to_string()]))
            .with(Transform::new(820, 32, 0, 96, 32, 0.0, 1.0, 1.0))
            .with(Sprite{ frame_name: "sell.png".to_string(), visible: true })
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // upgrade stuff
        let mut text = Text::new(&font, 32.0);
        text.visible = false;
        text.set_text(format!("{}", Upgrade::new().get_cost()));
        let entity = world.create_entity()
            .with(UpgradeCost{})
            .with(text)
            .with(Transform::new(750, 100, 0, 32, 32, 0.0, 1.0, 1.0))
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // build
        let mut text = Text::new(&font, 32.0);
        text.set_text(format!("{}", GathererType::Coal.get_build_cost()));
        let entity = world.create_entity()
            .with(BuildCost{})
            .with(Transform::new(775, 32, 0, 0, 0, 0.0, 1.0, 1.0))
            .with(text)
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        // sell
        let mut text = Text::new(&font, 32.0);
        text.set_text("10".to_string());
        let entity = world.create_entity()
            .with(SellCost{})
            .with(Transform::new(925, 32, 0, 0, 0, 0.0, 1.0, 1.0))
            .with(text)
            .with(Color([0.0, 1.0, 0.0, 1.0]))
            .build();
        scene.nodes.push(Node::new(Some(entity), None));

        self.scene = Arc::new(Mutex::new(scene));
    }
}